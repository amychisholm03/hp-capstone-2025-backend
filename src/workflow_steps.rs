/**
 * {
 * 		step_val, 		Enum variant of the step
 * 		step_props,		Database ID of the step's properties, such as num_cores, can be NULL
 * 		prev [],		List of indices of previous steps
 * 		next [],		List of indices of next steps
 * }
 **/
use futures::future::try_join_all;
use serde::{Serialize, Deserialize};
use std::{
	collections::{HashMap,HashSet}
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tokio::sync::OnceCell;
use crate::database::*;


// This is the workflow step struct that gets returned from API calls
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    #[serde(default)]
    pub id: Option<DocID>,
    pub Title: String,
    pub SetupTime: u32,
    pub TimePerPage: u32,
}

/**
 * To add a new Workflow Step, add a new variant to the enum below, 
 * define its attributes in the get_attributes() function, and if
 * the variant has any fields, it must be added to fill_properties()
 * The compiler will give an error if either is missing
 * 
 * Any variants with additional properties will require an additional 
 * table in the database, see rasterization_params table for reference
 * 
 * At runtime, any new enums will be automatically added to the 
 * database and any removed enums will be removed from the database, 
 * giving an error before starting the server if there are any 
 * foreign key constraints
 **/

#[derive(Clone,Copy,Debug,EnumIter)]
pub enum WFSVariant {
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
pub async fn get_workflow_step_by_id(wfs_id: DocID, prop_id: Option<DocID>) -> Result<WFSVariant,CustomError> {
	return get_variant_by_id(wfs_id)?.fill_properties(prop_id).await;
}


impl WorkflowStep {
	pub async fn get(id: DocID) -> Result<WorkflowStep,CustomError> {
		let wfs = get_variant_by_id(id)?;
		return Ok(WorkflowStep {
			id: Some(id),
			Title: wfs.title().await,
			SetupTime: wfs.setup_time().await,
			TimePerPage: wfs.time_per_page().await
		});
	}
}


pub async fn get_all_workflow_steps() -> Vec<WorkflowStep> {
	let mut output = Vec::<WorkflowStep>::new();
	for variant in WFSVariant::iter() {
		output.push(WorkflowStep::get(variant.id().await).await.expect(""));
	}
	return output;
}


impl WFSVariant {
	// Retrieve a specific attribute for a given Workflow Step
	pub async fn id(&self) -> DocID { self.get_attributes().await.id }
	pub async fn title(&self) -> String { self.get_attributes().await.title }
	pub async fn setup_time(&self) -> u32 { self.get_attributes().await.setup_time }
	pub async fn time_per_page(&self) -> u32 { self.get_attributes().await.time_per_page }


	// For enum variants with fields, this function retrieves them from
	// the database, otherwise it does nothing
	async fn fill_properties(&mut self, prop_id: Option<DocID>) -> Result<WFSVariant,CustomError> {
		use WFSVariant::*;
		match self {
			Rasterization { num_cores } => {
				if let Some(id) = prop_id {
					*num_cores = find_rasterization_params(id).await?;
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
	async fn get_attributes(&self) -> Attributes {
		use WFSVariant::*;
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


static ID_TABLE: OnceCell<HashMap<DocID,WFSVariant>> = OnceCell::const_new();
pub fn get_variant_by_id(id: DocID) -> Result<WFSVariant,CustomError> {
	return ID_TABLE.get()
		.and_then(|table| { table.get(&id).copied() })
		.ok_or_else(|| CustomError::OtherError("WorkflowStep not found".to_string()));
}


pub async fn build_workflow_step_table() -> Result<(),CustomError> {
	if let Some(_) = ID_TABLE.get() { return Ok(()); }
	let mut lookup_table = HashMap::<DocID,WFSVariant>::new();
	let mut in_db = HashSet::<DocID>::from_iter(get_workflow_step_ids().await?);

	for variant in WFSVariant::iter() {
		let id = variant.id().await;
		lookup_table.insert(id, variant);
		if in_db.contains(&id) { in_db.remove(&id); }
		else { insert_workflow_step(id).await?; }
	}
	
	try_join_all(in_db.into_iter().map(|id| remove_workflow_step(id))).await?;
	ID_TABLE.set(lookup_table)?;
	return Ok(());
}


pub fn get_wfs_param_table(id: DocID) -> Option<String> {
	use WFSVariant::*;
	return match get_variant_by_id(id).expect("") {
		Rasterization {..} => Some("rasterization_params".to_string()),
		_ => None
	}
}