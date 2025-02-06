use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use crate::simulation::{*};
use crate::validation::{*};
use rusqlite::{params, Connection, Row, Result};

pub type DocID = u32;
const DATABASE_LOCATION: &str = "./db/database.db3";

// Wrap database in mutex so it can be used concurrently. Connection is opened lazily at first usage, then kept open.
lazy_static! {
    pub static ref DB_CONNECTION: Arc<Mutex<Connection>> = Arc::new(Mutex::new(
        Connection::open(DATABASE_LOCATION).expect("Failed to connect to database.")
    ));
}


#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintJob {
	#[serde(default)] pub id: Option<DocID>,
	#[serde(default)] pub DateCreated: Option<u32>,
	pub Title: String,
	pub PageCount: u32,
	pub RasterizationProfileID: u32
}


#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RasterizationProfile {
    pub id: DocID,
    pub title: String,
    pub profile: String,
}


#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignedWorkflowStep {
	pub id: DocID,             // primary key for this workflow step
    pub WorkflowStepID: DocID, // foreign key ID pertaining to what type of workflow step this is.
	pub Prev: Vec<usize>,      // list of indices into a vec of AssignedWorkflowSteps, denoting which steps came last.
	pub Next: Vec<usize>       // list of indicies into a vec of AssignedWorkflowSteps, denoting which steps come next.
}


#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
	#[serde(default)] id: Option<DocID>,
	Title: String,
	pub WorkflowSteps: Vec<AssignedWorkflowStep>,
    pub Parallelizable: bool,
    pub numOfRIPs: u32,
}


#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    #[serde(default)]
    pub id: Option<DocID>,
    pub Title: String,
    pub SetupTime: u32,
    pub TimePerPage: u32,
}


#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationReport {
	#[serde(default)] id: Option<DocID>,
	PrintJobID: DocID,
	WorkflowID: DocID,
	CreationTime: u32,
	TotalTimeTaken: u32,
    StepTimes: HashMap<DocID, u32>,
}


#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationReportDetailed {
	#[serde(default)] id: Option<DocID>,
	PrintJobID: DocID,
    PrintJobTitle: String,
	WorkflowID: DocID,
    WorkflowTitle: String,
    RasterizationProfile: String,
	CreationTime: u32,
	TotalTimeTaken: u32,
    StepTimes: HashMap<DocID, u32>,
    //TODO: There is a table in the database called ran_workflow_step, which associates a workflow
    //step with a simulation_report_id and time_taken value. We could potentially store a list of
    //RanWorkflowSteps in this struct, similar to how a workflow struct does.
}


#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationReportArgs {
    pub PrintJobID: DocID,
    pub WorkflowID: DocID,
}


#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignedWorkflowStepArgs {
    pub WorkflowStepID: DocID, // foreign key ID pertaining to what type of workflow step this is.
	pub Prev: Vec<usize>,      // list of indices into a vec of AssignedWorkflowSteps, denoting which steps came last.
	pub Next: Vec<usize>       // list of indicies into a vec of AssignedWorkflowSteps, denoting which steps come next.
}


#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowArgs {
	#[serde(default)] id: Option<DocID>,
	Title: String,
	pub WorkflowSteps: Vec<AssignedWorkflowStepArgs>,
    pub Parallelizable: bool,
    pub numOfRIPs: u32,
}


impl SimulationReport {
	pub fn new(print_job_id: DocID, workflow_id: DocID, creation_time: u32, total_time_taken: u32, step_times: HashMap<DocID,u32>) -> SimulationReport {
		return SimulationReport{
			id: None,
			PrintJobID: print_job_id,
			WorkflowID: workflow_id,
			CreationTime: creation_time,
			TotalTimeTaken: total_time_taken,
            StepTimes: step_times,
		}
	}
}


pub async fn enable_foreign_key_checking() -> Result<(), String> {
    let db = DB_CONNECTION.lock().unwrap();
    db.execute("PRAGMA foreign_keys = ON;", [])
    .map_err(|e| e.to_string())?;
    Ok(())
}


pub async fn query_print_jobs() -> Result<Vec<PrintJob>,String> {
    let db = DB_CONNECTION.lock().unwrap();

    let mut stmt = db
        .prepare("SELECT id, title, page_count, rasterization_profile_id FROM printjob;")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row: &Row| {
            Ok(PrintJob {
                id: row.get(0)?,        // Get ID from the first column
                Title: row.get(1)?,     // Get name from the second column
                DateCreated: Some(0),
                PageCount: row.get(2)?,
                RasterizationProfileID: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut results = Vec::new();
    for job_result in rows {
        let job = job_result.map_err(|e| e.to_string())?;
        results.push(job);
    }

    return Ok(results);
}


pub async fn query_workflows() -> Result<Vec<Workflow>,String> {
    let db = DB_CONNECTION.lock().unwrap();

    let mut stmt = db
        .prepare("SELECT id, title, parallelizable, num_of_RIPs FROM workflow;")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row: &Row| {
            Ok(Workflow {
                id: row.get(0)?,
                Title: row.get(1)?,
                WorkflowSteps: vec![],
                Parallelizable: row.get::<_, i32>(2)? != 0,
                numOfRIPs: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut results = Vec::new();
    for workflow_result in rows {
        let workflow = workflow_result.map_err(|e| e.to_string())?;
        results.push(workflow);
    }

    return Ok(results);

}


pub async fn query_workflow_steps() -> Result<Vec<WorkflowStep>,String> {
    let db = DB_CONNECTION.lock().unwrap();

    let mut stmt = db
        .prepare("SELECT id, title, setup_time, time_per_page FROM workflow_step;")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row: &Row| {
            Ok(WorkflowStep {
                id: row.get(0)?,        
                Title: row.get(1)?, 
                SetupTime: row.get(2)?,
                TimePerPage: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut results = Vec::new();
    for workflow_step_result in rows {
        let workflow_step = workflow_step_result.map_err(|e| e.to_string())?;
        results.push(workflow_step);
    }

    return Ok(results);

}


pub async fn query_simulation_reports() -> Result<Vec<SimulationReportDetailed>,String> {
    let db = DB_CONNECTION.lock().unwrap();

    let mut stmt = db.prepare
        ("
            SELECT 
                simulation_report.id,
                simulation_report.title,
                simulation_report.creation_time,
                simulation_report.total_time_taken,
                printjobID,
                workflowID,
                workflow.title,
                printjob.title,
                rasterization_profile.title
            FROM simulation_report
            LEFT JOIN workflow
                ON simulation_report.workflowID=workflow.id
            LEFT JOIN printjob
                ON simulation_report.printjobID=printjob.id
            LEFT JOIN rasterization_profile
                ON printjob.rasterization_profile_id=rasterization_profile.id;
        ")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row: &Row| {
            Ok(SimulationReportDetailed {
                id: row.get(0)?, 
                CreationTime: row.get(2)?,
                TotalTimeTaken: row.get(3)?,
                PrintJobID: row.get(4)?,
                WorkflowID: row.get(5)?,
                StepTimes: HashMap::from([(2, 15)]),
                PrintJobTitle: row.get(6)?,
                WorkflowTitle: row.get(7)?,
                RasterizationProfile: row.get(8)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut results : Vec<SimulationReportDetailed> = Vec::new();
    for report_result in rows {
        let report = report_result.map_err(|e| e.to_string())?;
        results.push(report);
    }

    return Ok(results);

}


pub async fn query_rasterization_profiles() -> Result<Vec<RasterizationProfile>, String> {
    let db = DB_CONNECTION.lock().unwrap();
    
    let mut stmt = db
        .prepare("SELECT id, title, profile FROM rasterization_profile;")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row: &Row| {
            Ok(RasterizationProfile {
                id: row.get(0)?, 
                title: row.get(1)?, 
                profile: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut results = Vec::new();
    for profile_result in rows {
        let profile = profile_result.map_err(|e| e.to_string())?;
        results.push(profile);
    }

    return Ok(results);

}

pub async fn find_print_job(id: DocID) -> Result<PrintJob,String> {
    let db = DB_CONNECTION.lock().unwrap();

    let mut stmt = db
        .prepare("SELECT id, title, creation_time, page_count, rasterization_profile_id FROM printjob WHERE id=(?);")
        .map_err(|e| e.to_string())?;

    let mut rows = stmt
        .query_map([id], |row: &Row| {
            Ok(PrintJob {
                id: row.get(0)?,
                Title: row.get(1)?,
                DateCreated: row.get(2)?,
                PageCount: row.get(3)?,
                RasterizationProfileID: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;


    let val = match rows.next() {
        Some(pj) => pj,
        None => return Err("Printjob not found.".to_string()),
    };


    return Ok(val.unwrap());

}


pub async fn find_rasterization_profile(id: DocID) -> Result<RasterizationProfile,String> {
    let db = DB_CONNECTION.lock().unwrap();

    let mut stmt = db
        .prepare("SELECT id, title, profile FROM rasterization_profile WHERE id=(?);")
        .map_err(|e| e.to_string())?;

    let mut rows = stmt
        .query_map([id], |row: &Row| {
            Ok(RasterizationProfile {
                id: row.get(0)?,
                title: row.get(1)?,
                profile: row.get(2)?,

            })
        })
        .map_err(|e| e.to_string())?;

    let val = match rows.next() {
        Some(pj) => pj,
        None => return Err("Rasterization profile not found.".to_string()),
    };

    return Ok(val.unwrap());

}


pub async fn find_workflow(id: DocID) -> Result<Workflow, String> {
    let db = DB_CONNECTION.lock().unwrap();

    // Get the workflow matching the supplied id
    let mut stmt0 = db.prepare("SELECT id, title FROM workflow WHERE id=(?);")
    .map_err(|e| e.to_string())?;
    let mut workflow_iter = stmt0.query_map([id], |row: &Row| {
        Ok(Workflow {
            id: row.get(0)?,
            Title: row.get(1)?,
            WorkflowSteps: vec![],
            Parallelizable: row.get::<_, i32>(2)? != 0,
            numOfRIPs: row.get(3)?,
        })
    })
    .map_err(|e| e.to_string())?;

    let mut workflow = match workflow_iter.next() {
        Some(Ok(w)) => w,
        _ => return Err("Workflow not found".to_string()),
    };

    // Get all of the steps that belong to this workflow
    let mut stmt1 = db.prepare("SELECT id, workflow_id, workflow_step_id FROM assigned_workflow_step WHERE workflow_id = ?")
    .map_err(|e| e.to_string())?;
    let steps_iter = stmt1.query_map([id], |row| {
        Ok(AssignedWorkflowStep {
            id: row.get(0)?,
            WorkflowStepID: row.get(2)?,
            Prev: vec![],
            Next: vec![],
        })
    })
    .map_err(|e| e.to_string())?;

    let mut id_to_indice : HashMap<DocID, usize> = HashMap::new();

    // Place all workflow steps in a vector. Keep track of which step is at which index.
    for step_result in steps_iter {
        
        let step = match step_result {
            Ok(s) => s,
            Err(e) => return Err(e.to_string()),
        };

        id_to_indice.insert(step.id, workflow.WorkflowSteps.len());
        workflow.WorkflowSteps.push(step);

    }

    // Add previous and next workflow step information to each step.
    for step in &mut workflow.WorkflowSteps {
        
        //// Add all of the steps that come next
        let mut stmt2 = db.prepare("SELECT assigned_workflow_step_id, next_step_id FROM next_workflow_step WHERE assigned_workflow_step_id=(?);")
        .map_err(|e| e.to_string())?;

        let next_steps_iter = stmt2.query_map([step.id], |row| {
            let next_step_id: u32 = row.get(1)?;
            Ok(next_step_id)
        })
        .map_err(|e| e.to_string())?;

        for next_step_result in next_steps_iter {
            let next_step = match next_step_result {
                Ok(s) => s,
                Err(e) => return Err(e.to_string()),
            };
            step.Next.push(*id_to_indice.get(&next_step).unwrap());
        }

        //// Add all of the steps that come before this step
        let mut stmt3 = db.prepare("SELECT assigned_workflow_step_id, prev_step_id FROM prev_workflow_step WHERE assigned_workflow_step_id=(?);")
        .map_err(|e| e.to_string())?;

        let prev_steps_iter = stmt3.query_map([step.id], |row| {
            let next_step_id: u32 = row.get(1)?;
            Ok(next_step_id)
        })
        .map_err(|e| e.to_string())?;

        for prev_step_result in prev_steps_iter {
            let prev_step = match prev_step_result {
                Ok(s) => s,
                Err(e) => return Err(e.to_string()),
            };
            step.Prev.push(*id_to_indice.get(&prev_step).unwrap());
        }

    }

    return Ok(workflow);

}


pub async fn find_workflow_step(id: DocID) -> Result<WorkflowStep,String> {
    let db = DB_CONNECTION.lock().unwrap();

    let mut stmt = db
        .prepare("SELECT id, title, setup_time, time_per_page FROM workflow_step WHERE id=(?);")
        .map_err(|e| e.to_string())?;

    let mut rows = stmt
        .query_map([id], |row: &Row| {
            Ok(WorkflowStep {
                id: row.get(0)?,
                Title: row.get(1)?,
                SetupTime: row.get(2)?,
                TimePerPage: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let val = match rows.next() {
        Some(pj) => pj,
        None => return Err("Workflow step not found.".to_string()),
    };

    return Ok(val.unwrap());

}


pub async fn find_simulation_report(id: DocID) -> Result<SimulationReport,String> {
    let db = DB_CONNECTION.lock().unwrap();

    let mut stmt = db
        .prepare("SELECT id, creation_time, total_time_taken, printjobID, workflowID FROM simulation_report WHERE id=(?);")
        .map_err(|e| e.to_string())?;

    let mut rows = stmt
        .query_map([id], |row: &Row| {
            Ok(SimulationReport {
                id: row.get(0)?,
                CreationTime: row.get(1)?,
                TotalTimeTaken: row.get(2)?, 
                PrintJobID: row.get(3)?,
                WorkflowID: row.get(4)?,
                StepTimes: HashMap::from([(2, 15)]),

            })
        })
        .map_err(|e| e.to_string())?;

    let val = match rows.next() {
        Some(pj) => pj,
        None => return Err("Simulation Report not found.".to_string()),
    };

    return Ok(val.unwrap());

}

pub async fn insert_print_job(data: PrintJob) -> Result<DocID,String> {
    let db = DB_CONNECTION.lock().unwrap();
    
    db.execute(
        "INSERT INTO printjob (id, title, creation_time, page_count, rasterization_profile_id) VALUES (NULL, ?1, ?2, ?3, ?4)",
        params![data.Title, data.DateCreated, data.PageCount, data.RasterizationProfileID]
    ).map_err(|e| e.to_string())?;

    let inserted_id : u32 = db.last_insert_rowid() as u32;
	return Ok(inserted_id);

}


pub async fn insert_rasterization_profile(data: RasterizationProfile) -> Result<DocID,String> {
    let db = DB_CONNECTION.lock().unwrap();
    
    db.execute(
        "INSERT INTO rasterization_profile (id, title) VALUES (?1, ?2);",
        params![data.id, data.title]
    ).map_err(|e| e.to_string())?;

    let inserted_id : u32 = db.last_insert_rowid() as u32;
	return Ok(inserted_id);

}


pub async fn insert_workflow(data: WorkflowArgs) -> Result<DocID,String> {
    let db = DB_CONNECTION.lock().unwrap();
    // Ensure that the workflow is valid
    if !ensure_valid_workflow(&data) {
        return Err("Invalid workflow".to_string());
    }

    // Insert the Workflow
    db.execute(
        "INSERT INTO workflow (id, title, parallelizable, num_of_RIPs) VALUES (NULL, ?1, 0, 0)",
        params![data.Title]
    ).map_err(|e| e.to_string())?;
    let inserted_id : DocID = db.last_insert_rowid() as DocID;
    
    // Load all workflow steps into the database.
    let mut indexcounter : usize = 0;
    let mut index_to_id : HashMap<usize, DocID> = HashMap::new();
    for step in &data.WorkflowSteps {
        db.execute(
            "INSERT INTO assigned_workflow_step (id, workflow_id, workflow_step_id) VALUES (NULL, ?1, ?2)",
            params![inserted_id, step.WorkflowStepID]
        ).map_err(|e| e.to_string())?;

        // map the primary key of each AssignedWorkflowStep to it's index in the vector.
        let inserted_id : DocID = db.last_insert_rowid() as DocID;
        index_to_id.insert(indexcounter, inserted_id); 
        indexcounter+=1;
    }

    // Now tie each step to it's previous/next workflow steps
    indexcounter = 0;
    for step in &data.WorkflowSteps {

	// TODO: make so we don't have to use queries in a loop at some point. pry fine for now but it's shitty for performance
        // ... all steps that come after this step
        for next_step in &step.Next {
            db.execute(
                "INSERT INTO next_workflow_step (assigned_workflow_step_id, next_step_id) VALUES (?1, ?2)",
                params![index_to_id.get(&indexcounter), index_to_id.get(next_step)] 
            ).map_err(|e| e.to_string())?;
        }

	// TODO: make so we don't have to use queries in a loop at some point. pry fine for now but it's shitty for performance
        // ... all steps that come before this step
        for prev_step in &step.Prev {
            db.execute(
                "INSERT INTO prev_workflow_step (assigned_workflow_step_id, prev_step_id) VALUES (?1, ?2)",
                params![index_to_id.get(&indexcounter), index_to_id.get(prev_step)] 
            ).map_err(|e| e.to_string())?;
        }

        indexcounter+=1;
    
    }

    return Ok(inserted_id);

}


pub async fn insert_simulation_report(print_job_id: u32, workflow_id: u32) -> Result<DocID,String> {
    // Run the simulation
    let new_report = match simulate(print_job_id, workflow_id).await {
		Ok(data) => data,
		Err(_) => return Err("Error".to_string())
	};

    // Store resulting simulation data in the db.
    let db = DB_CONNECTION.lock().unwrap();
    db.execute(
        "INSERT INTO simulation_report (id, title, creation_time, total_time_taken, printjobID, workflowID) VALUES (NULL, 'Default', ?1, ?2, ?3, ?4)",
        params![new_report.CreationTime, new_report.TotalTimeTaken, new_report.PrintJobID, new_report.WorkflowID]
    ).map_err(|e| return e.to_string())?;
    let inserted_id : u32 = db.last_insert_rowid() as u32;
	
    return Ok(inserted_id);

}


pub async fn remove_print_job(id: DocID) -> Result<usize, String> {
    let db = DB_CONNECTION.lock().unwrap();
    
    let mut stmt = db.prepare("DELETE FROM printjob WHERE id=(?)")
    .map_err(|e| return e.to_string())?;

    let res = stmt.execute([id])
    .map_err(|e| e.to_string())?;
    
    return Ok(res);

}


pub async fn remove_rasterization_profile(id: DocID) -> Result<usize, String> {
    let db = DB_CONNECTION.lock().unwrap();
    
    let mut stmt = db.prepare("DELETE FROM rasterization_profile WHERE id=(?)")
    .map_err(|e| return e.to_string())?;

    let res = stmt.execute([id])
    .map_err(|e| e.to_string())?;
    
    return Ok(res);

}

/// Deletes the assigned workflow steps associated with this
/// workflow id, then deletes the workflow itself.
pub async fn remove_workflow(id: DocID) -> Result<usize, String> {
    let db = DB_CONNECTION.lock().unwrap();

    // Delete all assigned workflow steps associated with the workflow
    let mut stmt_steps = db.prepare("DELETE FROM assigned_workflow_step WHERE workflow_id=(?)")
        .map_err(|e| e.to_string())?;
    stmt_steps.execute([id])
        .map_err(|e| e.to_string())?;

    // Delete the workflow
    let mut stmt = db.prepare("DELETE FROM workflow WHERE id=(?)")
        .map_err(|e| return e.to_string())?;
    let res = stmt.execute([id])
        .map_err(|e| e.to_string())?;

    return Ok(res);
}


pub async fn remove_simulation_report(id: DocID) -> Result<usize,String> {
    let db = DB_CONNECTION.lock().unwrap();
    
    let mut stmt = db.prepare("DELETE FROM simulation_report WHERE id=(?)")
    .map_err(|e| e.to_string())?;

    let res = stmt.execute([id])
    .map_err(|e| e.to_string())?;
    
    return Ok(res);

}

