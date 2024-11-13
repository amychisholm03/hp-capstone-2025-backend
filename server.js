const { dbConnect, dbSetup, newPrintJob, newWorkflow, newWorkflowStep } = require("./mongodb.js");
const fastify = require('fastify')({ logger: true })
const cors = require('@fastify/cors');
const { simulate } = require('./simulation.js');

// TODO: Where should we store this?
const mongoUrl = "mongodb://db.wsuv-hp-capstone.com:27017/hp"; 

/**
 * Starts up the fastify server and connects to the Mongo database.
 * @param {string} host The host address
 * @param {number} port The port number
 */
async function start(host = "0.0.0.0", port = 80){
  try {
    // Register the fastify-cors plugin
    fastify.register(cors, {
      origin: '*', // Allow all origins
      methods: ['GET', 'POST'], // Allow specific methods
    });

    // Connect to the Mongo database
    const [_, database] = await dbConnect(mongoUrl);
    await dbSetup(database); // TODO: get rid of once in mongodb.test.js?
    setupPosts(database);

    // Start the server
    setupGets(database);
    fastify.listen({ host: host, port: port }, (err, address) => {
      if (err) {
        console.error(err.message);
        process.exit(1);
      }
      console.log(`Server listening at ${address}`);
    });
  } catch (err) {
    console.error(err.message);
    process.exit(1);
  }
}

/**
 * Sets up the GETs for the server
 * @param {Db} database
 */
function setupGets(database) {
  fastify.get('/', async (_, reply) => {
    reply.code(200).send('Hello, client!');
  });

  /**
   * Get an existing simulation report from a 
   * PrintJob id and Workflow id
   */
  fastify.get('/getSimulationReport', async (request, reply) => {
    const {jobID, workflowID} = request.query;
    const printJob = await database.collection('PrintJob').findOne({_id: jobID});
    if (!printJob) {
      reply.code(404).send("PrintJob not found");
      return;
    }
    const workflow = await database.collection('Workflow').findOne({_id: workflowID});
    if (!workflow) {
      reply.code(404).send("WorkflowDoc not found");
      return;
    }
    const simulationReport = await database.collection('SimulationReport').findOne({PrintJobID: printJob._id, WorkflowID: workflow._id});
    if (!simulationReport) reply.code(404).send("Simulation report not found");
    else reply.code(200).send({PrintJob: printJob, SimulationReport: simulationReport});
  });

  /**
   * Generate a simulation report (see simulation.js)
   * for a given PrintJob id and Workflow id
   */
  fastify.get('/generateSimulationReport', async (request, reply) => {
    const {jobID, workflowID} = request.query;

    const printJob = await database.collection('PrintJob').findOne({_id: jobID});
    if (!printJob) {
      reply.code(404).send("PrintJob not found");
      return;
    }

    const workflow = await database.collection('Workflow').findOne({_id: workflowID});
    if (!workflow) {
      reply.code(404).send("Workflow not found");
      return;
    }

    const simulationReport = await simulate(printJob, workflow);
    reply.code(200).send(simulationReport);
  });

  fastify.get('/getWorkflowList', async (request, reply) => {
    const workflowDocs = await database.collection('Workflow').find();
    if (!workflowDocs) {
      reply.code(404).send("WorkflowDocs not found");
      return;
    }
    const workflowList = [];
    for await (const doc of workflowDocs) {
      workflowList.push({WorkflowID: doc._id, Title: doc.Title});
    }
    reply.code(200).send(workflowList);
  });

  fastify.get('/getWorkflowStepList', async (request, reply) => {
    try {
      const steps = await database.collection('WorkflowStep').find({}).toArray();
      reply.code(200).send(steps);
    } catch (err) {
      reply.code(500).send({error: err.message});
    }
  });
}

/**
 * Sets up the POSTs for the server
 * @param {Db} database 
 */
function setupPosts(database) {
  fastify.post('/createJob', async (request, reply) => {
    await fastifyPostHelper(reply, database, newPrintJob,
      [request.body.Title, request.body.PageCount, request.body.RasterizationProfile]);
  });

  fastify.post('/createWorkflow', async (request, reply) => {
    await fastifyPostHelper(reply, database, newWorkflow,
      [request.body.Title, request.body.WorkflowSteps]);
  });

  fastify.post('/createWorkflowStep', async (request, reply) => {
    await fastifyPostHelper(reply, database, newWorkflowStep,
      [request.body.Title, request.body.PreviousStep, request.body.NextStep, request.body.SetupTime, request.body.TimePerPage]);
  });

  fastify.post('/query', async (request, reply) => {
    // TODO: could the helper function be modified to support this?
    // TODO: reformat to a get instead of post?
    let message = ""
    let code = 200;
    const collection = database.collection(request.body.CollectionName);
    try { message = await collection.find(request.body.Query).toArray(); }
    catch (err) { message = err; code = 500; }
    reply.code(code).send(message);
  });
}

async function fastifyPostHelper(reply, database, func, args) {
  let message = "Operation successful\n";
  let code = 200;
  try { await func(database, ...args); }
  catch (err) { message = err; code = 500; }
  finally { reply.code(code).send(message); }
}

function main(){
  start();
}


// This is needed so that server.test.js doesn't run main()
if (require.main === module) {
  main();
}


module.exports = { fastify, start };
