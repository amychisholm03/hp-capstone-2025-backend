const { Mutex } = require('async-mutex');
const { dbConnect, dbSetup, newSimulationReport } = require("./mongodb.js"); //TODO: Remove


async function main() {
	const [_, database] = await dbConnect("mongodb://localhost:27017/hp");
	await dbSetup(database);
	const workflow = await database.collection("Workflow").find().toArray();
	const printJob = await database.collection("PrintJob").findOne();
	simulate(printJob, workflow[0], database);
}

/**
 * Simulate a PrintJob through the given Workflow
 * @param {dictionary} printJob 
 * @param {dictionary} workflow 
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

	const results = await traverseGraph(printJob, workflowSteps,
		Object.keys(workflowSteps)[0],
		Object.fromEntries(Object.keys(workflowSteps).map((k) => [k, false])),
	);

	// Find times for report
	// TODO: This could be done in a single loop
	let rastTime;
	const totalTime = await Math.max(...Object.keys(results).map((k) => {
		if (results[k].stepName === "Rasterization") rastTime = results[k].stepTime;
		return results[k].cumulative;
	}));

	return await newSimulationReport(database, printJob._id, workflow._id, totalTime, rastTime);
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


async function simulateStep(printJob, workflowSteps, step) {
	// await new Promise(resolve => setTimeout(resolve, Math.random()*1000)); //TODO: Remove
	const funcs = {
		"Preflight": placeholder,
		"Metrics": placeholder,
		"Rasterization": placeholder,
		"Printing": placeholder,
		"Cutting": placeholder,
		"Laminating": placeholder,
	}
	return await funcs[workflowSteps[step].func](workflowSteps[step], printJob)
}


async function placeholder(workflowStep, printJob) {
	return workflowStep.time * printJob.PageCount;
}


if (require.main === module) { main(); }

module.exports = { simulate };