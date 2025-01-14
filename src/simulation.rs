use std::{
	sync::RwLock,
	collections::HashMap
};
use futures::future::join_all;

use crate::database::{*};


async fn static_testing() -> DocID {
	// let workflow_steps = query_workflow_steps().expect("Failed to get WorkflowSteps");
	// dbg!(workflow_steps);
	// Graph 1 - Linear
	let workflow: Workflow = serde_json::from_str(&format!("{{
		\"Title\":\"Simulation Testing Workflow\",
		\"WorkflowSteps\":[
			{{\"Next\":[1],\"Prev\":[],\"id\":2}},
			{{\"Next\":[2],\"Prev\":[0],\"id\":3}},
			{{\"Next\":[],\"Prev\":[2],\"id\":4}}
		]
	}}")).expect("Failed to deserialize");
	// // Graph 2 - Split, recombine, simple
	// let workflow: Workflow = serde_json::from_str(&format!("{{
	// 	\"Title\":\"Simulation Testing Workflow\",
	// 	\"WorkflowSteps\":[
	// 		{{\"Next\":[1, 2],\"Prev\":[],\"id\":2}},
	// 		{{\"Next\":[3],\"Prev\":[0],\"id\":3}},
	// 		{{\"Next\":[3],\"Prev\":[0],\"id\":4}},
	// 		{{\"Next\":[],\"Prev\":[1, 2],\"id\":2}}
	// 	]
	// }}")).expect("Failed to deserialize");
	// // Graph 3 - Split, recombine, complex
	// let workflow: Workflow = serde_json::from_str(&format!("{{
	// 	\"Title\":\"Simulation Testing Workflow\",
	// 	\"WorkflowSteps\":[
	// 		{{\"Next\":[1, 2, 3],\"Prev\":[],\"id\":2}},
	// 		{{\"Next\":[5],\"Prev\":[0],\"id\":3}},
	// 		{{\"Next\":[4],\"Prev\":[0],\"id\":4}},
	// 		{{\"Next\":[4],\"Prev\":[0],\"id\":2}},
	// 		{{\"Next\":[5],\"Prev\":[2, 3],\"id\":3}},
	// 		{{\"Next\":[],\"Prev\":[1, 4],\"id\":4}}
	// 	]
	// }}")).expect("Failed to deserialize");
	// // Graph 4 - Split, don't recombine
	// let workflow: Workflow = serde_json::from_str(&format!("{{
	// 	\"Title\":\"Simulation Testing Workflow\",
	// 	\"WorkflowSteps\":[
	// 		{{\"Next\":[1, 2, 3],\"Prev\":[],\"id\":2}},
	// 		{{\"Next\":[4],\"Prev\":[0],\"id\":3}},
	// 		{{\"Next\":[5],\"Prev\":[0],\"id\":4}},
	// 		{{\"Next\":[5],\"Prev\":[0],\"id\":2}},
	// 		{{\"Next\":[],\"Prev\":[1],\"id\":3}},
	// 		{{\"Next\":[],\"Prev\":[2, 3],\"id\":4}}
	// 	]
	// }}")).expect("Failed to deserialize");
	return insert_workflow(workflow).await.expect("Failed to insert");
}


struct Visited { 
	data: RwLock<(Vec<bool>,usize,HashMap<DocID,u32>)>,
}
impl Visited {
	fn new(workflow_steps: usize) -> Visited {
		return Visited { 
			data: RwLock::new((vec![false; workflow_steps], 0, HashMap::new())),
		}
	}

	fn visit(&self, index: usize) -> bool {
		if self.data.read().unwrap().0[index] == true { return false; }
		let mut visited = self.data.write().unwrap();
		visited.0[index] = true;
		visited.1 += 1;
		return true;
	}

	fn add_result(&self, id: &DocID, result: u32){
		match self.data.read().unwrap().2.get(id) {
			Some(data) => self.data.write().unwrap().2.insert(*id, result+data),
			None => self.data.write().unwrap().2.insert(*id, result)
		};
	}

	fn get_result(&self, id: &DocID) -> u32 {
		return *self.data.read().unwrap().2.get(id).expect("ID not in HashMap");
	}
}


//TODO: async?
pub async fn simulate(data: SimulationReportArgs) -> Result<SimulationReport,String> {
	let static_wf_id = static_testing().await;
	let printjob: PrintJob = match find_print_job(data.PrintJobID).await{
		Ok(data) => data,
		Err(_) => return Err("PrintJob not found".to_string())
	};
	//TODO: Make dynamic
	// let workflow = match find_workflow(data.WorkflowID).await{
	let workflow: Workflow = match find_workflow(static_wf_id).await{
		Ok(data) => data,
		Err(_) => return Err("Workflow not found".to_string())
	};
	dbg!(printjob.clone());
	dbg!(workflow.clone());


	// Graph Search
	let visited = Visited::new(workflow.WorkflowSteps.len());
	traverse_graph(&printjob, &visited, &workflow.WorkflowSteps.clone(), 0).await;


	return Ok(SimulationReport::new(data.PrintJobID, data.WorkflowID, 6, 25, HashMap::from([(2, 15)])));
}


async fn traverse_graph(print_job: &PrintJob, visited: &Visited, steps: &Vec<WFS>, step: usize) {
	if !(visited.visit(step)) { return; }
	
	// Visit all previous nodes first
	join_all(steps[step].Prev.iter().map(|&i| 
		traverse_graph(print_job, visited, steps, i)
	).collect::<Vec<_>>()).await;

	// Simulate the current step
	visited.add_result(&steps[step].id, simulate_step(print_job, &steps[step]).await);
	Iterator::max(steps[step].Prev.iter().map(|&i| visited.get_result(&steps[i].id)));

	// Visit next nodes
	join_all(steps[step].Next.iter().map(|&i| 
		traverse_graph(print_job, visited, steps, i)
	).collect::<Vec<_>>()).await;
}


async fn simulate_step(_printjob: &PrintJob, _workflow_step: &WFS) -> u32 {
	return 0;
}