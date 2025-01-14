const { test, before, after } = require('node:test');
const assert = require('node:assert');
const { Int32, ObjectId } = require("mongodb");
const { dbConnect, dbSetup, newPrintJob, newWorkflow, newWorkflowStep, newSimulationReport } = require("../../mongodb.js");

let database = null;

before(async () => {
	// Setup and connect to mongodb
	const [_, db] = await dbConnect("mongodb://localhost:27017");
	await dbSetup(db);
	database = db;
});

after(async () => {
	// The next dbSetup call will drop the database
	// so no need to do it here
    process.exit(0);
});

test('newPrintJob - valid', async () => {
	const id = await newPrintJob(database, "Test Print Job", new Int32(10), ["Profile 1"]);
	assert.ok(ObjectId.isValid(id), "Should return a valid ObjectId\n");
});

test('newPrintJob - invalid', async () => {
	await assert.rejects(
		() => newPrintJob(database, "", new Int32(0), []),
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
	const id = await newWorkflowStep(database, "Test Workflow Step", null, null, new Int32(5), new Int32(2));
	assert.ok(ObjectId.isValid(id), "Should return a valid ObjectId\n");
});

test('newWorkflowStep - invalid', async () => {
	await assert.rejects(
		() => newWorkflowStep(database, "", null, null, new Int32(0), new Int32(0)),
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
		new Int32(30),
		{ step1: 10 },
		new Int32(20)
	);
	assert.ok(ObjectId.isValid(id), "Should return a valid ObjectId\n");
});

test('newSimulationReport - invalid', async () => {
	await assert.rejects(
		() => newSimulationReport(database, null, null, new Int32(0), null, new Int32(0)),
		{ message: /Invalid parameters for newSimulationReport/ },
		"Should throw an error for invalid parameters\n"
	);
});