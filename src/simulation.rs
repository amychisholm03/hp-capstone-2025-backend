use std::collections::HashMap;

use crate::database::{*};

pub fn simulate(data: SimulationReportArgs) -> SimulationReport {
	return SimulationReport::new(data.PrintJobID, data.WorkflowID, 6, 25, HashMap::from([(2, 15)]));
}