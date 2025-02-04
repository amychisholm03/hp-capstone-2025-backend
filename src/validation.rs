use std::collections::{HashSet, HashMap};
use crate::database::*;

pub fn ensure_valid_workflow(workflow: &WorkflowArgs) -> bool {
    if workflow.WorkflowSteps.is_empty() {
        return false;
    }

    if !ensure_direct_acyclic_graph(&workflow.WorkflowSteps) {
        return false;
    }

    true
}

fn ensure_direct_acyclic_graph(steps: &Vec<AssignedWorkflowStepArgs>) -> bool {
    let mut visited = HashSet::new();
    let mut stack = HashSet::new();
    let mut step_map: HashMap<u32, &AssignedWorkflowStepArgs> = HashMap::new();

    // Build a lookup map (WorkflowStepID -> Step)
    for step in steps {
        step_map.insert(step.WorkflowStepID, step);
    }

    // Check for cycles
    for step in steps {
        if !visited.contains(&step.WorkflowStepID) {
            if has_cycle(step.WorkflowStepID, &step_map, &mut visited, &mut stack) {
                return false;
            }
        }
    }

    return true;
}

fn has_cycle(
    step_id: u32,
    step_map: &HashMap<u32, &AssignedWorkflowStepArgs>,
    visited: &mut HashSet<u32>,
    stack: &mut HashSet<u32>,
) -> bool {
    if stack.contains(&step_id) {
        return true; // Cycle detected
    }
    if visited.contains(&step_id) {
        return false;
    }

    visited.insert(step_id);
    stack.insert(step_id);

    if let Some(step) = step_map.get(&step_id) {
        for &next_id in &step.Next {
            if has_cycle(next_id.try_into().unwrap(), step_map, visited, stack) {
                return true;
            }
        }
    }

    stack.remove(&step_id);
    false
}