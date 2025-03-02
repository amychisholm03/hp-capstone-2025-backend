// #[allow(non_snake_case)]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Workflow {
// 	#[serde(default)] id: Option<DocID>,
// 	Title: String,
// 	pub WorkflowSteps: Vec<AssignedWorkflowStep>,
//     pub Parallelizable: bool,
//     pub numOfRIPs: u32,
// }

use crate::database::DocID;
use crate::workflow_steps::*;
use serde::Serialize;
use serde_json::json;


#[derive(Debug, Clone, Serialize)]
struct WorkflowNode {
	data: WFSVariant,
	prev: Vec<usize>,
	next: Vec<usize>
}


#[derive(Debug, Clone, Serialize)]
struct Workflow {
	id: DocID,
	title: String,
	steps: Vec<WorkflowNode>
}


// struct WorkflowArgs {
// 	title: String,
// 	steps: Vec<WFSVariant>
// }


pub fn wf_test() {
	let wf = Workflow {
		id: 5,
		title: "Test Workflow".to_string(),
		steps: vec![
			WorkflowNode { 
				data: WFSVariant::DownloadFile,
				prev: vec![],
				next: vec![1]
			},
			WorkflowNode {
				data: WFSVariant::Rasterization {num_cores: 7},
				prev: vec![0],
				next: vec![]
			}
		]
	};
	dbg!(wf.clone());
	println!("{}", json!(wf));
}