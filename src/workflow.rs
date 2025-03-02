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
use serde::{Deserialize, Serialize};
use serde::de::{Deserializer, Error};
use serde_json::{json, Value};


#[derive(Debug, Clone, Serialize)]
struct WorkflowNode {
	data: WFSVariant,
	prev: Vec<usize>,
	next: Vec<usize>
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct Workflow {
	id: DocID,
	title: String,
	#[serde(deserialize_with="deserialize_steps")]
	steps: Vec<WorkflowNode>
}


// struct WorkflowArgs {
// 	title: String,
// 	steps: Vec<WFSVariant>
// }


fn deserialize_steps<'de, D>(deserializer: D) -> Result<Vec<WorkflowNode>, D::Error>
where D: Deserializer<'de> {
	let json_vector: Vec<Value> = Deserialize::deserialize(deserializer)?;
	dbg!(json_vector.clone());

	// let steps: Vec<WorkflowNode> = 
	for object in json_vector {
		let obj: WFSVariant = serde_json::from_value(object)
			.map_err(|_| Error::custom(format!("TODO")))?;
		dbg!(obj);
	}

	return Ok(vec![WorkflowNode{data: WFSVariant::DownloadFile, prev: vec![], next: vec![]}]);
}


pub fn wf_test() {
	// let wf = Workflow {
	// 	id: 5,
	// 	title: "Test Workflow".to_string(),
	// 	steps: vec![
	// 		WorkflowNode { 
	// 			data: WFSVariant::DownloadFile,
	// 			prev: vec![],
	// 			next: vec![1]
	// 		},
	// 		WorkflowNode {
	// 			data: WFSVariant::Rasterization {num_cores: 7},
	// 			prev: vec![0],
	// 			next: vec![]
	// 		}
	// 	]
	// };
	// dbg!(wf.clone());
	// println!("{}", json!(wf));

	let data = "{\"id\": 5, \"title\": \"Test Workflow 2\", \"steps\": [{\"id\": 0}, {\"id\": 1}, {\"id\": 5, \"num_cores\": 3}]}";
	dbg!(serde_json::from_str::<Workflow>(data));
}