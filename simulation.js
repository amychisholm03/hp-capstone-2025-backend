const { Mutex } = require('async-mutex');
const random = Math.random(); //TODO: Remove
const { dbConnect, dbSetup, newPrintJob, newWorkflow, newWorkflowStep } = require("./mongodb.js"); //TODO: Remove


async function main(){

	const [_, database] = await dbConnect("mongodb://localhost:27017/hp");
    await dbSetup(database);
    workflow = await database.collection("Workflow").find().toArray();
    console.log(workflow[0]);
	simulate(0, workflow[0], database);
}


/**
 * Simulate a PrintJob through the given Workflow
 * @param {*} printJob 
 * @param {*} workflow 
 * @returns {*} the SimulationReport as a JSON 
 */

async function simulate(printJob, workflow, database){
	workflowSteps = {}
	temp = workflow.WorkflowSteps;
	for(i = 0; i < temp.length; i++){
		workflowSteps[temp[i]] = {};
		// console.log()
		// console.log(await database.collection("WorkflowSteps").findOne({_id: temp[i]}));
		workflowSteps[temp[i]].func = "Preflight";
		workflowSteps[temp[i]].prev = i == 0 ? [] : [temp[i-1]];
		workflowSteps[temp[i]].next = i == temp.length-1 ? [] : [temp[i+1]];
	}
	console.log(workflowSteps);

	await traverseGraph(printJob, workflowSteps, 
		Object.keys(workflowSteps)[0], 
		Object.fromEntries(Object.keys(workflowSteps).map((k) => [k, false])), 	
	);
}


async function traverseGraph(printJob, workflowSteps, step, visited, mutex=new Mutex(), results={}){
	if(await isVisited(visited, step, mutex)) return;
	await Promise.all(workflowSteps[step].prev.map((k) => 
		traverseGraph(printJob, workflowSteps, k, visited, mutex, results)));
	results[step] = simulateStep(workflowSteps, step);
	await Promise.all(workflowSteps[step].next.map((k) => 
		traverseGraph(printJob, workflowSteps, k, visited, mutex, results)));
	return results
}


async function isVisited(visited, check, mutex){
	return await mutex.runExclusive(() => {
		output = false
        if (visited[check]) output = true;
        else visited[check] = true;
        return output;
    });
}


async function simulateStep(workflowStep, step){
	await new Promise(resolve => setTimeout(resolve, Math.random()*1000)); //TODO: Remove
	const funcs = {
		"Preflight": preflight,
		"Metrics": metrics,
		"Rasterization": rasterization,
		"Printing": printing,
		"Cutting": cutting,
		"Laminating": laminating,
	}
	await funcs[workflowStep[step].func]();
	console.log(step, ": Done");
}


async function preflight(){
	console.log("preflight");
}


async function metrics(){
	console.log("metrics");
}


async function rasterization(){
	console.log("rasterization");
}


async function printing(){
	console.log("printing");
}


async function cutting(){
	console.log("cutting");
}


async function laminating(){
	console.log("laminating");
}


if (require.main === module){ main(); }

module.exports = { simulate };