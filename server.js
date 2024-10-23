const { dbConnect, dbSetup, newPrintJob, newWorkflow, newWorkflowStep } = require("./mongodb.js");
const fastify = require('fastify')({ logger: true })
const cors = require('@fastify/cors');

// TODO: Where should we store these constants?
const port = 80;
const host = "0.0.0.0";
const mongoUrl = "mongodb://localhost:27017/HP"; // TODO: better place for this

/**
 * Starts up the fastify server.
 */
async function start() {
  // Register the fastify-cors plugin
  fastify.register(cors, {
    origin: '*', // Allow all origins
    methods: ['GET', 'POST'], // Allow specific methods
  });

  // Start the server
  setupGets();
  fastify.listen({ host: host, port: port }, (err, address) => {
    if (err) {
      console.error(err);
      process.exit(1);
    }
    console.log(`Server listening at ${address}`);
  });
}

/**
 * Connects to the Mongo database.
 */
async function connectToDB() {
  try {
    const [_, database] = await dbConnect(mongoUrl);
    await dbSetup(database); // TODO: get rid of once in mongodb.test.js?
    setupPosts(database);
  }
  catch (err) {
    console.log(err);
    process.exit(1);
  }
}

/**
 * Sets up the GETs for the server
 */
function setupGets() {
  fastify.get('/', async (_, reply) => {
    reply.code(200).send('Hello, client!');
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

function main() {
  start().then(() =>
    connectToDB()
  );
}

// This is needed so that server.test.js doesn't run main()
if (require.main === module) {
  main();
}

module.exports = { fastify, start, connectToDB };