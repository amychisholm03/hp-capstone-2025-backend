use std::{
	collections::HashMap,
	sync::{Mutex, OnceLock}
};
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::fmt::Debug;

use crate::simulation::{*};


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


#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintJob {
	#[serde(default)]
	id: Option<u32>,
	Title: String,
	#[serde(default)]
	DateCreated: Option<u32>,
	PageCount: u32,
	RasterizationProfile: String
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
//TODO: Different name?
struct WFS {
	id: u32,
	Prev: Vec<u32>,
	Next: Vec<u32>
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
	#[serde(default)]
	id: Option<u32>,
	Title: String,
	WorkflowSteps: Vec<WFS>
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
	#[serde(default)]
	id: Option<u32>,
	Title: String,
	SetupTime: u32,
	TimePerPage: u32
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationReport {
	#[serde(default)]
	id: Option<u32>,
	PrintJobID: u32,
	WorkflowID: u32,
	CreationTime: u32,
	TotalTimeTaken: u32,
	StepTimes: HashMap<u32,u32> //Key: WorkflowStep ID; Value: Total time for that step
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationReportArgs {
	pub PrintJobID: u32,
	pub WorkflowID: u32,
}


impl SimulationReport {
	pub fn new(print_job_id: u32, workflow_id: u32, creation_time: u32, total_time_taken: u32, step_times: HashMap<u32,u32>) -> SimulationReport {
		return SimulationReport{
			id: None,
			PrintJobID: print_job_id,
			WorkflowID: workflow_id,
			CreationTime: creation_time,
			TotalTimeTaken: total_time_taken,
			StepTimes: step_times
		}
	}
}


// TODO: SQLize this, using HashMaps as placeholders because it's easier
static PRINT_JOBS: OnceLock<Mutex<HashMap<u32, PrintJob>>> = OnceLock::new();
static WORKFLOWS: OnceLock<Mutex<HashMap<u32, Workflow>>> = OnceLock::new();
static WORKFLOW_STEPS: OnceLock<Mutex<HashMap<u32, WorkflowStep>>> = OnceLock::new();
static SIMULATION_REPORTS: OnceLock<Mutex<HashMap<u32, SimulationReport>>> = OnceLock::new();

static ID_COUNTER: OnceLock<Mutex<u32>> = OnceLock::new();
pub fn next_id() -> u32 {
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
		DateCreated: Some(0), 
		PageCount: 5, 
		RasterizationProfile: "CMYK".to_string()
	});

	let id = next_id();
	workflows.lock().unwrap().insert(id, Workflow{
		id: Some(id),
		Title: "Workflow 1".to_string(),
		WorkflowSteps: vec![WFS{id:2, Next:vec![], Prev:vec![]}]
	});

	let id = next_id();
	workflow_steps.lock().unwrap().insert(id, WorkflowStep{
		id: Some(id),
		Title: "WorkflowStep 1".to_string(),
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
		StepTimes: HashMap::from([(2, 15)])
	});
}


// TODO: Update to allow for querying
pub fn query_print_jobs() -> Option<String> {
	let print_jobs = PRINT_JOBS.get_or_init(|| Mutex::new(HashMap::new()));
	let vals: Vec<PrintJob> = print_jobs.lock().unwrap().values().cloned().collect();
	return Some(json!(vals).to_string());
}


// TODO: Update to allow for querying
pub fn query_workflows() -> Option<String> {
	let workflows = WORKFLOWS.get_or_init(|| Mutex::new(HashMap::new()));
	let vals: Vec<Workflow> = workflows.lock().unwrap().values().cloned().collect();
	return Some(json!(vals).to_string());
}


// TODO: Update to allow for querying
pub fn query_workflow_steps() -> Option<String> {
	let workflow_steps = WORKFLOW_STEPS.get_or_init(|| Mutex::new(HashMap::new()));
	let vals: Vec<WorkflowStep> = workflow_steps.lock().unwrap().values().cloned().collect();
	return Some(json!(vals).to_string());
}


// TODO: Update to allow for querying
pub fn query_simulation_reports() -> Option<String> {
	let simulation_reports = SIMULATION_REPORTS.get_or_init(|| Mutex::new(HashMap::new()));
	let vals: Vec<SimulationReport> = simulation_reports.lock().unwrap().values().cloned().collect();
	return Some(json!(vals).to_string());
}


pub fn find_print_job(id: u32) -> Option<String> {
	let print_jobs = PRINT_JOBS.get_or_init(|| Mutex::new(HashMap::new()));
	return match print_jobs.lock().unwrap().get(&id) {
		Some(data) => Some(json!(data).to_string()),
		None => None
	}
}


pub fn find_workflow(id: u32) -> Option<String> {
	let workflows = WORKFLOWS.get_or_init(|| Mutex::new(HashMap::new()));
	return match workflows.lock().unwrap().get(&id) {
		Some(data) => Some(json!(data).to_string()),
		None => None
	}
}


pub fn find_workflow_step(id: u32) -> Option<String> {
	let workflow_steps = WORKFLOW_STEPS.get_or_init(|| Mutex::new(HashMap::new()));
	return match workflow_steps.lock().unwrap().get(&id) {
		Some(data) => Some(json!(data).to_string()),
		None => None
	}
}


pub fn find_simulation_report(id: u32) -> Option<String> {
	let simulation_reports = SIMULATION_REPORTS.get_or_init(|| Mutex::new(HashMap::new()));
	return match simulation_reports.lock().unwrap().get(&id) {
		Some(data) => Some(json!(data).to_string()),
		None => None
	}
}


pub fn insert_print_job(mut data: PrintJob) -> Option<u32> {
	if data.id != None || data.DateCreated != None { return None }
	let print_jobs = PRINT_JOBS.get_or_init(|| Mutex::new(HashMap::new()));
	let id = next_id();
	data.id = Some(id);
	data.DateCreated = Some(0);
	print_jobs.lock().unwrap().insert(id, data);
	return Some(id);
}


pub fn insert_workflow(mut data: Workflow) -> Option<u32> {
	if data.id != None { return None }
	let workflows = WORKFLOWS.get_or_init(|| Mutex::new(HashMap::new()));
	let id = next_id();
	data.id = Some(id);
	workflows.lock().unwrap().insert(id, data);
	return Some(id);
}


pub fn insert_simulation_report(data: SimulationReportArgs) -> Option<u32> {
	let simulation_reports = SIMULATION_REPORTS.get_or_init(|| Mutex::new(HashMap::new()));
	let mut new_report = simulate(data);
	let id = next_id();
	new_report.id = Some(id);
	simulation_reports.lock().unwrap().insert(id, new_report);
	return Some(id);
}


pub fn remove_print_job(id: u32) -> Option<String> {
	let print_jobs = PRINT_JOBS.get_or_init(|| Mutex::new(HashMap::new()));
	return match print_jobs.lock().unwrap().remove(&id) {
		Some(data) => Some(json!(data).to_string()),
		None => None
	}
}


pub fn remove_workflow(id: u32) -> Option<String> {
	let workflows = WORKFLOWS.get_or_init(|| Mutex::new(HashMap::new()));
	return match workflows.lock().unwrap().remove(&id) {
		Some(data) => Some(json!(data).to_string()),
		None => None
	}
}


pub fn remove_simulation_report(id: u32) -> Option<String> {
	let simulation_reports = SIMULATION_REPORTS.get_or_init(|| Mutex::new(HashMap::new()));
	return match simulation_reports.lock().unwrap().remove(&id) {
		Some(data) => Some(json!(data).to_string()),
		None => None
	}
}