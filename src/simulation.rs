use std::{
	sync::RwLock,
	collections::HashMap,
    time::{SystemTime, UNIX_EPOCH}
};

use crate::database::{*};

struct Visited { 
	data: RwLock<(Vec<bool>,usize)>
}
impl Visited {
	fn new(workflow_steps: usize) -> Visited {
		return Visited { data: RwLock::new((vec![false; workflow_steps], 0)) }
	}

	fn visit(&self, index: usize) -> bool {
		if self.data.read().unwrap().0[index] == true { return false; }
		let mut visited = self.data.write().unwrap();
		visited.0[index] = true;
		visited.1 += 1;
		return true;
	}

	fn can_visit(&self) -> bool {
		return self.data.read().unwrap().1 < self.data.read().unwrap().0.len();
	}
}


//TODO: async?
pub async fn simulate(PrintJobID : u32, WorkflowID: u32) -> Result<SimulationReport,String> {
	
    let printjob: PrintJob = match find_print_job(PrintJobID).await{
		Ok(data) => data,
		Err(_) => return Err("PrintJob not found".to_string())
	};
    
    let workflow = match find_workflow(WorkflowID).await{
        Ok(data) => data,
        Err(_) => return Err("Workflow not found".to_string())
    };

	// Graph Search
	let visited = Visited::new(workflow.WorkflowSteps.len());
	let _results = traverse_graph(&printjob, &visited, &workflow.WorkflowSteps.clone(), 0).await;
    let current_time_in_secs = SystemTime::now().duration_since(UNIX_EPOCH).expect("Issue discerning current time.").as_secs() as u32;

	return Ok(SimulationReport::new(PrintJobID, WorkflowID, current_time_in_secs, 25, HashMap::from([(2, 15)])));

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
