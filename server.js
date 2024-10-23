const fastify = require('fastify')({ logger: true })
const { dbConnect, dbSetup, newPrintJob, newWorkflow, newWorkflowStep } = require("./mongodb.js");

// TODO: Where should we store these constants?
const port = 80;
const mongoUri = "mongodb://localhost:27017/HP";

/**
 * Starts up the fastify server
 */
async function start() {
  try {
    const [_, database] = await dbConnect(mongoUri);
    await dbSetup(database); // TODO: get rid of once in mongodb.test.js

    setupGets();
    setupPosts(database);

    fastify.listen({ port: port, host: '0.0.0.0' });

  } catch (err) {
    fastify.log.error(err);
    process.exit(1);
  }
}

function setupGets() {
  fastify.get('/', function handler(_, reply) {
    reply.code(200).send('Hello, client!');
  });
}

function setupPosts(database) {
  // TODO: Test how mongoDB handles multiple simultaneous requests
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
  reply.code(code).send(message);
}

start();