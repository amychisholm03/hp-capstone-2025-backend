const { MongoClient, Timestamp } = require("mongodb");

module.exports = { db_connect, db_setup, new_print_job, new_workflow, new_workflow_step, new_simulation_report };


async function db_connect(){
	const uri = "mongodb://localhost:27017/HP";
	const client = new MongoClient(uri);
	console.log("Connecting to MongoDB...");
	await client.connect();
	console.log("Database connected");
	const database = client.db('HP');
	return [client, database];
}


// DELETES all existing collections, recreates them, then inserts
// some dummy data
async function db_setup(database){
	//Drop all collections
	const old_collections = await database.listCollections().toArray();
	Promise.all(old_collections.map((c) => database.collection(c.name).drop()));

	//Create collections
	const collections = ["PrintJob", "Workflow", "WorkflowStep", "SimulationReport"];
	Promise.all(collections.map((c) => database.createCollection(c)));

	//Insert dummy data
	const pj_id = await new_print_job(database, "PrintJob 1", 5, ["RP 1"]);
	const ws_id = await new_workflow_step(database, "WorkflowStep 1", null, null, 10, 7);
	const wf_id = await new_workflow(database, "Workflow 1", [ws_id]);
	await new_simulation_report(database, pj_id, wf_id, 3, 4);
}


async function insert(database, collection_name, doc){
	const collection = database.collection(collection_name);
	const insert = await collection.insertOne(doc);
	if(insert.acknowledged){
		//TODO: Do Stuff
	} else { 
		//TODO: Do other stuff
	}
	return insert.insertedId;
}


function check_null(args){
	for(const arg of args){
		if(arg == null) throw new Error("Invalid argument");
	}
}


//title is a string, page_count is an int, and rasterization_profile is an array of strings
//TODO: Check the validity of foreign keys
//TODO: Is there a way to place type constraints on a function?
async function new_print_job(database, title, page_count, rasterization_profile){
	check_null([database, title, page_count, rasterization_profile]);
	return await insert(database, "PrintJob", {
		Title: title, 
		DateCreated: new Timestamp(), 
		PageCount: page_count, 
		RasterizationProfile: rasterization_profile
	});
}



//title is a string, workflow_steps is an array of ObjectID()s
async function new_workflow(database, title, workflow_steps){
	check_null([database, title, workflow_steps]);
	return await insert(database, "Workflow", {
		Title: title, 
		WorkflowSteps: workflow_steps
	});
}


//title is a string, previous_step and next_step are ObjectID()s, setup_time and time_per_page are ints
async function new_workflow_step(database, title, previous_step, next_step, setup_time, time_per_page){
	check_null([database, title, setup_time, time_per_page]);
	return await insert(database, "WorkflowStep", {
		Title: title, 
		PreviousStep: previous_step,
		NextStep: next_step,
		SetupTime: setup_time,
		TimePerPage: time_per_page
	});
}


//print_job_id and workflow_id are ObjectID()s, total_time_taken and rasterization_time_taken are ints
async function new_simulation_report(database, print_job_id, workflow_id, total_time_taken, rasterization_time_taken){
	check_null([database, print_job_id, workflow_id, total_time_taken, rasterization_time_taken]);
	return await insert(database, "SimulationReport", {
		PrintJobID: print_job_id,
		WorkflowID: workflow_id,
		TotalTimeTaken: total_time_taken,
		RasterizationTimeTaken: rasterization_time_taken
	});
}