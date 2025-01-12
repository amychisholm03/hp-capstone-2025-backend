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


#[derive(Debug, Serialize, Deserialize)]
pub struct PrintJob {
	#[serde(default)]
	id: Option<u32>,
	title: String,
	#[serde(default)]
	date_created: Option<u32>,
	page_count: u32,
	rasterization_profile: String
}

#[derive(Debug, Serialize, Deserialize)]
//TODO: Different name?
struct WFS {
	id: u32,
	prev: Vec<u32>,
	next: Vec<u32>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workflow {
	#[serde(default)]
	id: Option<u32>,
	title: String,
	workflow_steps: Vec<WFS>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowStep {
	#[serde(default)]
	id: Option<u32>,
	title: String,
	setup_time: u32,
	time_per_page: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationReport {
	#[serde(default)]
	id: Option<u32>,
	pj_id: u32,
	wf_id: u32,
	creation_time: u32,
	total_time: u32,
	step_times: HashMap<u32,u32> //Key: WorkflowStep ID; Value: Total time for that step
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationReportArgs {
	pub pj_id: u32,
	pub wf_id: u32,
}


impl SimulationReport {
	pub fn new(pj_id: u32, wf_id: u32, creation_time: u32, total_time: u32, step_times: HashMap<u32,u32>) -> SimulationReport {
		return SimulationReport{
			id: None,
			pj_id: pj_id,
			wf_id: wf_id,
			creation_time: creation_time,
			total_time: total_time,
			step_times: step_times
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
		title: "PrintJob1".to_string(), 
		date_created: Some(0), 
		page_count: 5, 
		rasterization_profile: "CMYK".to_string()
	});

	let id = next_id();
	workflows.lock().unwrap().insert(id, Workflow{
		id: Some(id),
		title: "Workflow 1".to_string(),
		workflow_steps: vec![WFS{id:2, next:vec![], prev:vec![]}]
	});

	let id = next_id();
	workflow_steps.lock().unwrap().insert(id, WorkflowStep{
		id: Some(id),
		title: "WorkflowStep 1".to_string(),
		setup_time: 7,
		time_per_page: 3
	});

	let id = next_id();
	simulation_reports.lock().unwrap().insert(id, SimulationReport{
		id: Some(id),
		pj_id: 0,
		wf_id: 1,
		creation_time: 6,
		total_time: 25,
		step_times: HashMap::from([(2, 15)])
	});
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
	if data.id != None || data.date_created != None { return None }
	let print_jobs = PRINT_JOBS.get_or_init(|| Mutex::new(HashMap::new()));
	let id = next_id();
	data.id = Some(id);
	data.date_created = Some(0);
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