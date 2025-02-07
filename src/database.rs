use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, Mutex}
};
use thiserror;
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;
use rusqlite::{params, Connection, Error, Row, Result, Params};
use crate::simulation::{*};
use crate::validation::{*};

pub type DocID = u32;
const DATABASE_LOCATION: &str = "./db/database.db3";

// Wrap database in mutex so it can be used concurrently. Connection is opened lazily at first usage, then kept open.
lazy_static! {
    pub static ref DB_CONNECTION: Arc<Mutex<Connection>> = Arc::new(Mutex::new(
        Connection::open(DATABASE_LOCATION).expect("Failed to connect to database.")
    ));
}


// This is a wrapper for passing along a rusqlite error or a custom error string
// More error types could be added to this enum if needed
#[derive(Debug, thiserror::Error)]
pub enum CustomError {
    #[error("{0}")]
    OtherError(String),
    #[error(transparent)]
    DatabaseError(#[from] Error)
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
	pub WorkflowSteps: Vec<AssignedWorkflowStep>
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
	pub WorkflowSteps: Vec<AssignedWorkflowStepArgs>
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


/**
 * Create struct from database row
 * 
 * These functions transform a row from a SQL query and creates the
 * appropriate struct from the result. These functions assume that
 * the columns in the row are in a specific order
 **/

fn print_job_from_row(row: &Row) -> Result<PrintJob> {
    return Ok(PrintJob {
        id: row.get(0)?,
        Title: row.get(1)?,
        DateCreated: row.get(2)?,
        PageCount: row.get(3)?,
        RasterizationProfileID: row.get(4)?,
    });
}

fn workflow_from_row(row: &Row) -> Result<Workflow> {
    return Ok(Workflow {
        id: row.get(0)?,
        Title: row.get(1)?,
        WorkflowSteps: vec![],
    });
}

fn workflow_step_from_row(row: &Row) -> Result<WorkflowStep> {
    return Ok(WorkflowStep {
        id: row.get(0)?,        
        Title: row.get(1)?, 
        SetupTime: row.get(2)?,
        TimePerPage: row.get(3)?,
    });
}

fn simulation_report_detailed_from_row(row: &Row) -> Result<SimulationReportDetailed> {
    return Ok(SimulationReportDetailed {
        id: row.get(0)?, 
        CreationTime: row.get(2)?,
        TotalTimeTaken: row.get(3)?,
        PrintJobID: row.get(4)?,
        WorkflowID: row.get(5)?,
        StepTimes: HashMap::from([(2, 15)]),
        PrintJobTitle: row.get(6)?,
        WorkflowTitle: row.get(7)?,
        RasterizationProfile: row.get(8)?,
    });
}

fn rasterization_profile_from_row(row: &Row) -> Result<RasterizationProfile> {
    return Ok(RasterizationProfile {
        id: row.get(0)?, 
        title: row.get(1)?, 
        profile: row.get(2)?,
    });
}

fn simulation_report_from_row(row: &Row) -> Result<SimulationReport> {
    return Ok(SimulationReport {
        id: row.get(0)?,
        CreationTime: row.get(1)?,
        TotalTimeTaken: row.get(2)?, 
        PrintJobID: row.get(3)?,
        WorkflowID: row.get(4)?,
        StepTimes: HashMap::from([(2, 15)]),
    });
}

fn assigned_workflow_step_from_row(row: &Row) -> Result<AssignedWorkflowStep> {
    return Ok(AssignedWorkflowStep {
        id: row.get(0)?,
        WorkflowStepID: row.get(2)?,
        Prev: vec![],
        Next: vec![],
    });
}


pub async fn enable_foreign_key_checking() -> Result<()> {
    let db = DB_CONNECTION.lock().unwrap();
    db.execute("PRAGMA foreign_keys = ON;", [])?;
    return Ok(())
}

// Query Functions

// TODO: Comment
fn query<T,P,F>(query: &str, params: P, f: F) -> Result<Vec<T>> 
where P: Params, F: FnMut(&Row<'_>) -> Result<T> {
    let db = DB_CONNECTION.lock().unwrap();
    let mut stmt = db.prepare(query)?;
    let rows = stmt.query_map(params, f)?;

    let mut results = Vec::new();
    for job_result in rows {
        let job = job_result?;
        results.push(job);
    }
    
    return Ok(results);
}


pub async fn query_print_jobs() -> Result<Vec<PrintJob>> {
    return query("SELECT id, title, creation_time, page_count, rasterization_profile_id FROM printjob;", 
        [], print_job_from_row);
}


pub async fn query_workflows() -> Result<Vec<Workflow>> {
    return query("SELECT id, title FROM workflow;",
        [], workflow_from_row);
}


pub async fn query_workflow_steps() -> Result<Vec<WorkflowStep>> {
    return query("SELECT id, title, setup_time, time_per_page FROM workflow_step;",
        [], workflow_step_from_row);
}


pub async fn query_simulation_reports() -> Result<Vec<SimulationReportDetailed>> {
    return query("
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
    ", [], simulation_report_detailed_from_row);
}


pub async fn query_rasterization_profiles() -> Result<Vec<RasterizationProfile>> {
    return query("SELECT id, title, profile FROM rasterization_profile;",
        [], rasterization_profile_from_row);
}

// Find functions

// TODO: Comment
fn check_id_lookup_results<T>(mut rows: Vec<T>) -> Result<T,CustomError> {
    return match rows.len() {
        0 => Err(CustomError::DatabaseError(Error::QueryReturnedNoRows)),
        1 => Ok(rows.pop().unwrap()),
        _ => Err(CustomError::OtherError("ID search returned multiple rows".to_string()))
    };
}


pub async fn find_print_job(id: DocID) -> Result<PrintJob,CustomError> {
    let rows = query("SELECT id, title, creation_time, page_count, rasterization_profile_id FROM printjob WHERE id=(?);",
        [id], print_job_from_row)?;
    return check_id_lookup_results(rows);
}


pub async fn find_rasterization_profile(id: DocID) -> Result<RasterizationProfile,CustomError> {
    let rows = query("SELECT id, title, profile FROM rasterization_profile WHERE id=(?);",
        [id], rasterization_profile_from_row)?;
    return check_id_lookup_results(rows);
}


// TODO: refactor similar to other find functions
pub async fn find_workflow(id: DocID) -> Result<Workflow> {
    let db = DB_CONNECTION.lock().unwrap();

    // Get the workflow matching the supplied id
    let mut stmt0 = db.prepare("SELECT id, title FROM workflow WHERE id=(?);")?;
    let mut workflow_iter = stmt0.query_map([id], workflow_from_row)?;

    let mut workflow = match workflow_iter.next() {
        Some(Ok(w)) => w,
        _ => return Err(Error::QueryReturnedNoRows),
    };

    // Get all of the steps that belong to this workflow
    let mut stmt1 = db.prepare("SELECT id, workflow_id, workflow_step_id FROM assigned_workflow_step WHERE workflow_id = ?")?;
    let steps_iter = stmt1.query_map([id], assigned_workflow_step_from_row)?;

    // Place all workflow steps in a vector. Keep track of which step is at which index.
    let mut id_to_indice : HashMap<DocID, usize> = HashMap::new();
    for step_result in steps_iter {
        let step = match step_result {
            Ok(s) => s,
            Err(e) => return Err(e),
        };
        id_to_indice.insert(step.id, workflow.WorkflowSteps.len());
        workflow.WorkflowSteps.push(step);
    }

    // Add previous and next workflow step information to each step.
    for step in &mut workflow.WorkflowSteps {
        //// Add all of the steps that come next
        let mut stmt2 = db.prepare("SELECT assigned_workflow_step_id, next_step_id FROM next_workflow_step WHERE assigned_workflow_step_id=(?);")?;
        let next_steps_iter = stmt2.query_map([step.id], |row| {
            let next_step_id: u32 = row.get(1)?;
            Ok(next_step_id)
        })?;

        for next_step_result in next_steps_iter {
            let next_step = match next_step_result {
                Ok(s) => s,
                Err(e) => return Err(e),
            };
            step.Next.push(*id_to_indice.get(&next_step).unwrap());
        }

        //// Add all of the steps that come before this step
        let mut stmt3 = db.prepare("SELECT assigned_workflow_step_id, prev_step_id FROM prev_workflow_step WHERE assigned_workflow_step_id=(?);")?;

        let prev_steps_iter = stmt3.query_map([step.id], |row| {
            let next_step_id: u32 = row.get(1)?;
            Ok(next_step_id)
        })?;

        for prev_step_result in prev_steps_iter {
            let prev_step = match prev_step_result {
                Ok(s) => s,
                Err(e) => return Err(e),
            };
            step.Prev.push(*id_to_indice.get(&prev_step).unwrap());
        }
    }

    return Ok(workflow);
}


pub async fn find_workflow_step(id: DocID) -> Result<WorkflowStep,CustomError> {
    let rows = query("SELECT id, title, setup_time, time_per_page FROM workflow_step WHERE id=(?);",
        [id], workflow_step_from_row)?;
    return check_id_lookup_results(rows);
}


pub async fn find_simulation_report(id: DocID) -> Result<SimulationReport,CustomError> {
    let rows = query("SELECT id, creation_time, total_time_taken, printjobID, workflowID FROM simulation_report WHERE id=(?);",
        [id], simulation_report_from_row)?;
    return check_id_lookup_results(rows);
}

// Insert functions
// TODO: Consolidate insert_print_job, insert_rasterization_profile, and maybe insert_simulation_report

pub async fn insert_print_job(data: PrintJob) -> Result<DocID> {
    let db = DB_CONNECTION.lock().unwrap();
    
    db.execute(
        "INSERT INTO printjob (id, title, creation_time, page_count, rasterization_profile_id) VALUES (NULL, ?1, ?2, ?3, ?4)",
        params![data.Title, data.DateCreated, data.PageCount, data.RasterizationProfileID]
    )?;

    let inserted_id : u32 = db.last_insert_rowid() as u32;
	return Ok(inserted_id);
}


pub async fn insert_rasterization_profile(data: RasterizationProfile) -> Result<DocID> {
    let db = DB_CONNECTION.lock().unwrap();
    
    db.execute(
        "INSERT INTO rasterization_profile (id, title) VALUES (?1, ?2);",
        params![data.id, data.title]
    )?;

    let inserted_id : u32 = db.last_insert_rowid() as u32;
	return Ok(inserted_id);
}


pub async fn insert_workflow(data: WorkflowArgs) -> Result<DocID,CustomError> {
    // Ensure that the workflow is valid
    if !ensure_valid_workflow(&data) {
        return Err(CustomError::OtherError("Invalid workflow".to_string()));
    }
    let db = DB_CONNECTION.lock().unwrap();

    // Insert the Workflow
    db.execute(
        "INSERT INTO workflow (id, title) VALUES (NULL, ?1)",
        params![data.Title]
    )?;
    let inserted_id : DocID = db.last_insert_rowid() as DocID;
    
    // Load all workflow steps into the database.
    let mut indexcounter : usize = 0;
    let mut index_to_id : HashMap<usize, DocID> = HashMap::new();
    for step in &data.WorkflowSteps {
        db.execute(
            "INSERT INTO assigned_workflow_step (id, workflow_id, workflow_step_id) VALUES (NULL, ?1, ?2)",
            params![inserted_id, step.WorkflowStepID]
        )?;

        // map the primary key of each AssignedWorkflowStep to it's index in the vector.
        let inserted_id : DocID = db.last_insert_rowid() as DocID;
        index_to_id.insert(indexcounter, inserted_id); 
        indexcounter += 1;
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
            )?;
        }

	// TODO: make so we don't have to use queries in a loop at some point. pry fine for now but it's shitty for performance
        // ... all steps that come before this step
        for prev_step in &step.Prev {
            db.execute(
                "INSERT INTO prev_workflow_step (assigned_workflow_step_id, prev_step_id) VALUES (?1, ?2)",
                params![index_to_id.get(&indexcounter), index_to_id.get(prev_step)] 
            )?;
        }
        indexcounter+=1;
    }

    return Ok(inserted_id);
}


pub async fn insert_simulation_report(print_job_id: u32, workflow_id: u32) -> Result<DocID,CustomError> {
    // Run the simulation
    let new_report = match simulate(print_job_id, workflow_id).await {
		Ok(data) => data,
		Err(e) => return Err(CustomError::OtherError(e))
	};

    // Store resulting simulation data in the db.
    let db = DB_CONNECTION.lock().unwrap();
    db.execute(
        "INSERT INTO simulation_report (id, title, creation_time, total_time_taken, printjobID, workflowID) VALUES (NULL, 'Default', ?1, ?2, ?3, ?4)",
        params![new_report.CreationTime, new_report.TotalTimeTaken, new_report.PrintJobID, new_report.WorkflowID]
    )?;
    let inserted_id : u32 = db.last_insert_rowid() as u32;
	
    return Ok(inserted_id);
}

// Remove functions
// TODO: consolidate remove_print_job, remove_rasterization_profile, remove_simulation_report

pub async fn remove_print_job(id: DocID) -> Result<usize> {
    let db = DB_CONNECTION.lock().unwrap();
    let mut stmt = db.prepare("DELETE FROM printjob WHERE id=(?)")?;
    let res = stmt.execute([id])?;
    return Ok(res);
}


pub async fn remove_rasterization_profile(id: DocID) -> Result<usize> {
    let db = DB_CONNECTION.lock().unwrap();
    let mut stmt = db.prepare("DELETE FROM rasterization_profile WHERE id=(?)")?;
    let res = stmt.execute([id])?;
    return Ok(res);
}


/// Deletes the assigned workflow steps associated with this
/// workflow id, then deletes the workflow itself.
pub async fn remove_workflow(id: DocID) -> Result<usize> {
    let db = DB_CONNECTION.lock().unwrap();

    // Delete all assigned workflow steps associated with the workflow
    let mut stmt_steps = db.prepare("DELETE FROM assigned_workflow_step WHERE workflow_id=(?)")?;
    stmt_steps.execute([id])?;

    // Delete the workflow
    let mut stmt = db.prepare("DELETE FROM workflow WHERE id=(?)")?;
    let res = stmt.execute([id])?;

    return Ok(res);
}


pub async fn remove_simulation_report(id: DocID) -> Result<usize> {
    let db = DB_CONNECTION.lock().unwrap();
    let mut stmt = db.prepare("DELETE FROM simulation_report WHERE id=(?)")?;
    let res = stmt.execute([id])?;
    return Ok(res);
}