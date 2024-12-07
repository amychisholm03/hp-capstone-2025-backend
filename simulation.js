const { Mutex } = require('async-mutex');
const { dbConnect, dbSetup, newSimulationReport } = require("./mongodb.js"); //TODO: Remove
const typedefs = require("./typedefs");
/** @type {
 * typedefs.PrintJob,
 * typedefs.WorkflowStep,
 * typedefs.Workflow,
 * typedefs.SimulationReport
 * } */

async function main() {
	const [_, database] = await dbConnect("mongodb://localhost:27017/hp");
	await dbSetup(database);
	const workflow = await database.collection("Workflow").find().toArray();
	const printJob = await database.collection("PrintJob").findOne();
	simulate(printJob, workflow[0], database);
}

/**
 * Simulate a PrintJob through the given Workflow
 * @param {PrintJob} printJob 
 * @param {Workflow} workflow 
 * @param {Db} database
 * @returns {ObjectID} the ID of the Simulation Report
 */
async function simulate(printJob, workflow, database) {
	// Format workflowSteps correctly for traverseGraph
	let workflowSteps = {}
	let temp = workflow.WorkflowSteps;
	for (let i = 0; i < temp.length; i++) {
		const step = await database.collection("WorkflowStep").findOne({ _id: temp[i] });
		if (!step) throw new Error("WorkflowStep not found");
		workflowSteps[temp[i]] = {
			func: step.Title,
			time: step.TimePerPage,
			prev: i == 0 ? [] : [temp[i - 1]],
			next: i == temp.length - 1 ? [] : [temp[i + 1]]
		};
	}

	// Get all workflow steps from graph along with corresponding times
	const results = await traverseGraph(printJob, workflowSteps,
		Object.keys(workflowSteps)[0],
		Object.fromEntries(Object.keys(workflowSteps).map((k) => [k, false])),
	);

	// Find times for report
	let rastTime = 0;
	let stepTimes = {};
	const totalTime = Math.max(...Object.keys(results).map((k) => {
		if (results[k].stepName === "Rasterization") rastTime = results[k].stepTime; //TODO: Remove
		//TODO: This doesn't properly support multiple steps of the same type
		stepTimes[results[k].stepName] = results[k].stepTime;
		return results[k].cumulative;
	}));

	return await newSimulationReport(database, printJob._id, workflow._id, totalTime, stepTimes, rastTime);
}


async function traverseGraph(printJob, workflowSteps, step, visited, mutex = new Mutex(), results = {}) {
	if (await isVisited(visited, step, mutex)) return;
	await Promise.all(workflowSteps[step].prev.map((k) =>
		traverseGraph(printJob, workflowSteps, k, visited, mutex, results)));
	const simulatedTime = await simulateStep(printJob, workflowSteps, step);
	results[step] = { stepName: workflowSteps[step].func, stepTime: simulatedTime, cumulative: simulatedTime };
	results[step].cumulative += Math.max(workflowSteps[step].prev.map((k) =>
		results[k].cumulative));
	await Promise.all(workflowSteps[step].next.map((k) =>
		traverseGraph(printJob, workflowSteps, k, visited, mutex, results)));
	return results;
}


async function isVisited(visited, check, mutex) {
	return await mutex.runExclusive(() => {
		let output = false
		if (visited[check]) output = true;
		else visited[check] = true;
		return output;
	});
}

/**
 * Simulate a step and determine the time it takes to run
 * @param {PrintJob} printJob 
 * @param {[WorkflowStep]} workflowSteps 
 * @param {string} step (The name of the workflow step)
 * @returns {int} the time taken for the step
 */
async function simulateStep(printJob, workflowSteps, step) {
	if(!workflowSteps[step]){
		throw new Error("simulateStep: Step not found");
	}

	// TODO: in the future, steps will have different functions
	// to simulate how long they take
	const funcs = {
		"Preflight": placeholder,
		"Metrics": placeholder,
		"Rasterization": placeholder,
		"Printing": placeholder,
		"Cutting": placeholder,
		"Laminating": placeholder,
	}

	// Testing steps do not exist in funcs, so they will just
	// use the simple calculation
	if (typeof funcs[workflowSteps[step].func] === 'function') {
		return await funcs[workflowSteps[step].func](workflowSteps[step], printJob);
	} else {
		return workflowSteps[step].time * printJob.PageCount;
	}
}

/** 
 * A placeholder function to be replaced with specific step functions
 * in the future
 * @param {WorkflowStep} workflowStep
 * @param {PrintJob} printJob
 * @returns
 */
async function placeholder(workflowStep, printJob) {
	return workflowStep.time * printJob.PageCount;
}


if (require.main === module) { main(); }

module.exports = { simulate };