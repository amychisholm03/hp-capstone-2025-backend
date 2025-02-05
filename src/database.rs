use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

use crate::simulation::*;

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

pub type DocID = u32;

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintJob {
    #[serde(default)]
    id: Option<DocID>,
    #[serde(default)]
    DateCreated: Option<u32>,
    Title: String,
    pub PageCount: u32,
    RasterizationProfile: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
//TODO: Different name?
pub struct WFS {
    pub id: DocID,
    pub Prev: Vec<usize>,
    pub Next: Vec<usize>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    #[serde(default)]
    id: Option<DocID>,
    Title: String,
    pub WorkflowSteps: Vec<WFS>,
    pub Parallelizable: bool,
    pub RIPs: u32,
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
    #[serde(default)]
    id: Option<DocID>,
    PrintJobID: DocID,
    WorkflowID: DocID,
    CreationTime: u32,
    TotalTimeTaken: u32,
    StepTimes: HashMap<DocID, u32>, //Key: WorkflowStep ID; Value: Total time for that step
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationReportArgs {
    pub PrintJobID: DocID,
    pub WorkflowID: DocID,
}

impl SimulationReport {
    pub fn new(
        print_job_id: DocID,
        workflow_id: DocID,
        creation_time: u32,
        total_time_taken: u32,
        step_times: HashMap<DocID, u32>,
    ) -> SimulationReport {
        return SimulationReport {
            id: None,
            PrintJobID: print_job_id,
            WorkflowID: workflow_id,
            CreationTime: creation_time,
            TotalTimeTaken: total_time_taken,
            StepTimes: step_times,
        };
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
    return *out - 1;
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
		DateCreated: Some(0), 
		Title: "PrintJob1".to_string(), 
		PageCount: 5, 
		RasterizationProfile: "CMYK".to_string()
	});

	next_id();
	let id = next_id();
	workflow_steps.lock().unwrap().insert(id, WorkflowStep{
		id: Some(id),
		Title: "Preflight".to_string(),
		SetupTime: 10,
		TimePerPage: 7
	});

	let id = next_id();
	workflow_steps.lock().unwrap().insert(id, WorkflowStep{
		id: Some(id),
		Title: "Metrics".to_string(),
		SetupTime: 2,
		TimePerPage: 1
	});

	let id = next_id();
	workflow_steps.lock().unwrap().insert(id, WorkflowStep{
		id: Some(id),
		Title: "Rasterization".to_string(),
		SetupTime: 50,
		TimePerPage: 16
	});

	let id = next_id();
	workflow_steps.lock().unwrap().insert(id, WorkflowStep{
		id: Some(id),
		Title: "Printing".to_string(),
		SetupTime: 10,
		TimePerPage: 7
	});

	let id = next_id();
	workflow_steps.lock().unwrap().insert(id, WorkflowStep{
		id: Some(id),
		Title: "Cutting".to_string(),
		SetupTime: 10,
		TimePerPage: 7
	});

	let id = next_id();
	workflow_steps.lock().unwrap().insert(id, WorkflowStep{
		id: Some(id),
		Title: "Laminating".to_string(),
		SetupTime: 10,
		TimePerPage: 7
	});

	let id = next_id();
	workflows.lock().unwrap().insert(id, Workflow{
		id: Some(id),
		Title: "Workflow 1".to_string(),
		WorkflowSteps: vec![
			WFS{id:2, Next:vec![2,3], Prev:vec![]},
			WFS{id:3, Next:vec![2,3], Prev:vec![]},
			WFS{id:4, Next:vec![4], Prev:vec![0,1]},
			WFS{id:4, Next:vec![4], Prev:vec![0,1]},
			WFS{id:5, Next:vec![5], Prev:vec![2,3]},
			WFS{id:6, Next:vec![6], Prev:vec![4]},
			WFS{id:7, Next:vec![], Prev:vec![5]},
		], false, 2
	});

	let id = next_id();
	simulation_reports.lock().unwrap().insert(id, SimulationReport{
		id: Some(id),
		PrintJobID: 0,
		WorkflowID: 1,
		CreationTime: 6,
		TotalTimeTaken: 25,
		StepTimes: HashMap::from([(2, 15)])
	});
}


// TODO: Update to allow for querying
pub async fn query_print_jobs() -> Result<Vec<PrintJob>, String> {
    let print_jobs = PRINT_JOBS.get_or_init(|| Mutex::new(HashMap::new()));
    return Ok(print_jobs.lock().unwrap().values().cloned().collect());
}

// TODO: Update to allow for querying
pub async fn query_workflows() -> Result<Vec<Workflow>, String> {
    let workflows = WORKFLOWS.get_or_init(|| Mutex::new(HashMap::new()));
    return Ok(workflows.lock().unwrap().values().cloned().collect());
}

// TODO: Update to allow for querying
pub async fn query_workflow_steps() -> Result<Vec<WorkflowStep>, String> {
    let workflow_steps = WORKFLOW_STEPS.get_or_init(|| Mutex::new(HashMap::new()));
    return Ok(workflow_steps.lock().unwrap().values().cloned().collect());
}

// TODO: Update to allow for querying
pub async fn query_simulation_reports() -> Result<Vec<SimulationReport>, String> {
    let simulation_reports = SIMULATION_REPORTS.get_or_init(|| Mutex::new(HashMap::new()));
    return Ok(simulation_reports
        .lock()
        .unwrap()
        .values()
        .cloned()
        .collect());
}

pub async fn find_print_job(id: DocID) -> Result<PrintJob, String> {
    let print_jobs = PRINT_JOBS.get_or_init(|| Mutex::new(HashMap::new()));
    return match print_jobs.lock().unwrap().get(&id) {
        Some(data) => Ok(data.clone()),
        None => Err("Error".to_string()),
    };
}

pub async fn find_workflow(id: DocID) -> Result<Workflow, String> {
    let workflows = WORKFLOWS.get_or_init(|| Mutex::new(HashMap::new()));
    return match workflows.lock().unwrap().get(&id) {
        Some(data) => Ok(data.clone()),
        None => Err("Error".to_string()),
    };
}

pub async fn find_workflow_step(id: DocID) -> Result<WorkflowStep, String> {
    let workflow_steps = WORKFLOW_STEPS.get_or_init(|| Mutex::new(HashMap::new()));
    return match workflow_steps.lock().unwrap().get(&id) {
        Some(data) => Ok(data.clone()),
        None => Err("Error".to_string()),
    };
}

pub async fn find_simulation_report(id: DocID) -> Result<SimulationReport, String> {
    let simulation_reports = SIMULATION_REPORTS.get_or_init(|| Mutex::new(HashMap::new()));
    return match simulation_reports.lock().unwrap().get(&id) {
        Some(data) => Ok(data.clone()),
        None => Err("Error".to_string()),
    };
}

pub async fn insert_print_job(mut data: PrintJob) -> Result<DocID, String> {
    if data.id != None || data.DateCreated != None {
        return Err("Error".to_string());
    }
    let print_jobs = PRINT_JOBS.get_or_init(|| Mutex::new(HashMap::new()));
    let id = next_id();
    data.id = Some(id);
    data.DateCreated = Some(0);
    print_jobs.lock().unwrap().insert(id, data);
    return Ok(id);
}

pub async fn insert_workflow(mut data: Workflow) -> Result<DocID, String> {
    // TODO: it would be a good idea to check that all the prev and next indices are valid
    if data.id != None {
        return Err("Error".to_string());
    }
    let workflows = WORKFLOWS.get_or_init(|| Mutex::new(HashMap::new()));
    let id = next_id();
    data.id = Some(id);
    workflows.lock().unwrap().insert(id, data);
    return Ok(id);
}

pub async fn insert_simulation_report(data: SimulationReportArgs) -> Result<DocID, String> {
    let simulation_reports = SIMULATION_REPORTS.get_or_init(|| Mutex::new(HashMap::new()));
    let mut new_report = match simulate(data).await {
        Ok(data) => data,
        Err(_) => return Err("Error".to_string()),
    };
    let id = next_id();
    new_report.id = Some(id);
    simulation_reports.lock().unwrap().insert(id, new_report);
    return Ok(id);
}

//TODO: Removing a print job should fail if any simulation reports refer to it
pub async fn remove_print_job(id: DocID) -> Result<PrintJob, String> {
    let print_jobs = PRINT_JOBS.get_or_init(|| Mutex::new(HashMap::new()));
    return match print_jobs.lock().unwrap().remove(&id) {
        Some(data) => Ok(data),
        None => Err("Error".to_string()),
    };
}

//TODO: Removing a workflow should fail if any simulation reports refer to it
pub async fn remove_workflow(id: DocID) -> Result<Workflow, String> {
    let workflows = WORKFLOWS.get_or_init(|| Mutex::new(HashMap::new()));
    return match workflows.lock().unwrap().remove(&id) {
        Some(data) => Ok(data),
        None => Err("Error".to_string()),
    };
}

pub async fn remove_simulation_report(id: DocID) -> Result<SimulationReport, String> {
    let simulation_reports = SIMULATION_REPORTS.get_or_init(|| Mutex::new(HashMap::new()));
    return match simulation_reports.lock().unwrap().remove(&id) {
        Some(data) => Ok(data),
        None => Err("Error".to_string()),
    };
}