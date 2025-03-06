use std::string;

use crate::database::DocID;
use crate::workflow_steps::*;
use serde::de::{Deserializer, Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A node in the workflow graph
#[derive(Debug, Clone, Serialize)]
pub struct WorkflowNode {
    pub data: WFSVariant,
    /// List of indices corresponding to previous nodes
    pub prev: Vec<usize>,
    /// List of indices corresponding to subsequent nodes
    pub next: Vec<usize>,
}

/// A workflow, represented as a graph of nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    #[serde(default)]
    pub id: Option<DocID>,
    pub title: String,
    #[serde(deserialize_with = "deserialize_steps")]
    pub steps: Vec<WorkflowNode>,
}

/// Arguments for creating a new workflow, sent by the frontend
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowArgs {
	#[serde(default)] 
    pub id: Option<DocID>,
	pub Title: string::String,
	pub WorkflowSteps: Vec<AssignedWorkflowStepArgs>
}

fn deserialize_steps<'de, D>(deserializer: D) -> Result<Vec<WorkflowNode>, D::Error>
where
    D: Deserializer<'de>,
{
    let json_vector: Vec<Value> = Deserialize::deserialize(deserializer)?;
    let mut steps = Vec::<WorkflowNode>::new();
    for object in json_vector {
        steps.push(WorkflowNode {
            data: serde_json::from_value(object).map_err(|_| Error::custom(format!("Failed to serialize")))?,
            prev: vec![],
            next: vec![],
        });
    }

    // This will fill in the `prev`` and `next`` fields of the steps
    fill_edges(steps).map_err(|_| Error::custom("Failed to fill edges, likely an invalid workflow"))
}

/// Given a list of nodes with no edges, fill in the edges to create a graph
fn fill_edges(steps: Vec<WorkflowNode>) -> Result<Vec<WorkflowNode>, ()> {
    // TODO: respect no_valid_prev and no_valid_next
    let mut new_steps = steps.clone();
    for (i, step) in steps.iter().enumerate() {
        for (j, other_step) in steps.iter().enumerate() {
            if i != j {
                if step.data.valid_prev().contains(&other_step.data) {
                    new_steps[i].prev.push(j);
                }
                if step.data.valid_next().contains(&other_step.data) {
                    new_steps[i].next.push(j);
                }
            }
        }
    }
    return Ok(new_steps);
}