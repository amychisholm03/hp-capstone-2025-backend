const { Mutex } = require('async-mutex');
const random = Math.random(); //TODO: Remove
const { dbConnect, dbSetup, newPrintJob, newWorkflow, newWorkflowStep } = require("./mongodb.js"); //TODO: Remove


async function main(){

	const [_, database] = await dbConnect("mongodb://localhost:27017/hp");
    await dbSetup(database);
    workflow = await database.collection("Workflow").find().toArray();
    printJob = await database.collection("PrintJob").findOne();
	simulate(printJob, workflow[0], database);
}


/**
 * Simulate a PrintJob through the given Workflow
 * @param {*} printJob 
 * @param {*} workflow 
 * @returns {*} the SimulationReport as a JSON 
 */

async function simulate(printJob, workflow, database){
	//Format workflowSteps correctly for traverseGraph
	workflowSteps = {}
	temp = workflow.WorkflowSteps;
	for(i = 0; i < temp.length; i++){
		workflowSteps[temp[i]] = {
			func: (await database.collection("WorkflowStep").findOne({_id: temp[i]})).Title,
			prev: i == 0 ? [] : [temp[i-1]],
			next: i == temp.length-1 ? [] : [temp[i+1]]
		};
	}

	results = await traverseGraph(printJob, workflowSteps, 
		Object.keys(workflowSteps)[0], 
		Object.fromEntries(Object.keys(workflowSteps).map((k) => [k, false])), 	
	);

	totalTime = await Math.max(...Object.keys(results).map((k) => 
		results[k].cumulative));

	rastTime = await results[Object.keys(results).filter((k) => 
		results[k].stepName === "Rasterization")].stepTime;

	simulatedPrintJob = {
		PrintJobID: printJob._id,
		WorkflowID: workflow._id,
		TotalTimeTaken: totalTime,
		RasterizationTimeTaken: rastTime
	};

	inserted = await database.collection("SimulationReport").insertOne(simulatedPrintJob);
	if (inserted.acknowledged === false) {
		console.log("Insertion failed!");
		return null;
	}

	console.log("Simulation completed and stored");

	return inserted.insertedId;
}


async function traverseGraph(printJob, workflowSteps, step, visited, mutex=new Mutex(), results={}){
	if(await isVisited(visited, step, mutex)) return;
	await Promise.all(workflowSteps[step].prev.map((k) => 
		traverseGraph(printJob, workflowSteps, k, visited, mutex, results)));
	simulatedTime =  await simulateStep(printJob, workflowSteps, step);
	results[step] = {stepName: workflowSteps[step].func, stepTime: simulatedTime, cumulative: simulatedTime};
	results[step].cumulative += await Math.max(workflowSteps[step].prev.map((k) =>
		results[k].cumulative));
	await Promise.all(workflowSteps[step].next.map((k) => 
		traverseGraph(printJob, workflowSteps, k, visited, mutex, results)));
	return results;
}


async function isVisited(visited, check, mutex){
	return await mutex.runExclusive(() => {
		output = false
        if (visited[check]) output = true;
        else visited[check] = true;
        return output;
    });
}


async function simulateStep(printJob, workflowSteps, step){
	// await new Promise(resolve => setTimeout(resolve, Math.random()*1000)); //TODO: Remove
	const funcs = {
		"Preflight": preflight,
		"Metrics": metrics,
		"Rasterization": rasterization,
		"Printing": printing,
		"Cutting": cutting,
		"Laminating": laminating,
	}
	return await funcs[workflowSteps[step].func](printJob)
}


async function preflight(printJob){
	// console.log("step: preflight");
	return 0.05 * printJob.PageCount;
}


async function metrics(printJob){
	// console.log("step: metrics");
	return 0.01 * printJob.PageCount;
}


async function rasterization(printJob){
	// console.log("step: rasterization");
	return 0.1 * printJob.PageCount;
}


async function printing(printJob){
	// console.log("step: printing");
	return 0.5 * printJob.PageCount;
}


async function cutting(printJob){
	// console.log("step: cutting");
	return 0.2 * printJob.PageCount;
}


async function laminating(printJob){
	// console.log("step: laminating");
	return 0.3 * printJob.PageCount;
}


if (require.main === module){ main(); }

module.exports = { simulate };