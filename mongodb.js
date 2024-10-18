const { MongoClient, Timestamp } = require("mongodb");


async function main() {
	console.log("main");
	[client, database] = db_connect();
	try {
		await db_setup(database);
	} finally { await client.close(); }
	console.log("Code ran successfully")
}



function db_connect(){
	console.log("db_connect");
	const uri = "mongodb://localhost:27017/HP";
	const client = new MongoClient(uri);
	const database = client.db('HP');
	return [client, database];
}



// DELETES all existing collections, recreates them, then inserts
// some dummy data
async function db_setup(database){
	console.log("db_setup");

	//Drop all collections
	const old_collections = await database.listCollections().toArray();
	for(const coll of old_collections){
		await database.collection(coll.name).drop();
	}

	//Create collections and insert dummy data
	const collections = ["PrintJob", "Workflow", "WorkflowStep", "SimulationReport"];
	for(i = 0; i < collections.length; i++){
		await database.createCollection(collections[i]);
	}
	const pj_id = await new_print_job(database, "PrintJob 1", 5, ["RP 1"]);
	const ws_id = await new_workflow_step(database, "WorkflowStep 1", null, null, 10, 7);
	const wf_id = await new_workflow(database, "Workflow 1", [ws_id]);
	await new_simulation_report(database, pj_id, wf_id, 3, 4);
}



//title is a string, page_count is an int, and rasterization_profile is an array of strings
//TODO: Is there a way to place type constraints on a function?
async function new_print_job(database, title, page_count, rasterization_profile){
	const PrintJob = database.collection("PrintJob");
	const insert = await PrintJob.insertOne({
		Title: title, 
		DateCreated: new Timestamp(), 
		PageCount: page_count, 
		RasterizationProfile: rasterization_profile
	});
	//TODO: Check value of insert.acknowledged
	return insert.insertedId;
}



//title is a string, workflow_steps is an array of ObjectID()s
async function new_workflow(database, title, workflow_steps){
	//TODO: verify that all inserted workflow steps exist
	const Workflow = database.collection("Workflow");
	const insert = await Workflow.insertOne({
		Title: title, 
		WorkflowSteps: workflow_steps
	});
	//TODO: Check value of insert.acknowledged
	return insert.insertedId;
}



//title is a string, previous_step and next_step are ObjectID()s, setup_time and time_per_page are ints
async function new_workflow_step(database, title, previous_step, next_step, setup_time, time_per_page){
	//TODO: verify that prev and next steps exist
	const WorkflowStep = database.collection("WorkflowStep");
	const insert = await WorkflowStep.insertOne({
		Title: title, 
		PreviousStep: previous_step,
		NextStep: next_step,
		SetupTime: setup_time,
		TimePerPage: time_per_page
	});
	//TODO: Check value of insert.acknowledged
	return insert.insertedId;
}



//print_job_id and workflow_id are ObjectID()s, total_time_taken and rasterization_time_taken are ints
async function new_simulation_report(database, print_job_id, workflow_id, total_time_taken, rasterization_time_taken){
	//TODO: verify that that the printjob and workflow exist
	const SimulationReport = database.collection("SimulationReport");
	const insert = await SimulationReport.insertOne({
		PrintJobID: print_job_id,
		WorkflowID: workflow_id,
		TotalTimeTaken: total_time_taken,
		RasterizationTimeTaken: rasterization_time_taken
	});
	//TODO: Check value of insert.acknowledged
	return insert.insertedId;
}


main().catch(console.dir);