use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use crate::simulation::{*};
use rusqlite::{params, Connection, Row, Result};

const DATABASE_LOCATION: &str = "./db/database.db3";
pub type DocID = u32;

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintJob {
	#[serde(default)] id: Option<DocID>,
	#[serde(default)] DateCreated: Option<u32>,
	Title: String,
	PageCount: u32,
	RasterizationProfileID: u32
}


#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RasterizationProfile {
    pub id: DocID,
    pub title: String,
}


#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignedWorkflowStep {
	pub id: DocID,             // id by which to track this workflow step in the graph
    pub WorkflowStepID: DocID, // which type of workflow step this is
	pub Prev: Vec<DocID>,
	pub Next: Vec<DocID>
}


#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
	#[serde(default)] id: Option<DocID>,
	Title: String,
	pub WorkflowSteps: Vec<AssignedWorkflowStep>
}


#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
	#[serde(default)] id: Option<DocID>,
	Title: String,
	pub SetupTime: u32,
	pub TimePerPage: u32
}


#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationReport {
	#[serde(default)] id: Option<DocID>,
	PrintJobID: DocID,
	WorkflowID: DocID,
	CreationTime: u32,
	TotalTimeTaken: u32,
}


#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationReportArgs {
	pub PrintJobID: DocID,
	pub WorkflowID: DocID,
}


impl SimulationReport {
	pub fn new(print_job_id: DocID, workflow_id: DocID, creation_time: u32, total_time_taken: u32, step_times: HashMap<DocID,u32>) -> SimulationReport {
		return SimulationReport{
			id: None,
			PrintJobID: print_job_id,
			WorkflowID: workflow_id,
			CreationTime: creation_time,
			TotalTimeTaken: total_time_taken,
		}
	}
}


pub async fn query_print_jobs() -> Result<Vec<PrintJob>,String> {
    let db = Connection::open(DATABASE_LOCATION).unwrap();

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
    let db = Connection::open(DATABASE_LOCATION).map_err(|e| e.to_string())?;

    let mut stmt = db
        .prepare("SELECT id, title FROM workflow;")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row: &Row| {
            Ok(Workflow {
                id: row.get(0)?,
                Title: row.get(1)?,
                WorkflowSteps: vec![],
            })
        })
        .map_err(|e| e.to_string())?;

    let mut results = Vec::new();
    for job_result in rows {
        results.push(job_result.unwrap());
    }

    return Ok(results);

}


pub async fn query_workflow_steps() -> Result<Vec<WorkflowStep>,String> {
    let db = Connection::open(DATABASE_LOCATION).map_err(|e| e.to_string())?;

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
    for workflow_step in rows {
        results.push(workflow_step.unwrap());
    }

    return Ok(results);

}


pub async fn query_simulation_reports() -> Result<Vec<SimulationReport>,String> {
    let db = Connection::open(DATABASE_LOCATION).map_err(|e| e.to_string())?;

    let mut stmt = db
        .prepare("SELECT id, title, creation_time, total_time_taken, printjobID, workflowID FROM simulation_report;")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row: &Row| {
            Ok(SimulationReport {
                id: row.get(0)?, 
                CreationTime: row.get(2)?,
                TotalTimeTaken: row.get(3)?,
                PrintJobID: row.get(4)?,
                WorkflowID: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut results = Vec::new();
    for report in rows {
        results.push(report.unwrap());
    }

    return Ok(results);
}


pub async fn query_rasterization_profiles() -> Result<Vec<RasterizationProfile>, String> {
    let db = Connection::open(DATABASE_LOCATION).map_err(|e| e.to_string())?;
    
    let mut stmt = db
        .prepare("SELECT id, title FROM rasterization_profile;")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row: &Row| {
            Ok(RasterizationProfile {
                id: row.get(0)?, 
                title: row.get(1)?, 
            })
        })
        .map_err(|e| e.to_string())?;

    let mut results = Vec::new();
    for profile in rows {
        results.push(profile.unwrap());
    }

    return Ok(results);

}


pub async fn find_print_job(id: DocID) -> Result<PrintJob,String> {
    let db = Connection::open(DATABASE_LOCATION).map_err(|e| e.to_string())?;

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
    let db = Connection::open(DATABASE_LOCATION).map_err(|e| e.to_string())?;

    let mut stmt = db
        .prepare("SELECT id, title FROM rasterization_profile WHERE id=(?);")
        .map_err(|e| e.to_string())?;

    let mut rows = stmt
        .query_map([id], |row: &Row| {
            Ok(RasterizationProfile {
                id: row.get(0)?,
                title: row.get(1)?,
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
    let db = Connection::open(DATABASE_LOCATION).map_err(|e| e.to_string())?;

    // Get the workflow matching the supplied id
    let mut stmt0 = db.prepare("SELECT id, title FROM workflow WHERE id=(?);")
    .map_err(|e| e.to_string())?;
    let mut workflow_iter = stmt0.query_map([id], |row: &Row| {
        Ok(Workflow {
            id: row.get(0)?,
            Title: row.get(1)?,
            WorkflowSteps: vec![],
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


    // Gather all of the prev/next steps for each step in this workflow
    for step_result in steps_iter {
        let mut step = match step_result {
            Ok(s) => s,
            Err(e) => return Err(e.to_string()),
        };


        // grab all of the workflow steps that come next
        let mut stmt2 = db.prepare("SELECT assigned_workflow_step_id, next_step_id FROM next_workflow_step WHERE assigned_workflow_step_id=(?);")
        .map_err(|e| e.to_string())?;

        let next_step_iter = stmt2.query_map([step.id], |row| {
            let next_step_id: u32 = row.get(1)?;
            Ok(next_step_id)
        });

        let next_step_vec: Vec<u32> = next_step_iter.unwrap().collect::<Result<Vec<u32>, _>>().unwrap();
    

        // grab all of the workflow steps that came previously
        let mut stmt2 = db.prepare("SELECT assigned_workflow_step_id, prev_step_id FROM prev_workflow_step WHERE assigned_workflow_step_id=(?);")
        .map_err(|e| e.to_string())?;

        let next_step_iter = stmt2.query_map([step.id], |row| {
            let prev_step_id: u32 = row.get(1)?;
            Ok(prev_step_id)
        });

        let prev_step_vec: Vec<u32> = next_step_iter.unwrap().collect::<Result<Vec<u32>, _>>().unwrap();

        step.Prev = prev_step_vec;
        step.Next = next_step_vec;
        workflow.WorkflowSteps.push(step);

    }

    return Ok(workflow);

}


pub async fn find_workflow_step(id: DocID) -> Result<WorkflowStep,String> {
    let db = Connection::open(DATABASE_LOCATION).map_err(|e| e.to_string())?;

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
    let db = Connection::open(DATABASE_LOCATION).map_err(|e| e.to_string())?;

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
    let db = Connection::open(DATABASE_LOCATION).map_err(|e| e.to_string())?;
    
    db.execute(
        "INSERT INTO printjob (id, title, creation_time, page_count, rasterization_profile_id) VALUES (NULL, ?1, ?2, ?3, ?4)",
        params![data.Title, data.DateCreated, data.PageCount, data.RasterizationProfileID]
    ).map_err(|e| e.to_string())?;

    let inserted_id : u32 = db.last_insert_rowid() as u32;
	return Ok(inserted_id);

}


pub async fn insert_rasterization_profile(data: RasterizationProfile) -> Result<DocID,String> {
    let db = Connection::open(DATABASE_LOCATION).map_err(|e| e.to_string())?;
    
    db.execute(
        "INSERT INTO rasterization_profile (id, title) VALUES (?1, ?2);",
        params![data.id, data.title]
    ).map_err(|e| e.to_string())?;

    let inserted_id : u32 = db.last_insert_rowid() as u32;
	return Ok(inserted_id);

}


pub async fn insert_workflow(data: Workflow) -> Result<DocID,String> {
    let db = Connection::open(DATABASE_LOCATION).map_err(|e| e.to_string())?;

    // Insert the Workflow
    db.execute(
        "INSERT INTO workflow (id, title) VALUES (NULL, ?1)",
        params![data.Title]
    ).map_err(|e| e.to_string())?;
    let inserted_id : DocID = db.last_insert_rowid() as DocID;

    // Load all workflow steps into the database
    for step in &data.WorkflowSteps {
        db.execute(
            "INSERT INTO assigned_workflow_step (id, workflow_id, workflow_step_id) VALUES (?1, ?2, ?3)",
            params![step.id, inserted_id, step.WorkflowStepID]
        ).map_err(|e| e.to_string())?;
    }

    // Tie each step to it's previous/next workflow steps
    for step in &data.WorkflowSteps {

        // ... all steps that come after this step
        for next_step in &step.Next {
            db.execute(
                "INSERT INTO next_workflow_step (assigned_workflow_step_id, next_step_id) VALUES (?1, ?2)",
                params![step.id, next_step] 
            ).map_err(|e| e.to_string())?;
        }

        // ... all steps that come before this step
        for prev_step in &step.Prev {
            db.execute(
                "INSERT INTO prev_workflow_step (assigned_workflow_step_id, prev_step_id) VALUES (?1, ?2)",
                params![step.id, prev_step] 
            ).map_err(|e| e.to_string())?;
        }

    }

    return Ok(inserted_id);

}


pub async fn insert_simulation_report(PrintJobID: u32, WorkflowID: u32) -> Result<DocID,String> {

    // Run the simulation
    let new_report = match simulate(PrintJobID, WorkflowID).await {
		Ok(data) => data,
		Err(_) => return Err("Error".to_string())
	};

    // Store resulting simulation data in the db.
    let db = Connection::open(DATABASE_LOCATION).map_err(|e| e.to_string())?;
    db.execute(
        "INSERT INTO simulation_report (id, title, creation_time, total_time_taken, printjobID, workflowID) VALUES (NULL, 'Default', ?2, ?3, ?4, ?5)",
        params![new_report.CreationTime, new_report.TotalTimeTaken, new_report.PrintJobID, new_report.WorkflowID]
    ).map_err(|e| return e.to_string())?;
    let inserted_id : u32 = db.last_insert_rowid() as u32;
	
    return Ok(inserted_id);

}


//TODO: Removing a print job should fail if any simulation reports refer to it
pub async fn remove_print_job(id: DocID) -> Result<usize, String> {
    let db = Connection::open(DATABASE_LOCATION)
    .map_err(|e| return e.to_string())?;
    
    let mut stmt = db.prepare("DELETE FROM print_job WHERE id=(?)")
    .map_err(|e| return e.to_string())?;

    let res = stmt.execute([id])
    .map_err(|e| e.to_string())?;
    
    return Ok(res);

}


//TODO: Removing a simulation report should fail if any print job refers to it
pub async fn remove_rasterization_profile(id: DocID) -> Result<usize, String> {
    let db = Connection::open(DATABASE_LOCATION)
    .map_err(|e| return e.to_string())?;
    
    let mut stmt = db.prepare("DELETE FROM rasterization_profile WHERE id=(?)")
    .map_err(|e| return e.to_string())?;

    let res = stmt.execute([id])
    .map_err(|e| e.to_string())?;
    
    return Ok(res);

}


//TODO: Removing a workflow should fail if any simulation reports refer to it
pub async fn remove_workflow(id: DocID) -> Result<usize,String> {
    let db = Connection::open(DATABASE_LOCATION)
    .map_err(|e| return e.to_string())?;
    
    let mut stmt = db.prepare("DELETE FROM workflow WHERE id=(?)")
    .map_err(|e| return e.to_string())?;

    let res = stmt.execute([id])
    .map_err(|e| e.to_string())?;
    
    return Ok(res);

}


pub async fn remove_simulation_report(id: DocID) -> Result<usize,String> {
    let db = Connection::open(DATABASE_LOCATION)
    .map_err(|e| e.to_string())?;
    
    let mut stmt = db.prepare("DELETE FROM simulation_report WHERE id=(?)")
    .map_err(|e| e.to_string())?;

    let res = stmt.execute([id])
    .map_err(|e| e.to_string())?;
    
    return Ok(res);

}

