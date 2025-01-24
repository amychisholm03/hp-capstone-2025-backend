use std::{
	collections::HashMap,
	sync::{Mutex, OnceLock}
};
use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use crate::simulation::{*};
use rusqlite::{params, Connection, Row, Result};

/**
 * This file has a lot of placeholder stuff to allow for development 
 * of the API before the real database has been set up
 * 
 * The struct member types are my best approximation of what they 
 * should be, but will likely need to be changed, as they will need 
 * to be able to be converted from JSON -> Rust -> mySQL and back and 
 * some of the types may not be the best fit for the data
 * 
 * The "database" right now is just four HashMaps that the functions
 * do their operations on, which will need to be replaced with 
 * functions that interact with a real database soon
 */

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
pub struct AssignedWorkflowStep {
	pub id: DocID,           // id by which to track this workflow step in the graph
    pub WorkflowStepID: u32, // which type of workflow step this is
	pub Prev: Vec<u32>,
	pub Next: Vec<u32>
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
	//StepTimes: HashMap<DocID,u32> //Key: WorkflowStep ID; Value: Total time for that step
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


// TODO: SQLize this, using HashMaps as placeholders because it's easier
static PRINT_JOBS: OnceLock<Mutex<HashMap<DocID, PrintJob>>> = OnceLock::new();
static WORKFLOWS: OnceLock<Mutex<HashMap<DocID, Workflow>>> = OnceLock::new();
static WORKFLOW_STEPS: OnceLock<Mutex<HashMap<DocID, WorkflowStep>>> = OnceLock::new();
static SIMULATION_REPORTS: OnceLock<Mutex<HashMap<DocID, SimulationReport>>> = OnceLock::new();

static ID_COUNTER: OnceLock<Mutex<DocID>> = OnceLock::new();
pub fn next_id() -> DocID {
	let mut out = ID_COUNTER.get_or_init(|| Mutex::new(0)).lock().unwrap();
	*out += 1;
	return *out-1;
}


pub fn database_init(){
	let print_jobs = PRINT_JOBS.get_or_init(|| Mutex::new(HashMap::new()));
	let workflows = WORKFLOWS.get_or_init(|| Mutex::new(HashMap::new()));
	let workflow_steps = WORKFLOW_STEPS.get_or_init(|| Mutex::new(HashMap::new()));
	let simulation_reports = SIMULATION_REPORTS.get_or_init(|| Mutex::new(HashMap::new()));

	//Insert some dummy data
	let id = next_id();
	print_jobs.lock().unwrap().insert(id, PrintJob{
		id: Some(id), 
		Title: "PrintJob1".to_string(), 
		PageCount: 5, 
		RasterizationProfileID: 2,
        DateCreated: Some(0)
	});

	let id = next_id();
	workflows.lock().unwrap().insert(id, Workflow{
		id: Some(id),
		Title: "Workflow 1".to_string(),
		WorkflowSteps: vec![AssignedWorkflowStep{id:2, WorkflowStepID: 0, Next:vec![], Prev:vec![]}]
	});

	let id = next_id();
	workflow_steps.lock().unwrap().insert(id, WorkflowStep{
		id: Some(id),
		Title: "WorkflowStep 1".to_string(),
		SetupTime: 7,
		TimePerPage: 3
	});

	let id = next_id();
	workflow_steps.lock().unwrap().insert(id, WorkflowStep{
		id: Some(id),
		Title: "WorkflowStep 2".to_string(),
		SetupTime: 7,
		TimePerPage: 3
	});

	let id = next_id();
	workflow_steps.lock().unwrap().insert(id, WorkflowStep{
		id: Some(id),
		Title: "WorkflowStep 3".to_string(),
		SetupTime: 7,
		TimePerPage: 3
	});

	let id = next_id();
	simulation_reports.lock().unwrap().insert(id, SimulationReport{
		id: Some(id),
		PrintJobID: 0,
		WorkflowID: 1,
		CreationTime: 6,
		TotalTimeTaken: 25,
		//StepTimes: HashMap::from([(2, 15)])
	});
}


// TODO: Update to allow for querying
pub async fn query_print_jobs() -> Result<Vec<PrintJob>,String> {
    let db = Connection::open(DATABASE_LOCATION).unwrap();

    // Prepare the SELECT statement
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

    // Collect the results into a Vec and update the shared map
    let mut results = Vec::new();
    for job_result in rows {
        let job = job_result.map_err(|e| e.to_string())?;
        results.push(job);
    }

    return Ok(results);
}


// TODO: Update to allow for querying
pub async fn query_workflows() -> Result<Vec<Workflow>,String> {
    let db = Connection::open(DATABASE_LOCATION).map_err(|e| e.to_string())?;

    // Prepare the SELECT statement
    let mut stmt = db
        .prepare("SELECT id, title FROM workflow;")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row: &Row| {
            Ok(Workflow {
                id: row.get(0)?,        // Get ID from the first column
                Title: row.get(1)?,     // Get name from the second column
                WorkflowSteps: vec![],
            })
        })
        .map_err(|e| e.to_string())?;

    // Collect the results into a Vec and update the shared map
    let mut results = Vec::new();
    for job_result in rows {
        results.push(job_result.unwrap());
    }

    return Ok(results);

}


pub async fn query_workflow_steps() -> Result<Vec<WorkflowStep>,String> {
    let db = Connection::open(DATABASE_LOCATION).map_err(|e| e.to_string())?;

    // Prepare the SELECT statement
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


// TODO: Update to allow for querying
pub async fn query_simulation_reports() -> Result<Vec<SimulationReport>,String> {
    let db = Connection::open(DATABASE_LOCATION).map_err(|e| e.to_string())?;

    // Prepare the SELECT statement
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

    return Ok(rows.next().unwrap().unwrap());
}


// Todo: Coming back to this after i finish the adding workflow steps
pub async fn find_workflow(id: DocID) -> Result<Workflow,String> {
    let db = Connection::open(DATABASE_LOCATION).map_err(|e| e.to_string())?;


    // get all the steps associated with this workflow
    let mut stmt1 = db
        .prepare("SELECT id, workflow_id, workflow_step_id FROM assigned_workflow_step WHERE workflow_id=(?);")
        .map_err(|e| e.to_string())?;
    let mut steps = stmt1
        .query_map([id], |row: &Row| {
            Ok(AssignedWorkflowStep {
                id: row.get(0)?,
                WorkflowStepID: row.get(2)?,
                Next: vec![],
                Prev: vec![],
            }) 
        }).map_err(|e| e.to_string())?;

    // get the prev/next steps associated with each step
    for step in steps {
        let step_id = step.as_ref().unwrap().id;
        let mut step_unwrapped = step.unwrap();

        let mut stmt2 = db
            .prepare("SELECT assigned_workflow_step_id, next_step_id FROM next_workflow_step WHERE assigned_workflow_step_id=(?);")
            .map_err(|e| e.to_string())?;
        let prev_steps = stmt2
            .query_map([step_id], |row: &Row| row.get(1))
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<u32>, _>>() // Collect into Vec<u32>, handling errors.
            .map_err(|e| e.to_string())?;
        step_unwrapped.Prev = prev_steps;

        let mut stmt3 = db
            .prepare("SELECT assigned_workflow_step_id, prev_step_id FROM prev_workflow_step WHERE assigned_workflow_step_id=(?);")
            .map_err(|e| e.to_string())?;
        let next_steps = stmt3
            .query_map([step_id], |row: &Row| row.get(1))
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<u32>, _>>() // Collect into Vec<u32>, handling errors.
            .map_err(|e| e.to_string())?;
        step_unwrapped.Next = next_steps;
    }

    let mut stmt2 = db
        .prepare("SELECT id, title FROM workflow WHERE id=(?);")
        .map_err(|e| e.to_string())?;

    let mut rows = stmt2
        .query_map([id], |row: &Row| {
            Ok(Workflow {
                id: row.get(0)?,
                Title: row.get(1)?,
                WorkflowSteps: vec![],
            })
        })
        .map_err(|e| e.to_string())?;

    let workflow = rows.next().unwrap().unwrap(); 
    workflow.WorkflowSteps = steps;

    return Ok(rows.next().unwrap().unwrap());

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

    return Ok(rows.next().unwrap().unwrap());

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

    return Ok(rows.next().unwrap().unwrap());

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


pub async fn insert_workflow(data: Workflow) -> Result<DocID,String> {
    let db = Connection::open(DATABASE_LOCATION).map_err(|e| e.to_string())?;

    dbg!(&data);
    // Insert the Workflow
    db.execute(
        "INSERT INTO workflow (id, title) VALUES (NULL, ?1)",
        params![data.Title]
    ).map_err(|e| e.to_string())?;

    let inserted_id : u32 = db.last_insert_rowid() as u32;

    // Load all workflow steps into the database
    for step in &data.WorkflowSteps {
        db.execute(
            "INSERT INTO assigned_workflow_step (id, workflow_id, workflow_step_id) VALUES (?1, ?2, ?3)",
            params![step.id, inserted_id, step.id]
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
    ).map_err(|e| e.to_string())?;

    let inserted_id : u32 = db.last_insert_rowid() as u32;
	return Ok(inserted_id);
}


//TODO: Removing a print job should fail if any simulation reports refer to it
pub async fn remove_print_job(id: DocID) -> Result<PrintJob,String> {
	let print_jobs = PRINT_JOBS.get_or_init(|| Mutex::new(HashMap::new()));
	return match print_jobs.lock().unwrap().remove(&id) {
		Some(data) => Ok(data),
		None => Err("Error".to_string())
	}
}


//TODO: Removing a workflow should fail if any simulation reports refer to it
pub async fn remove_workflow(id: DocID) -> Result<Workflow,String> {
	let workflows = WORKFLOWS.get_or_init(|| Mutex::new(HashMap::new()));
	return match workflows.lock().unwrap().remove(&id) {
		Some(data) => Ok(data),
		None => Err("Error".to_string())
	}
}


pub async fn remove_simulation_report(id: DocID) -> Result<SimulationReport,String> {
	let simulation_reports = SIMULATION_REPORTS.get_or_init(|| Mutex::new(HashMap::new()));
	return match simulation_reports.lock().unwrap().remove(&id) {
		Some(data) => Ok(data),
		None => Err("Error".to_string())
	}
}
