use std::collections::HashMap;

use crate::database::{*};

pub fn simulate(data: SimulationReportArgs) -> Option<SimulationReport> {
	return Some(SimulationReport::new(data.PrintJobID, data.WorkflowID, 6, 25, HashMap::from([(2, 15)])));
}