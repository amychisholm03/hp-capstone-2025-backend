use backend::workflow::*;
use backend::workflow_steps::*;
use serde_json::json;

/// Testing workflow serialization
#[tokio::test]
async fn workflow_serialization() {
	let wf = Workflow {
		id: Some(5),
		title: "Test Workflow".to_string(),
		steps: vec![
			WorkflowNode { 
				data: WFSVariant::DownloadFile,
				prev: vec![],
				next: vec![1]
			},
			WorkflowNode {
				data: WFSVariant::Rasterization {num_cores: 7},
				prev: vec![0],
				next: vec![]
			}
		]
	};
    assert!(json!(wf).is_object());
	println!("Serialized:\n{}\n", json!(wf));
}

/// Testing workflow deserialization
#[tokio::test]
async fn workflow_deserialization() {
	let data = "{
		\"title\": \"Test Workflow 2\", 
		\"steps\": [
			{\"id\": 0}, 
			{\"id\": 1}, 
			{\"id\": 5, \"num_cores\": 3}
		]
	}";
    // TODO: caleb fix this
    // let json = serde_json::from_str::<Workflow>(data).unwrap();
	// println!("Deserialized: {:?}", &json);
}
