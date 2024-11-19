// Db, Int32, and ObjectId are needed for IDE support to know what the types are
// eslint-disable-next-line no-unused-vars
const { Db, Int32, ObjectId, MongoClient, Timestamp } = require("mongodb"); 

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
	const database = client.db('hp');
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
	const wk_id1 = await newWorkflowStep(database, "Preflight", null, null, 10, 7);
	const wk_id2 = await newWorkflowStep(database, "Metrics", wk_id1, null, 2, 1);
	const wk_id3 = await newWorkflowStep(database, "Rasterization", wk_id2, null, 50, 16);
	const wk_id4 = await newWorkflowStep(database, "Printing", wk_id3, null, 10, 7);
	const wk_id5 = await newWorkflowStep(database, "Cutting", wk_id4, null, 10, 7);
	const wk_id6 = await newWorkflowStep(database, "Laminating", wk_id5, null, 10, 7);
	const wf_id = await newWorkflow(database, "Workflow 1", [wk_id1, wk_id2, wk_id3, wk_id4, wk_id5, wk_id6]);
	await newSimulationReport(database, pj_id, wf_id, 3, 4);
}

/**
 * Inserts a document (i.e. instance/row) into the given collection (i.e. table).
 * @param {Db} database The Mongo database object
 * @param {string} collection_name The collection name
 * @param {*} doc The document to insert into the database
 * @returns {ObjectId} The ID of the inserted document or an Error if it failed
 */
async function insert(database, collection_name, doc) {
	const collection = database.collection(collection_name);
	const insert = await collection.insertOne(doc);
	if (insert.acknowledged) {
		return insert.insertedId;
	} else {
		throw new Error("Insert into " + collection_name + " failed");
	}
}

/**
 * Inserts a new print job into the database.
 * @param {Db} database 
 * @param {string} title 
 * @param {Int32} page_count 
 * @param {string[]} rasterization_profile 
 * @returns {ObjectId} The ID of the inserted print job or an Error if it failed
 */
async function newPrintJob(database, title, page_count, rasterization_profile) {
	// TODO: Check the validity of foreign keys
	if (!database || !title || !page_count || !rasterization_profile || rasterization_profile.length == 0) {
		throw new Error("Invalid parameters for newPrintJob");
	}
	return insert(database, "PrintJob", {
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
 * @returns {ObjectId} The ID of the inserted workflow or an Error if it failed
 */
async function newWorkflow(database, title, workflow_steps) {
	if (!database || !title || !workflow_steps || workflow_steps.length == 0) {
		throw new Error("Invalid parameters for newWorkflow");
	}
	return insert(database, "Workflow", {
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
 * @returns {ObjectId} The ID of the inserted step or an Error if it failed
 */
async function newWorkflowStep(database, title, previous_step=null, next_step=null, setup_time=0, time_per_page=1) {
	// previous_step and next_step are ok to be null
	if (!database || !title || !setup_time || !time_per_page) {
		throw new Error("Invalid parameters for newWorkflowStep");
	}
	return insert(database, "WorkflowStep", {
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
 * @returns {ObjectId} The ID of the inserted report or an Error if it failed
 */
async function newSimulationReport(database, print_job_id, workflow_id, total_time_taken, rasterization_time_taken) {
	if (!database || !print_job_id || !workflow_id || !total_time_taken || !rasterization_time_taken) {
		throw new Error("Invalid parameters for newSimulationReport");
	}
	return insert(database, "SimulationReport", {
		PrintJobID: print_job_id,
		WorkflowID: workflow_id,
		TotalTimeTaken: total_time_taken,
		RasterizationTimeTaken: rasterization_time_taken,
		CreationTime: Date.now()
	});
}

module.exports = {
	dbConnect,
	dbSetup,
	newPrintJob,
	newWorkflow,
	newWorkflowStep,
	newSimulationReport
};
