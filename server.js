// Require the framework and instantiate it
const fastify = require('fastify')({ logger: true })
const { db_start, db_connect, db_setup, new_print_job, new_workflow, new_workflow_step, new_simulation_report } = require("./mongodb.js");

const port_num = 80; //Default: 80


async function start(){ 
  try { 
    await db_start();
    
    setup_gets();
    setup_posts();
    
    fastify.listen({ port: port_num, host: '0.0.0.0' }, (err, address) => {
      if (err) throw err;
    });

  } catch (err){
    fastify.log.error(err);
    process.exit(1);
  }
}


function setup_gets(){
  fastify.get('/', function handler (request, reply) {
    reply.code(200).send('hello client!');
  });
}


function setup_posts(){
  fastify.post('/createJob', async (request, reply) => {
    await fastify_post_create(reply, new_print_job, 
      [request.body.Title, request.body.PageCount, request.body.RasterizationProfile]);
  });

  fastify.post('/createWorkflow', async (request, reply) => {
    await fastify_post_create(reply, new_workflow, 
      [request.body.Title, request.body.WorkflowSteps]);
  });

  fastify.post('/createWorkflowStep', async (request, reply) => {
    await fastify_post_create(reply, new_workflow_step, 
      [request.body.Title, request.body.PreviousStep, request.body.NextStep, request.body.SetupTime, request.body.TimePerPage]);
  });
}


async function fastify_post_create(reply, func, args){
  message = "Operation successful\n";
  code = 200;
  try {
    [client, database] = await db_connect();
    await func(database, ...(Array.isArray(args)?args:[args]));
  } catch(err){
    message = err; 
    code = 500;
  } finally {
    await client.close();
  }
  reply.code(code).send(message);
}


start();