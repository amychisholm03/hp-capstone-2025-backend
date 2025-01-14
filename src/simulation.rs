use std::collections::HashMap;

use crate::database::{*};

pub fn simulate(data: SimulationReportArgs) -> Result<SimulationReport,String> {
	let printjob = match find_workflow(data.WorkflowID){
		Some(data) => data,
		None => return Err("PrintJob Not Found".to_string())
	};
	let workflow = match find_workflow(data.WorkflowID){
		Some(data) => data,
		None => return Err("Workflow Not Found".to_string())
	};





	return Ok(SimulationReport::new(data.PrintJobID, data.WorkflowID, 6, 25, HashMap::from([(2, 15)])));
}