use crate::database::*;
use std::collections::{HashMap, HashSet};

/// A struct to hold the validation rules for a workflow step.
struct ValidationRule {
    /// Can this step end the workflow?
    can_end_here: bool,
    /// The set of valid next workflow steps.
    valid_next: HashSet<usize>,
    /// The set of valid previous workflow steps.
    valid_prev: HashSet<usize>,
}

/// Assumes the database only contains the following steps:
///   0: "Download File",
///   1: "Preflight",
///   2: "Impose",
///   3: "Analyzer",
///   4: "Color Setup",
///   5: "Rasterization",
///   6: "Loader",
///   7: "Cutting",
///   8: "Laminating"
///
/// Please refer to the Workflow in the README.md for a visual representation.
fn get_validation_rules() -> HashMap<u32, ValidationRule> {
    let mut rules = HashMap::new();

    // Download File
    rules.insert(
        0,
        ValidationRule {
            can_end_here: false,
            // Next steps are preflight (1) and impose (2)
            valid_next: vec![1, 2].into_iter().collect(),
            valid_prev: HashSet::new(),
        },
    );
    // Preflight
    rules.insert(
        1,
        ValidationRule {
            can_end_here: false,
            // Next step is loader (6)
            valid_next: vec![6].into_iter().collect(),
            valid_prev: vec![0].into_iter().collect(),
        },
    );
    // Impose
    rules.insert(
        2,
        ValidationRule {
            can_end_here: false,
            valid_next: vec![3].into_iter().collect(),
            valid_prev: vec![0].into_iter().collect(),
        },
    );
    // Analyzer
    rules.insert(
        3,
        ValidationRule {
            can_end_here: false,
            valid_next: vec![4].into_iter().collect(),
            valid_prev: vec![2].into_iter().collect(),
        },
    );
    // Color Setup
    rules.insert(
        4,
        ValidationRule {
            can_end_here: false,
            valid_next: vec![5].into_iter().collect(),
            valid_prev: vec![3].into_iter().collect(),
        },
    );
    // Rasterization
    rules.insert(
        5,
        ValidationRule {
            can_end_here: false,
            valid_next: vec![6].into_iter().collect(),
            valid_prev: vec![4].into_iter().collect(),
        },
    );
    // Loader i.e. finally printing
    rules.insert(
        6,
        ValidationRule {
            can_end_here: true,
            // cutting (7), laminating (8) or nothing should follow
            valid_next: vec![7, 8].into_iter().collect(),
            // preflight (1) or rasterization (5) should precede
            valid_prev: vec![1, 5].into_iter().collect(),
        },
    );
    // Cutting, optional
    rules.insert(
        7,
        ValidationRule {
            can_end_here: true,
            // laminating (8) or nothing should follow
            valid_next: vec![8].into_iter().collect(),
            valid_prev: vec![6].into_iter().collect(),
        },
    );
    // Laminating, optional
    rules.insert(
        8,
        ValidationRule {
            can_end_here: true,
            valid_next: HashSet::new(),
            valid_prev: vec![6, 7].into_iter().collect(),
        },
    );

    rules
}

pub fn is_valid_workflow(workflow: &WorkflowArgs) -> bool {
    return !workflow.WorkflowSteps.is_empty() && follows_validation_rules(&workflow.WorkflowSteps);
}

fn follows_validation_rules(steps: &Vec<AssignedWorkflowStepArgs>) -> bool {
    let rules = get_validation_rules();

    for i in 0..steps.len() {
        let step = &steps[i];
        let step_id = step.WorkflowStepID - 1; // WorkflowStepID's are 1-based
        let rule = match rules.get(&step_id) {
            Some(rule) => rule,
            None => return false,
        };

        // A step that cannot end the workflow
        // must not end the workflow.
        if rule.can_end_here == false && step.Next.is_empty() {
            return false;
        }

        // Check that each id in step.Next exists in rule.valid_next
        for next_step_id in &step.Next {
            if !rule.valid_next.contains(&next_step_id) {
                return false;
            }
        }

        // Check that each id in step.Prev exists in rule.valid_prev
        for prev_step_id in &step.Prev {
            if !rule.valid_prev.contains(&prev_step_id) {
                return false;
            }
        }
    }

    true
}
