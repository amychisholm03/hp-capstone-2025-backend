use std::collections::HashMap;

use crate::database::{*};


// fn static_testing() -> DocID {
// 	let workflow_steps = 
// 	let workflow: Workflow = serde_json::from_str(&format!("{{
// 		\"Title\":\"Simulation Testing Workflow\",
// 		\"WorkflowSteps\":[
// 			{{\"Next\":[],\"Prev\":[],\"id\":2}}
// 		]
// 	}}")).expect("Failed to deserialize");
// 	return insert_workflow(workflow).expect("Failed to insert");
// }


pub fn simulate(data: SimulationReportArgs) -> Result<SimulationReport,String> {
	// let static_wf_id = static_testing();
	// let printjob: PrintJob = match find_print_job(data.PrintJobID){
	// 	// TODO: update find_print_job to return Result<PrintJob>
	// 	Some(data) => serde_json::from_str(&data).expect("Should not see"),
	// 	None => return Err("PrintJob Not Found".to_string())
	// };
	// //TODO: Make dynamic
	// // let workflow = match find_workflow(data.WorkflowID){
	// let workflow: Workflow = match find_workflow(static_wf_id){
	// 	// TODO: update find_workflow to return Result<Workflow>
	// 	Some(data) => serde_json::from_str(&data).expect("Should not see"),
	// 	None => return Err("Workflow Not Found".to_string())
	// };
	// dbg!(printjob.clone());
	// dbg!(workflow.clone());




	return Ok(SimulationReport::new(data.PrintJobID, data.WorkflowID, 6, 25, HashMap::from([(2, 15)])));
}