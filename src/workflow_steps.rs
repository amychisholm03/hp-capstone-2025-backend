use crate::database::*;
/**
 * {
 * 		step_val, 		Enum variant of the step
 * 		step_props,		Database ID of the step's properties, such as num_cores, can be NULL
 * 		prev [],		List of indices of previous steps
 * 		next [],		List of indices of next steps
 * }
 **/
use futures::future::try_join_all;
use serde::de::{Deserializer, Error};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tokio::sync::OnceCell;

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
 * define its WFSAttributes in the get_WFSAttributes() function, and if
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

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
pub enum WFSVariant {
    DownloadFile,
    Preflight,
    Impose,
    Analyzer,
    ColorSetup,
    Rasterization { num_cores: u32 },
    Loader,
    Cutting,
    Laminating,
    Metrics,
}

/// Static properties of each Workflow Step
struct WFSAttributes {
    id: DocID,
    title: String,
    setup_time: u32,
    time_per_page: u32,
    /// List of valid previous steps for this type of step
    valid_prev: Vec<WFSVariant>,
    /// List of valid next steps for this type of step
    valid_next: Vec<WFSVariant>,
    /// Can this type of step be the first step in a workflow?
    no_prev_valid: bool,
    /// Can this type of step be the last step in a workflow?
    no_next_valid: bool,
}

// Gets a Workflow Step by its ID and fills its properties, if applicable
pub async fn get_workflow_step_by_id(
    wfs_id: DocID,
    prop_id: Option<DocID>,
) -> Result<WFSVariant, CustomError> {
    return get_variant_by_id(wfs_id)?.fill_properties(prop_id).await;
}

impl WorkflowStep {
    pub async fn get(id: DocID) -> Result<WorkflowStep, CustomError> {
        let wfs = get_variant_by_id(id)?;
        return Ok(WorkflowStep {
            id: Some(id),
            Title: wfs.title(),
            SetupTime: wfs.setup_time(),
            TimePerPage: wfs.time_per_page(),
        });
    }
}

pub async fn get_all_workflow_steps() -> Vec<WorkflowStep> {
    let mut output = Vec::<WorkflowStep>::new();
    for variant in WFSVariant::iter() {
        output.push(WorkflowStep::get(variant.id()).await.expect(""));
    }
    return output;
}

impl WFSVariant {
    // Retrieve a specific attribute for a given Workflow Step
    pub fn id(&self) -> DocID {
        self.get_wf_step_attributes().id
    }
    pub fn title(&self) -> String {
        self.get_wf_step_attributes().title
    }
    pub fn setup_time(&self) -> u32 {
        self.get_wf_step_attributes().setup_time
    }
    pub fn time_per_page(&self) -> u32 {
        self.get_wf_step_attributes().time_per_page
    }
    pub fn valid_prev(&self) -> Vec<WFSVariant> {
        self.get_wf_step_attributes().valid_prev
    }
    pub fn valid_next(&self) -> Vec<WFSVariant> {
        self.get_wf_step_attributes().valid_next
    }
    pub fn no_prev_valid(&self) -> bool {
        self.get_wf_step_attributes().no_prev_valid
    }
    pub fn no_next_valid(&self) -> bool {
        self.get_wf_step_attributes().no_next_valid
    }

    /// For enum variants with fields, this function retrieves them from
    /// the database, otherwise it does nothing
    async fn fill_properties(&mut self, prop_id: Option<DocID>) -> Result<WFSVariant, CustomError> {
        use WFSVariant::*;
        match self {
            Rasterization { num_cores } => {
                if let Some(id) = prop_id {
                    *num_cores = find_rasterization_params(id).await?;
                    return Ok(*self);
                } else {
                    return Err(CustomError::OtherError(
                        "Rasterization requires prop_id".to_string(),
                    ));
                }
            }

            _ => {
                // This will only match enum variants without fields
                if let Some(_) = prop_id {
                    return Err(CustomError::OtherError(
                        "Given WorkflowStep doesn't require prop_id".to_string(),
                    ));
                } else {
                    return Ok(*self);
                }
            }
        }
    }

    /// This is where a Workflow Step's static aAttributes are defined
    /// Public functions call this one to retrieve specific attributes
    fn get_wf_step_attributes(&self) -> WFSAttributes {
        use WFSVariant::*;
        return match self {
            DownloadFile => WFSAttributes {
                id: 0,
                title: "Download File".to_string(),
                setup_time: 0,
                time_per_page: 1,
                valid_prev: vec![],
                valid_next: vec![Preflight],
                no_prev_valid: true,
                no_next_valid: false,
            },

            Preflight => WFSAttributes {
                id: 1,
                title: "Preflight".to_string(),
                setup_time: 10,
                time_per_page: 20,
                valid_prev: vec![DownloadFile],
                valid_next: vec![Impose],
                no_prev_valid: false,
                no_next_valid: false,
            },

            Impose => WFSAttributes {
                id: 2,
                title: "Impose".to_string(),
                setup_time: 0,
                time_per_page: 5,
                valid_prev: vec![Preflight],
                valid_next: vec![Analyzer],
                no_prev_valid: false,
                no_next_valid: false,
            },

            Analyzer => WFSAttributes {
                id: 3,
                title: "Analyzer".to_string(),
                setup_time: 0,
                time_per_page: 5,
                valid_prev: vec![Impose],
                valid_next: vec![ColorSetup],
                no_prev_valid: false,
                no_next_valid: false,
            },

            ColorSetup => WFSAttributes {
                id: 4,
                title: "Color Setup".to_string(),
                setup_time: 2,
                time_per_page: 1,
                valid_prev: vec![Analyzer],
                // Any rasterization num_cores valid [1,10]
                valid_next: (1..=10)
                    .map(|num_cores| Rasterization { num_cores })
                    .collect(),
                no_prev_valid: false,
                no_next_valid: false,
            },

            Rasterization { .. } => WFSAttributes {
                id: 5,
                title: "Rasterization".to_string(),
                setup_time: 50,
                time_per_page: 15,
                valid_prev: vec![ColorSetup],
                valid_next: vec![Loader],
                no_prev_valid: false,
                no_next_valid: false,
            },

            Loader => WFSAttributes {
                id: 6,
                title: "Loader".to_string(),
                setup_time: 100,
                time_per_page: 1,
                // Any rasterization num_cores valid [1,10]
                valid_prev: (1..=10)
                    .map(|num_cores| Rasterization { num_cores })
                    .collect(),
                valid_next: vec![Cutting, Laminating, Metrics],
                no_prev_valid: false,
                no_next_valid: true,
            },

            Cutting => WFSAttributes {
                id: 7,
                title: "Cutting".to_string(),
                setup_time: 10,
                time_per_page: 2,
                valid_prev: vec![Loader, Metrics],
                valid_next: vec![Laminating, Metrics],
                no_prev_valid: false,
                no_next_valid: true,
            },

            Laminating => WFSAttributes {
                id: 8,
                title: "Laminating".to_string(),
                setup_time: 10,
                time_per_page: 5,
                valid_prev: vec![Loader, Cutting, Metrics],
                valid_next: vec![Metrics],
                no_prev_valid: false,
                no_next_valid: true,
            },

            Metrics => WFSAttributes {
                id: 9,
                title: "Metrics".to_string(),
                setup_time: 2,
                time_per_page: 1,
                valid_prev: vec![Loader, Cutting, Laminating],
                valid_next: vec![Cutting, Laminating],
                no_prev_valid: false,
                no_next_valid: true,
            },
        };
    }
}

impl Serialize for WFSVariant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use WFSVariant::*;
        let attr = self.get_wf_step_attributes();
        let mut state = serializer.serialize_struct("WFSAttributes", 4)?;
        state.serialize_field("id", &attr.id)?;
        state.serialize_field("title", &attr.title)?;
        state.serialize_field("setup_time", &attr.setup_time)?;
        state.serialize_field("time_per_page", &attr.time_per_page)?;

        match self {
            Rasterization { num_cores } => state.serialize_field("num_cores", num_cores)?,
            _ => {}
        }

        return state.end();
    }
}

impl<'de> Deserialize<'de> for WFSVariant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut fields: serde_json::Map<String, Value> = Deserialize::deserialize(deserializer)?;

        let mut output = get_variant_by_id(
            serde_json::from_value(
                fields
                    .remove("id")
                    .ok_or_else(|| Error::custom(format!("TODO")))?,
            )
            .map_err(|_| Error::custom(format!("TODO")))?,
        )
        .map_err(|_| Error::custom(format!("TODO")))?;

        match output {
            WFSVariant::Rasterization { ref mut num_cores } => {
                *num_cores = serde_json::from_value(
                    fields
                        .remove("num_cores")
                        .ok_or_else(|| Error::custom(format!("TODO")))?,
                )
                .map_err(|_| Error::custom(format!("TODO")))?;
            }
            _ => {}
        }

        if !fields.is_empty() {
            return Err(D::Error::custom(format!("TODO")));
        }

        return Ok(output);
    }
}

static ID_TABLE: OnceCell<HashMap<DocID, WFSVariant>> = OnceCell::const_new();
pub fn get_variant_by_id(id: DocID) -> Result<WFSVariant, CustomError> {
    return ID_TABLE
        .get()
        .and_then(|table| table.get(&id).copied())
        .ok_or_else(|| CustomError::OtherError("WorkflowStep not found".to_string()));
}

pub async fn build_workflow_step_table() -> Result<(), CustomError> {
    if let Some(_) = ID_TABLE.get() {
        return Ok(());
    }
    let mut lookup_table = HashMap::<DocID, WFSVariant>::new();
    let mut in_db = HashSet::<DocID>::from_iter(get_workflow_step_ids().await?);

    for variant in WFSVariant::iter() {
        let id = variant.id();
        lookup_table.insert(id, variant);
        if in_db.contains(&id) {
            in_db.remove(&id);
        } else {
            insert_workflow_step(id).await?;
        }
    }

    try_join_all(in_db.into_iter().map(|id| remove_workflow_step(id))).await?;
    ID_TABLE.set(lookup_table)?;
    return Ok(());
}

pub fn get_wfs_param_table(id: DocID) -> Option<String> {
    use WFSVariant::*;
    return match get_variant_by_id(id).expect("") {
        Rasterization { .. } => Some("rasterization_params".to_string()),
        _ => None,
    };
}
