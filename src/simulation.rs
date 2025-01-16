use std::{
	sync::RwLock,
	collections::HashMap
};
use futures::future::join_all;

use crate::database::{*};


struct SearchData {
	visited: Vec<bool>,
	step_times: Vec<u32>,
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

	// Graph Search
	let search = Search::new(&workflow);
	traverse_graph(&print_job, &search, &workflow.WorkflowSteps.clone(), 0).await;

	dbg!(search.get_step_times());
	dbg!(search.get_step_times_by_id());
	dbg!(&search.0.read().unwrap().step_times_cumulative);
	dbg!(&search.0.read().unwrap().cumulative_time);
	// Format Output
	return Ok(SimulationReport::new(
		data.PrintJobID, 
		data.WorkflowID, 
		0, //TODO: Properly determine creation time, should probably be handled in database.rs
		search.get_cumulative_time(), 
		search.get_step_times_by_id()
	));
}


// Assumes graph is acyclic and connected
// TODO: Guarantee the graph is acyclic and connected
async fn traverse_graph(print_job: &PrintJob, search: &Search, steps: &Vec<WFS>, step: usize){
	if !(search.visit(step)) { return; }
	println!("Visiting {step}");
	
	// Visit all previous nodes first
	traverse_list(&steps[step].Prev, print_job, search, steps).await;
	println!("Visit all {step}'s prev steps");

	// Simulate the current step
	let result = simulate_step(print_job, &steps[step]).await;
	
	// Update times
	search.update_step_time(&steps[step].id, step, result);
	match Iterator::max(steps[step].Prev.iter().map(|&i| search.get_step_time_cumulative(i))){
		Some(data) => search.update_step_time_cumulative(step, result+data),
		None => search.update_step_time_cumulative(step, result)
	};

	// Visit next nodes
	traverse_list(&steps[step].Next, print_job, search, steps).await;
	println!("Visit all {step}'s next steps");
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
		return Search ( RwLock::new(SearchData{
			visited: vec![false; workflow.WorkflowSteps.len()], 
			step_times: vec![0; workflow.WorkflowSteps.len()],
			step_times_cumulative: vec![0; workflow.WorkflowSteps.len()],
			step_times_by_id: HashMap::new(),
			cumulative_time: 0
		}));
	}

	fn visit(&self, index: usize) -> bool {
		if self.0.read().unwrap().visited[index] == true { return false; }
		let mut visited = self.0.write().unwrap();
		visited.visited[index] = true;
		return true;
	}

	fn update_step_time(&self, id: &DocID, step: usize, time: u32){
		let mut write_lock = self.0.write().unwrap();
		write_lock.step_times[step] = time;
		write_lock.step_times_by_id.entry(*id)
			.and_modify(|val| *val += time).or_insert(time);
	}

	fn get_step_times(&self) -> Vec<u32> {
		return self.0.read().unwrap().step_times.clone();
	}

	fn update_step_time_cumulative(&self, step: usize, time: u32){
		self.0.write().unwrap().step_times_cumulative[step] = time;
		/*Acquire read lock, then drop when it goes out of scope*/{ 
			if time < self.0.read().unwrap().cumulative_time { return; }
		} self.0.write().unwrap().cumulative_time = time;
	}

	fn get_step_time_cumulative(&self, step: usize) -> u32 {
		return self.0.read().unwrap().step_times_cumulative[step];
	}


	fn get_step_times_by_id(&self) -> HashMap<DocID,u32> {
		return self.0.read().unwrap().step_times_by_id.clone();
	}


	fn get_cumulative_time(&self) -> u32 {
		return self.0.read().unwrap().cumulative_time;
	}
}