const { Mutex } = require('async-mutex');
const random = Math.random(); //TODO: Remove
const { dbConnect, dbSetup, newPrintJob, newWorkflow, newWorkflowStep } = require("./mongodb.js"); //TODO: Remove


async function main(){

	const [_, database] = await dbConnect("mongodb://localhost:27017/hp");
    await dbSetup(database);
    workflow = await database.collection("Workflow").find().toArray();
    printJob = await database.collection("PrintJob").findOne();
    console.log(printJob);
    console.log(workflow[0]);
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

	await traverseGraph(printJob, workflowSteps, 
		Object.keys(workflowSteps)[0], 
		Object.fromEntries(Object.keys(workflowSteps).map((k) => [k, false])), 	
	);
}


async function traverseGraph(printJob, workflowSteps, step, visited, mutex=new Mutex(), results={}){
	if(await isVisited(visited, step, mutex)) return;
	await Promise.all(workflowSteps[step].prev.map((k) => 
		traverseGraph(printJob, workflowSteps, k, visited, mutex, results)));
	results[step] = {stepTime: await simulateStep(printJob, workflowSteps, step)};
	results[step].cumulative = results[step].stepTime;
	for (const item of workflowSteps[step].prev) {
		console.log(`previous step: ${item}`);
	}
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
	await new Promise(resolve => setTimeout(resolve, Math.random()*1000)); //TODO: Remove
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
	console.log("preflight");
	return 0.05 * printJob.PageCount;
}


async function metrics(printJob){
	console.log("metrics");
	return 0.01 * printJob.PageCount;
}


async function rasterization(printJob){
	console.log("rasterization");
	return 0.1 * printJob.PageCount;
}


async function printing(printJob){
	console.log("printing");
	return 0.5 * printJob.PageCount;
}


async function cutting(printJob){
	console.log("cutting");
	return 0.2 * printJob.PageCount;
}


async function laminating(printJob){
	console.log("laminating");
	return 0.3 * printJob.PageCount;
}


if (require.main === module){ main(); }

module.exports = { simulate };