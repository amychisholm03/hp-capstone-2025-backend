const fastify = require('fastify')({ logger: true })
const { db_connect, db_setup, new_print_job, new_workflow, new_workflow_step, new_simulation_report } = require("./mongodb.js");

//TODO: can this value be tracked somwhere else so we don't have to remember to change it before merging?
const port_num = 80; //Default: 80


async function start(){ 
  try { 
    [client, database] = await db_connect();
    await db_setup(database);
    
    setup_gets(database);
    setup_posts(database);
    
    fastify.listen({ port: port_num, host: '0.0.0.0' }, (err, address) => {
      if (err) throw err;
    });

  } catch (err){
    fastify.log.error(err);
    process.exit(1);
  }
}


function setup_gets(database){
  fastify.get('/', function handler (request, reply) {
    reply.code(200).send('hello client!');
  });
}


function setup_posts(database){
  //TODO: Test how mongoDB handles multiple simultaneous requests
  fastify.post('/createJob', async (request, reply) => {
    await fastify_post_helper(reply, database, new_print_job, 
      [request.body.Title, request.body.PageCount, request.body.RasterizationProfile]);
  });

  fastify.post('/createWorkflow', async (request, reply) => {
    await fastify_post_helper(reply, database, new_workflow, 
      [request.body.Title, request.body.WorkflowSteps]);
  });

  fastify.post('/createWorkflowStep', async (request, reply) => {
    await fastify_post_helper(reply, database, new_workflow_step, 
      [request.body.Title, request.body.PreviousStep, request.body.NextStep, request.body.SetupTime, request.body.TimePerPage]);
  });

  //TODO: could the helper function be modified to support this?
  //TODO: reformat to a get instead of post?
  fastify.post('/query', async (request, reply) => {
    message = ""
    code = 200;
    collection = database.collection(request.body.CollectionName);
    try { message = await collection.find(request.body.Query).toArray(); }
    catch(err) {message = err; code = 500; }
    reply.code(code).send(message);
  });
}


async function fastify_post_helper(reply, database, func, args){
  message = "Operation successful\n";
  code = 200;
  try { await func(database, ...args); }
  catch(err) { message = err; code = 500; }
  reply.code(code).send(message);
}


start();