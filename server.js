const { dbConnect, dbSetup, newPrintJob, newWorkflow, newWorkflowStep } = require("./mongodb.js");
const fastify = require('fastify')({ logger: true })
const cors = require('@fastify/cors');

// TODO: Where should we store this?
const mongoUrl = "mongodb://localhost:27017/hp"; 


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


  fastify.get('/query', async (request, reply) => {
    let message = "";
    let code = 200;
    const Query = JSON.parse(request.query.Query);
    const Collection = database.collection(request.query.CollectionName);
    try { message = await Collection.find(Query).toArray(); }
    catch(err){ message = err; code = 500; }
    reply.code(code).send(message);
  });


  fastify.get('/getPrintJob', async (request, reply) => {
    let message = "";
    let code = 200;
    const Collection = database.collection("PrintJob");
    try { message = await Collection.find({"Title": request.query.Title}).toArray(); }
    catch(err){ message = err; code = 500; }
    reply.code(code).send(message);
  });


  fastify.get('/getSimulationReport', async (request, reply) => {
    const {title, workflow} = request.query;
    const printJob = await database.collection('PrintJob').findOne({Title: title});
    if (!printJob) {
      reply.code(404).send("PrintJob not found");
      return;
    }
    const workflowDoc = await database.collection('Workflow').findOne({Title: workflow});
    if (!workflowDoc) {
      reply.code(404).send("WorkflowDoc not found");
      return;
    }
    const simulationReport = await database.collection('SimulationReport').findOne({PrintJobID: printJob._id, WorkflowID: workflowDoc._id});
    if (!simulationReport) reply.code(404).send("Simulation report not found");
    else reply.code(200).send({PrintJob: printJob, SimulationReport: simulationReport});

    
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
}


async function fastifyPostHelper(reply, database, func, args) {
  let message = "Operation successful\n";
  let code = 200;
  try { await func(database, ...args); }
  catch (err) { message = err; code = 500; }
  finally { reply.code(code).send(message); }
}


// This allows passing in an alternate port as a command line argument
if (require.main === module) {
  if(process.argv.length > 2) start("0.0.0.0", process.argv[2]);
  else start();
}


module.exports = { fastify, start };