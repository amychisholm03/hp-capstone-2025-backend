const { test, before, after } = require('node:test');
const assert = require('node:assert');
const { MongoClient, Int32, ObjectId } = require("mongodb");
const { dbConnect, newPrintJob, newWorkflow, newWorkflowStep, newSimulationReport } = require("../../mongodb.js");

let database;

before(async () => {
	const [client, db] = await dbConnect("mongodb://localhost:27017");
	database = db;
});

after(async () => {
	const collections = await database.listCollections().toArray();
	await Promise.all(collections.map(c => database.collection(c.name).deleteMany({})));
});

test('newPrintJob - valid', async () => {
	const id = await newPrintJob(database, "Test Print Job", Int32(10), ["Profile 1"]);
	assert.ok(ObjectId.isValid(id), "Should return a valid ObjectId\n");
});

test('newPrintJob - invalid', async () => {
	await assert.rejects(
		() => newPrintJob(database, "", Int32(0), []),
		{ message: /Invalid parameters for newPrintJob/ },
		"Should throw an error for invalid parameters\n"
	);
});

test('newWorkflow - valid', async () => {
	const stepId = new ObjectId();
	const id = await newWorkflow(database, "Test Workflow", [stepId]);
	assert.ok(ObjectId.isValid(id), "Should return a valid ObjectId\n");
});

test('newWorkflow - invalid', async () => {
	await assert.rejects(
		() => newWorkflow(database, "", []),
		{ message: /Invalid parameters for newWorkflow/ },
		"Should throw an error for invalid parameters\n"
	);
});

test('newWorkflowStep - valid', async () => {
	const id = await newWorkflowStep(database, "Test Workflow Step", null, null, Int32(5), Int32(2));
	assert.ok(ObjectId.isValid(id), "Should return a valid ObjectId\n");
});

test('newWorkflowStep - invalid', async () => {
	await assert.rejects(
		() => newWorkflowStep(database, "", null, null, Int32(0), Int32(0)),
		{ message: /Invalid parameters for newWorkflowStep/ },
		"Should throw an error for invalid parameters\n"
	);
});

test('newSimulationReport - valid', async () => {
	const printJobId = new ObjectId();
	const workflowId = new ObjectId();
	const id = await newSimulationReport(
		database,
		printJobId,
		workflowId,
		Int32(30),
		{ step1: 10 },
		Int32(20)
	);
	assert.ok(ObjectId.isValid(id), "Should return a valid ObjectId\n");
});

test('newSimulationReport - invalid', async () => {
	await assert.rejects(
		() => newSimulationReport(database, null, null, Int32(0), null, Int32(0)),
		{ message: /Invalid parameters for newSimulationReport/ },
		"Should throw an error for invalid parameters\n"
	);
});