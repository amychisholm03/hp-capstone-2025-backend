use std::{
	sync::RwLock,
	collections::HashMap,
    time::{SystemTime, UNIX_EPOCH}
};
use futures::future::join_all;

use crate::database::{*};


struct SearchData {
	visited: Vec<bool>,
	step_times_cumulative: Vec<u32>,
	step_times_by_id: HashMap<DocID,u32>,
	cumulative_time: u32
}
struct Search(RwLock<SearchData>);


pub async fn simulate(data: SimulationReportArgs) -> Result<SimulationReport,String> {
	// Get PrintJob and Workflow
	let print_job = match find_print_job(data.PrintJobID).await{
		Ok(data) => data,
		Err(_) => return Err("PrintJob not found".to_string())
	};
	let workflow = match find_workflow(data.WorkflowID).await{
		Ok(data) => data,
		Err(_) => return Err("Workflow not found".to_string())
	};

	// Return early if the workflow contains no steps
	// TODO: A workflow with no steps should be made impossible
	if workflow.WorkflowSteps.len() == 0 { 
		return Ok(SimulationReport::new( data.PrintJobID, data.WorkflowID,
			0, 0, HashMap::new()));
	}

	// Graph Search
	let search = Search::new(&workflow);
	traverse_graph(&print_job, &search, &workflow.WorkflowSteps.clone(), 0).await;

	// Pass results to SimulationReport constructor
	return Ok(SimulationReport::new(
		data.PrintJobID, 
		data.WorkflowID, 
		0, //TODO: Properly determine creation time, should probably be handled in database.rs
		search.get_cumulative_time(), 
		search.get_step_times_by_id()
	));
	let visited = Visited::new(workflow.WorkflowSteps.len());
	let _results = traverse_graph(&printjob, &visited, &workflow.WorkflowSteps.clone(), 0).await;
    let current_time_in_secs = SystemTime::now().duration_since(UNIX_EPOCH).expect("Issue discerning current time.").as_secs() as u32;

	return Ok(SimulationReport::new(PrintJobID, WorkflowID, current_time_in_secs, 25, HashMap::from([(2, 15)])));

}


// Assumes graph is acyclic and connected
// TODO: Guarantee the graph is acyclic and connected
async fn traverse_graph(print_job: &PrintJob, search: &Search, steps: &Vec<WFS>, step: usize){
	if !(search.visit(step)) { return; }
	
	// Recursively visit all previous nodes first
	traverse_list(&steps[step].Prev, print_job, search, steps).await;

	// Simulate the current step
	let result = simulate_step(print_job, &steps[step]).await;
	
	// Update times
	search.update_step_time_by_id(&steps[step].id, result);
	match Iterator::max(steps[step].Prev.iter().map(|&i| search.get_step_time_cumulative(i))){
		Some(data) => search.update_step_time_cumulative(step, result+data),
		None => search.update_step_time_cumulative(step, result)
	};

	// Recursively visit next nodes
	traverse_list(&steps[step].Next, print_job, search, steps).await;
}


async fn traverse_list(steps: &Vec<usize>, print_job: &PrintJob, search: &Search, all_steps: &Vec<WFS>){
	join_all(steps.iter().map(|&i| 
		traverse_graph(print_job, search, all_steps, i)
	).collect::<Vec<_>>()).await;
}


async fn simulate_step(print_job: &PrintJob, wfs: &WFS) -> u32 {
	let workflow_step = find_workflow_step(wfs.id).await.expect("WorkflowStep not found");
	return print_job.PageCount * workflow_step.TimePerPage + workflow_step.SetupTime;
}


impl Search {
	fn new(workflow: &Workflow) -> Search {
		return Search(RwLock::new(SearchData{
			visited: vec![false; workflow.WorkflowSteps.len()],
			step_times_cumulative: vec![0; workflow.WorkflowSteps.len()],
			step_times_by_id: HashMap::new(),
			cumulative_time: 0
		}));
	}

	// Returns false to indicate that an index can't be visited because 
	// it already has been, otherwise sets visited[index] to true 
	// and returns true to indicate that it's OK to evaluate that step
	fn visit(&self, index: usize) -> bool {
		if self.0.read().unwrap().visited[index] == true { return false; }
		let mut visited = self.0.write().unwrap();
		visited.visited[index] = true;
		return true;
	}

	// Sets the cumulative time to reach the end of a step and keeps 
	// track of the cumulative time overall
	fn update_step_time_cumulative(&self, step: usize, time: u32){
		let mut write_lock = self.0.write().unwrap();
		write_lock.step_times_cumulative[step] = time;
		if time > write_lock.cumulative_time { 
			write_lock.cumulative_time = time;
		}
	}

	fn get_step_time_cumulative(&self, step: usize) -> u32 {
		return self.0.read().unwrap().step_times_cumulative[step];
	}

	fn update_step_time_by_id(&self, id: &DocID, time: u32){
		self.0.write().unwrap().step_times_by_id.entry(*id)
			.and_modify(|val| *val += time).or_insert(time);
	}

	fn get_step_times_by_id(&self) -> HashMap<DocID,u32> {
		return self.0.read().unwrap().step_times_by_id.clone();
	}

	fn get_cumulative_time(&self) -> u32 {
		return self.0.read().unwrap().cumulative_time;
	}
// TODO: I expect we'll probably store the time/cost/other details from each step into the
// database here. There is a table in the database called ran_workflow_step that associates an
// AssignedWorkflowStep with a simulation_report_id & time_taken value
async fn traverse_graph(print_job: &PrintJob, visited: &Visited, steps: &Vec<AssignedWorkflowStep>, step: usize) -> bool {
	if !(visited.visit(step)) || !(visited.can_visit()) { return false; }

	// let previouses = steps[step].Prev.iter().map(|&i| traverse_graph(print_job, visited, steps, i)).collect();
	// join!(previouses);
	
	// for i in &steps[step].Prev {
	// 	traverse_graph(print_job, visited, steps, *i);
	// }
	// TODO: Simulate step
	// for i in &steps[step].Next {
	// 	traverse_graph(print_job, visited, steps, *i);
	// }

	return true;
}
