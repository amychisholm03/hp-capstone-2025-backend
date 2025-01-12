use std::{
	collections::HashMap,
	sync::{Mutex, OnceLock}
};
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::fmt::Debug;


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
	id: u32,
	title: String,
	date_created: u32,
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
	id: u32,
	title: String,
	workflow_steps: Vec<WFS>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowStep {
	id: u32,
	title: String,
	setup_time: u32,
	time_per_page: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationReport {
	id: u32,
	pj_id: u32,
	wf_id: u32,
	creation_time: u32,
	total_time: u32,
	step_times: HashMap<u32,u32> //Key: WorkflowStep ID; Value: Total time for that step
}


// TODO: SQLize this, using HashMaps as placeholders because it's easier
static PRINT_JOBS: OnceLock<Mutex<HashMap<u32, PrintJob>>> = OnceLock::new();
static WORKFLOWS: OnceLock<Mutex<HashMap<u32, Workflow>>> = OnceLock::new();
static WORKFLOW_STEPS: OnceLock<Mutex<HashMap<u32, WorkflowStep>>> = OnceLock::new();
static SIMULATION_REPORTS: OnceLock<Mutex<HashMap<u32, SimulationReport>>> = OnceLock::new();


pub fn database_init(){
	let print_jobs = PRINT_JOBS.get_or_init(|| Mutex::new(HashMap::new()));
	let workflows = WORKFLOWS.get_or_init(|| Mutex::new(HashMap::new()));
	let workflow_steps = WORKFLOW_STEPS.get_or_init(|| Mutex::new(HashMap::new()));
	let simulation_reports = SIMULATION_REPORTS.get_or_init(|| Mutex::new(HashMap::new()));

	//Insert some dummy data
	print_jobs.lock().unwrap().insert(0, PrintJob{
		id: 0, 
		title: "PrintJob1".to_string(), 
		date_created: 0, 
		page_count: 5, 
		rasterization_profile: "CMYK".to_string()
	});

	workflows.lock().unwrap().insert(0, Workflow{
		id: 1,
		title: "Workflow 1".to_string(),
		workflow_steps: vec![WFS{id:2, next:vec![], prev:vec![]}]
	});

	workflow_steps.lock().unwrap().insert(0, WorkflowStep{
		id: 2,
		title: "WorkflowStep 1".to_string(),
		setup_time: 7,
		time_per_page: 3
	});

	simulation_reports.lock().unwrap().insert(0, SimulationReport{
		id: 3,
		pj_id: 0,
		wf_id: 1,
		creation_time: 6,
		total_time: 25,
		step_times: HashMap::from([(2, 15)])
	});
}


pub fn get_from_coll(coll: String, id: u32) -> String {
	match coll.as_str() {
		"PrintJob" => {
			let print_jobs = PRINT_JOBS.get_or_init(|| Mutex::new(HashMap::new()));
			match print_jobs.lock().unwrap().get(&id) {
				Some(data) => return json!(data).to_string(),
				None => return "id not found".to_string()
			}
		}, "Workflow" => {
			let workflows = WORKFLOWS.get_or_init(|| Mutex::new(HashMap::new()));
			match workflows.lock().unwrap().get(&id) {
				Some(data) => return json!(data).to_string(),
				None => return "id not found".to_string()
			}
		}, "WorkflowStep" => {
			let workflow_steps = WORKFLOW_STEPS.get_or_init(|| Mutex::new(HashMap::new()));
			match workflow_steps.lock().unwrap().get(&id) {
				Some(data) => return json!(data).to_string(),
				None => return "id not found".to_string()
			}
		}, "SimulationReport" => {
			let simulation_reports = SIMULATION_REPORTS.get_or_init(|| Mutex::new(HashMap::new()));
			match simulation_reports.lock().unwrap().get(&id) {
				Some(data) => return json!(data).to_string(),
				None => return "id not found".to_string()
			}
		},
		_ => return "Collection Not Found".to_string()
	};
}