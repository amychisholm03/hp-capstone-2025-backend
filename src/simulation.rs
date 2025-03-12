use crate::database::*;
use crate::workflow::*;
use crate::workflow_steps::*;
use std::{
	  sync::RwLock,
	  collections::HashMap,
    time::{SystemTime, UNIX_EPOCH}
};
use futures::future::join_all;
use std::{
    collections::HashMap,
    sync::RwLock,
    time::{SystemTime, UNIX_EPOCH},
};

struct SearchData {
    visited: Vec<bool>,
    step_times_cumulative: Vec<u32>,
    step_times_by_id: HashMap<DocID, u32>,
    cumulative_time: u32,
}
struct Search(RwLock<SearchData>);

pub async fn simulate(print_job_id: DocID, workflow_id: DocID) -> Result<SimulationReport, String> {
    // Get PrintJob and Workflow
    let print_job = match find_print_job(print_job_id).await {
        Ok(pjid) => pjid,
        Err(_) => return Err("PrintJob not found".to_string()),
    };
    let workflow: Workflow = match find_workflow(workflow_id).await {
        Ok(wfid) => wfid,
        Err(_) => return Err("Workflow not found".to_string()),
    };

    // Graph Search
    let search = Search::new(&workflow);
    traverse_graph(&print_job, &workflow, &search, &workflow.Steps.clone(), 0).await;

    // Pass results to SimulationReport constructor
    return Ok(SimulationReport::new(
        print_job_id,
        workflow_id,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Issue discerning current time.")
            .as_secs() as u32,
        search.get_cumulative_time(),
        search.get_step_times_by_id(),
    ));
}

/// Assumes graph is acyclic and connected
/// TODO: I expect we'll probably store the time/cost/other details from each step into the
/// database here. There is a table in the database called ran_workflow_step that associates an
/// AssignedWorkflowStep with a simulation_report_id & time_taken value
async fn traverse_graph(
    print_job: &PrintJob,
    workflow: &Workflow,
    search: &Search,
    steps: &Vec<WorkflowNode>,
    step: usize,
) {
    if !(search.visit(step)) {
        return;
    }

    // Recursively visit all previous nodes first
    traverse_list(&steps[step].prev, print_job, workflow, search, steps).await;

    // Simulate the current step
    let result = simulate_step(print_job, &steps[step]).await;

    // Update times
    search.update_step_time_by_id(&steps[step].data.id(), result);
    match Iterator::max(
        steps[step]
            .prev
            .iter()
            .map(|&i| search.get_step_time_cumulative(i)),
    ) {
        Some(data) => search.update_step_time_cumulative(step, result + data),
        None => search.update_step_time_cumulative(step, result),
    };

    // Recursively visit next nodes
    traverse_list(&steps[step].next, print_job, workflow, search, steps).await;
}

async fn traverse_list(
    steps: &Vec<usize>,
    print_job: &PrintJob,
    workflow: &Workflow,
    search: &Search,
    all_steps: &Vec<WorkflowNode>,
) {
    join_all(
        steps
            .iter()
            .map(|&i| traverse_graph(print_job, workflow, search, all_steps, i))
            .collect::<Vec<_>>(),
    )
    .await;
}

async fn simulate_step(print_job: &PrintJob, wfs: &WorkflowNode) -> u32 {
    let workflow_step = get_workflow_step_by_id(wfs.data.id(), None)
        .await
        .expect("Workflow has invalid step");
    return match workflow_step {
        WFSVariant::Rasterization { num_cores } => {
            return ((print_job.PageCount as f32) / (num_cores as f32)).ceil() as u32
                * workflow_step.time_per_page()
                + workflow_step.setup_time();
        }
        _ => print_job.PageCount * workflow_step.time_per_page() + workflow_step.setup_time(),
    };
}

impl Search {
    fn new(workflow: &Workflow) -> Search {
		let length = workflow.Steps.len();	
        return Search(RwLock::new(SearchData {
            visited: vec![false; length],
            step_times_cumulative: vec![0; length],
            step_times_by_id: HashMap::new(),
            cumulative_time: 0,
        }));
    }

    // Returns false to indicate that an index can't be visited because
    // it already has been, otherwise sets visited[index] to true
    // and returns true to indicate that it's OK to evaluate that step
    fn visit(&self, index: usize) -> bool {
        if self.0.read().unwrap().visited[index] == true {
            return false;
        }
        let mut visited = self.0.write().unwrap();
        visited.visited[index] = true;
        return true;
    }

    // Sets the cumulative time to reach the end of a step and keeps
    // track of the cumulative time overall
    fn update_step_time_cumulative(&self, step: usize, time: u32) {
        let mut write_lock = self.0.write().unwrap();
        write_lock.step_times_cumulative[step] = time;
        if time > write_lock.cumulative_time {
            write_lock.cumulative_time = time;
        }
    }

    fn get_step_time_cumulative(&self, step: usize) -> u32 {
        return self.0.read().unwrap().step_times_cumulative[step];
    }

    fn update_step_time_by_id(&self, id: &DocID, time: u32) {
        self.0
            .write()
            .unwrap()
            .step_times_by_id
            .entry(*id)
            .and_modify(|val| *val += time)
            .or_insert(time);
    }

    fn get_step_times_by_id(&self) -> HashMap<DocID, u32> {
        return self.0.read().unwrap().step_times_by_id.clone();
    }

    fn get_cumulative_time(&self) -> u32 {
        return self.0.read().unwrap().cumulative_time;
    }
}
