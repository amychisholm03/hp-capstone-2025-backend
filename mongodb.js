import { Db, Int32, ObjectId, MongoClient, Timestamp } from "mongodb";
export default { dbConnect, dbSetup, newPrintJob, newWorfklow, newWorfklowStep, newSimulationReport };

/**
 * Connects to the MongoDB database and returns the client and database objects.
 * @param {string} url The URL of the MongoDB database connection
 * @returns {[MongoClient, Db]} A tuple containing the client and database objects.
 */
async function dbConnect(url) {
	const client = new MongoClient(url);
	console.log("Connecting to MongoDB...");
	await client.connect();
	console.log("Database connected");
	const database = client.db('HP');
	return [client, database];
}

/**
 * Deletes all existing collections, recreates them, then inserts some dummy data.
 * TODO: convert to a test in mongodb.test.js
 * @param {Db} database 
 */
async function dbSetup(database) {
	// Drop all collections
	const old_collections = await database.listCollections().toArray();
	await Promise.all(old_collections.map((c) => database.collection(c.name).drop()));

	// Create collections
	const collections = ["PrintJob", "Workflow", "WorkflowStep", "SimulationReport"];
	await Promise.all(collections.map((c) => database.createCollection(c)));

	// Insert dummy data
	const pj_id = await newPrintJob(database, "PrintJob 1", 5, ["RP 1"]);
	const ws_id = await newWorfklowStep(database, "WorkflowStep 1", null, null, 10, 7);
	const wf_id = await newWorfklow(database, "Workflow 1", [ws_id]);
	await newSimulationReport(database, pj_id, wf_id, 3, 4);
}

/**
 * 
 * @param {Db} database 
 * @param {*} collection_name 
 * @param {*} doc 
 * @returns 
 */
async function insert(database, collection_name, doc) {
	const collection = database.collection(collection_name);
	const insert = await collection.insertOne(doc);
	if (insert.acknowledged) {
		// TODO: Do Stuff
	} else {
		// TODO: Do other stuff
	}
	return insert.insertedId;
}

/**
 * Checks if the given array contains any null values.
 * @param {*[]} args 
 */
function checkNull(args) {
	for (const arg of args) {
		// TODO: are we sure about throwing an error here?
		if (arg === null) throw new Error("Invalid argument");
	}
}

/**
 * Inserts a new print job into the database.
 * @param {Db} database 
 * @param {string} title 
 * @param {Int32} page_count 
 * @param {string[]} rasterization_profile 
 * @returns 
 */
async function newPrintJob(database, title, page_count, rasterization_profile) {
	// TODO: Check the validity of foreign keys
	// TODO: Is there a way to place type constraints on a function?
	checkNull([database, title, page_count, rasterization_profile]);
	return await insert(database, "PrintJob", {
		Title: title,
		DateCreated: new Timestamp(),
		PageCount: page_count,
		RasterizationProfile: rasterization_profile
	});
}

/**
 * Inserts a new worfklow with its steps into the database.
 * @param {Db} database 
 * @param {string} title 
 * @param {ObjectId[]} workflow_steps 
 * @returns 
 */
async function newWorfklow(database, title, workflow_steps) {
	checkNull([database, title, workflow_steps]);
	return await insert(database, "Workflow", {
		Title: title,
		WorkflowSteps: workflow_steps
	});
}

/**
 * Inserts a new workflow step into the database.
 * @param {Db} database 
 * @param {string} title 
 * @param {ObjectId} previous_step 
 * @param {ObjectId} next_step 
 * @param {Int32} setup_time 
 * @param {Int32} time_per_page 
 * @returns 
 */
async function newWorfklowStep(database, title, previous_step, next_step, setup_time, time_per_page) {
	checkNull([database, title, setup_time, time_per_page]);
	return await insert(database, "WorkflowStep", {
		Title: title,
		PreviousStep: previous_step,
		NextStep: next_step,
		SetupTime: setup_time,
		TimePerPage: time_per_page
	});
}

/**
 * Inserts a new simulation report into the database.
 * @param {Db} database 
 * @param {ObjectId} print_job_id 
 * @param {ObjectId} workflow_id 
 * @param {Int32} total_time_taken 
 * @param {Int32} rasterization_time_taken 
 * @returns 
 */
async function newSimulationReport(database, print_job_id, workflow_id, total_time_taken, rasterization_time_taken) {
	checkNull([database, print_job_id, workflow_id, total_time_taken, rasterization_time_taken]);
	return await insert(database, "SimulationReport", {
		PrintJobID: print_job_id,
		WorkflowID: workflow_id,
		TotalTimeTaken: total_time_taken,
		RasterizationTimeTaken: rasterization_time_taken
	});
}