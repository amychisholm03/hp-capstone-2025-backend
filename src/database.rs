use std::collections::{HashMap};


pub struct PrintJob {
	id: u32, //TODO: Type?
	title: String,
	date_created: u32, //TODO: Type?
	page_count: u32,
	rasterization_profile: String
}

struct WFS {
	id: u32, //TODO: Type?
	prev: Vec<u32>, //TODO: Type?
	next: Vec<u32> //TODO: Type?
}

pub struct Workflow {
	id: u32, //TODO: Type?
	title: String,
	workflow_steps: Vec<WFS>
}

pub struct WorkflowStep {
	id: u32, //TODO: Type?
	title: String,
	setup_time: u32, //TODO: Type?
	time_per_page: u32 //TODO: Type?
}

pub struct SimulationReport {
	id: u32, //TODO: Type?
	pj_id: u32, //TODO: Type?
	wf_id: u32, //TODO: Type?
	creation_time: u32, //TODO: Type?
	total_time: u32, //TODO: Type?
	step_times: HashMap<u32,u32> //Key: WorkflowStep ID; Value: Total time for that step //TODO: Type?
}


