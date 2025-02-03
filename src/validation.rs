use crate::database::*;
use std::collections::HashSet;

pub fn ensure_valid_workflow(workflow: &WorkflowArgs) -> bool {
    // Check for empty workflow
    if workflow.WorkflowSteps.len() == 0 {
        return false;
    }

    // Check for cycles
    if !ensure_direct_acyclic_graph(&workflow.WorkflowSteps) {
        return false;
    }

    return true;
}

fn ensure_direct_acyclic_graph(graph: &Vec<AssignedWorkflowStepArgs>) -> bool {
    let mut visited = HashSet::new();
    let mut stack = HashSet::new();

    // Check for cycles
    for step in graph {
        if !visited.contains(&step.WorkflowStepID) {
            if has_cycle(step, graph, &mut visited, &mut stack) {
                return false;
            }
        }
    }

     // Check if all edges are directed
     for step in graph {
        for &next_index in &step.Next {
            if next_index >= graph.len() {
                return false; // Invalid index, not a directed edge
            }
        }
    }

    return true;
}

fn has_cycle(
    step: &AssignedWorkflowStepArgs,
    steps: &Vec<AssignedWorkflowStepArgs>,
    visited: &mut HashSet<u32>,
    stack: &mut HashSet<u32>,
) -> bool {
    visited.insert(step.WorkflowStepID);
    stack.insert(step.WorkflowStepID);

    for &next_index in &step.Next {
        if next_index >= steps.len() {
            return true; // Invalid index, indicates a cycle
        }
        let next_step = &steps[next_index];
        if !visited.contains(&next_step.WorkflowStepID) {
            if has_cycle(next_step, steps, visited, stack) {
                return true;
            }
        } else if stack.contains(&next_step.WorkflowStepID) {
            return true;
        }
    }

    stack.remove(&step.WorkflowStepID);
    false
}
