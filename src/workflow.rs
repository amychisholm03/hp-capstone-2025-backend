use crate::database::DocID;
use crate::workflow_steps::*;
use serde::{Deserialize, Serialize};
use serde::de::{Deserializer, Error};
use serde_json::{json, Value};


/**
 * Expected JSON input (from a POST):
 * 	{
 * 		"title": "Workflow Title",	// The title of the workflow
 * 		"steps": [					// A list of steps, to be made into a graph
 * 			{"id": 0},
 * 			{"id": 1},
 * 			{"id": 5, "num_cores": 7},
 * 		]
 * 	}
 * 
 * JSON output (sent in response to a GET):
 * 	{
 * 		"id": 0,
 * 		"title": "Workflow Title",
 * 		"steps": [
 * 			{
 * 				"data": {"id": 0, "title": "Download File", "setup_time": 0, "time_per_page": 1},
 * 				"prev": [],
 * 				"next": [1]
 * 			},{
 * 				"data": {"id": 1, "title": "Preflight", "setup_time": 10, "time_per_page": 20},
 * 				"prev": [0],
 * 				"next": [2]
 * 			},{
 * 				"data": {"id": 5, "title": "Rasterization", "setup_time": 50, "time_per_page": 15, "num_cores": 7},
 * 				"prev": [1],
 * 				"next": []
 * 			},
 * 		]
 * 	}
 **/


#[derive(Debug, Clone, Serialize)]
struct WorkflowNode {
	data: WFSVariant,
	prev: Vec<usize>, // List of indices corresponding to previous nodes
	next: Vec<usize>  // List of indices corresponding to subsequent nodes
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct Workflow {
	#[serde(default)] 
	id: Option<DocID>,
	title: String,
	#[serde(deserialize_with="deserialize_steps")]
	steps: Vec<WorkflowNode>
}


fn deserialize_steps<'de, D>(deserializer: D) -> Result<Vec<WorkflowNode>, D::Error>
where D: Deserializer<'de> {
	let json_vector: Vec<Value> = Deserialize::deserialize(deserializer)?;
	let mut steps = Vec::<WorkflowNode>::new();
	for object in json_vector {
		steps.push(WorkflowNode {
			data: serde_json::from_value(object)
				.map_err(|_| Error::custom(format!("TODO")))?,
			prev: vec![],
			next: vec![]
		});
	}
	
	return Ok(fill_edges(steps));
}


// Given a list of nodes with no edges, fill in the edges to create a graph
fn fill_edges(steps: Vec<WorkflowNode>) -> Vec<WorkflowNode> {
	// For Amy
	return steps;
}


// Testing serialization and deserialization
pub fn wf_test() {
	let wf = Workflow {
		id: Some(5),
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
	println!("Serialized:\n{}\n", json!(wf));


	let data = "{
		\"title\": \"Test Workflow 2\", 
		\"steps\": [
			{\"id\": 0}, 
			{\"id\": 1}, 
			{\"id\": 5, \"num_cores\": 3}
		]
	}";
	println!("Deserialized:");
	dbg!(serde_json::from_str::<Workflow>(data));
}