use crate::database::DocID;
use crate::workflow_steps::*;
use serde::de::{Deserializer, Error};
use serde::{Deserialize, Serialize};
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

/// A node in the workflow graph
#[derive(Debug, Clone, Serialize)]
pub struct WorkflowNode {
    pub data: WFSVariant,
    /// List of indices corresponding to previous nodes
    pub prev: Vec<usize>,
    /// List of indices corresponding to subsequent nodes
    pub next: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    #[serde(default)]
    pub id: Option<DocID>,
    pub title: String,
    #[serde(deserialize_with = "deserialize_steps")]
    pub steps: Vec<WorkflowNode>,
}

fn deserialize_steps<'de, D>(deserializer: D) -> Result<Vec<WorkflowNode>, D::Error>
where
    D: Deserializer<'de>,
{
    let json_vector: Vec<Value> = Deserialize::deserialize(deserializer)?;
    let mut steps = Vec::<WorkflowNode>::new();
    for object in json_vector {
        steps.push(WorkflowNode {
            data: serde_json::from_value(object).map_err(|_| Error::custom(format!("TODO")))?,
            prev: vec![],
            next: vec![],
        });
    }

    return Ok(fill_edges(steps));
}

/// Given a list of nodes with no edges, fill in the edges to create a graph
fn fill_edges(steps: Vec<WorkflowNode>) -> Vec<WorkflowNode> {
    // For Amy
    return steps;
}
