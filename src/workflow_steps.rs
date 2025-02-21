use std::{
	collections::HashMap,
	sync::OnceLock,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::database::{
	DocID,
	CustomError
};

/**
 * {
 * 		step_val, 		Enum variant of the step
 * 		step_props,		Database ID of the step's properties, such as num_cores, can be NULL
 * 		prev [],		List of indices of previous steps
 * 		next [],		List of indices of next steps
 * }
 **/

 /**
  * To add a new Workflow Step, add a new variant to the enum below, 
  * define its attributes in the get_attributes() function, and if
  * the variant has any fields, it must be added to fill_properties()
  * The compiler will give an error if either is missing
  **/

#[derive(Clone,Copy,Debug,EnumIter)]
pub enum WorkflowStep {
	DownloadFile,
	Preflight,
	Impose,
	Analyzer,
	ColorSetup,
	Rasterization {
		num_cores: u32,
	},
	Loader,
	Cutting,
	Laminating,
	Metrics,
}


struct Attributes {
	id: DocID,
	title: String,
	setup_time: u32,
	time_per_page: u32,
}


// Gets a Workflow Step by its ID and fills its properties, if applicable
// TODO: Convert to result
static ID_TABLE: OnceLock<HashMap<DocID,WorkflowStep>> = OnceLock::new();
pub fn get_workflow_step_by_id(wfs_id: DocID, prop_id: Option<DocID>) -> Result<WorkflowStep,CustomError> {
	return match ID_TABLE.get_or_init(build_id_table).get(&wfs_id).copied() {
		Some(mut variant) => variant.fill_properties(prop_id),
		None => Err(CustomError::OtherError("WorkflowStep not found".to_string()))
	};
}


impl WorkflowStep {
	// Retrieve a specific attribute for a given Workflow Step
	pub fn id(&self) -> DocID { self.get_attributes().id }
	pub fn title(&self) -> String { self.get_attributes().title }
	pub fn setup_time(&self) -> u32 { self.get_attributes().setup_time }
	pub fn time_per_page(&self) -> u32 { self.get_attributes().time_per_page }

	// For enum variants with fields, this function retrieves them from
	// the database, otherwise it does nothing
	// TODO: Convert to result
	fn fill_properties(&mut self, prop_id: Option<DocID>) -> Result<WorkflowStep,CustomError> {
		use WorkflowStep::*;
		match self {
			Rasterization { num_cores } => {
				if let Some(_id) = prop_id {
					// TODO: Database stuff
					*num_cores = 5;
					return Ok(*self);
				} else { return Err(CustomError::OtherError(
					"Rasterization requires prop_id".to_string())); }
			},
			_ => { // This will only match enum variants without fields
				if let Some(_) = prop_id { return Err(CustomError::OtherError(
					"Given WorkflowStep doesn't require prop_id".to_string())); }
				else { return Ok(*self); }
			}
		}
	}

	// This is where a Workflow Step's static attributes are defined
	// Public functions call this one to retrieve specific attributes
	fn get_attributes(&self) -> Attributes {
		use WorkflowStep::*;
		return match self {
			DownloadFile => Attributes {
				id: 0,
				title: "Download File".to_string(),
				setup_time: 0,
				time_per_page: 1,
			},

			Preflight => Attributes {
				id: 1,
				title: "Preflight".to_string(),
				setup_time: 10,
				time_per_page: 20,
			},
			
			Impose => Attributes {
				id: 2,
				title: "Impose".to_string(),
				setup_time: 0,
				time_per_page: 5,
			},
			
			Analyzer => Attributes {
				id: 3,
				title: "Analyzer".to_string(),
				setup_time: 0,
				time_per_page: 5,
			},
			
			ColorSetup => Attributes {
				id: 4,
				title: "Color Setup".to_string(),
				setup_time: 2,
				time_per_page: 1,
			},
			
			Rasterization {..} => Attributes {
				id: 5,
				title: "Rasterization".to_string(),
				setup_time: 50,
				time_per_page: 15,
			},
			
			Loader => Attributes {
				id: 6,
				title: "Loader".to_string(),
				setup_time: 100,
				time_per_page: 1,
			},
			
			Cutting => Attributes {
				id: 7,
				title: "Cutting".to_string(),
				setup_time: 10,
				time_per_page: 2,
			},
			
			Laminating => Attributes {
				id: 8,
				title: "Laminating".to_string(),
				setup_time: 10,
				time_per_page: 5
			},
			
			Metrics => Attributes {
				id: 9,
				title: "Metrics".to_string(),
				setup_time: 2,
				time_per_page: 1,
			},
		}
	}
}


// Iterates through the enum variants and builds a table for finding
// the variant given its ID
fn build_id_table() -> HashMap<DocID,WorkflowStep> {
	let mut output = HashMap::<DocID,WorkflowStep>::new();
	for variant in WorkflowStep::iter() {
		output.insert(variant.id(), variant);
	}
	return output;
}