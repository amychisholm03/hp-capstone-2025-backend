const { dbConnect, dbSetup, newPrintJob, newWorkflow, newWorkflowStep } = require("./mongodb.js");

const fastify = require('fastify')({ logger: true })
const cors = require('@fastify/cors');
const { simulate } = require('./simulation.js');
const { ObjectId } = require("mongodb");

// TODO: Where should we store this?
const mongoUrl = "mongodb://db.wsuv-hp-capstone.com:27017/hp";

/**
 * Starts up the fastify server and connects to the Mongo database.
 * @param {string} host The host address
 * @param {number} port The port number
 * @param {string} url The Mongo database URL
 */
async function start(host = "0.0.0.0", port = 80, url = mongoUrl) {
  try {
    // Register the fastify-cors plugin
    fastify.register(cors, {
      origin: '*', // Allow all origins
      methods: ['GET', 'POST'], // Allow specific methods
    });

    // Connect to the Mongo database
    const [_, database] = await dbConnect(url);
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
   * Preform a query on a given collection
   */
  fastify.get('/query', async (request, reply) => {
    try {
      const Query = JSON.parse(request.query.Query);
      const Collection = database.collection(request.query.CollectionName);
      const result = await Collection.find(Query).toArray();
      reply.code(200).send(result);
    }
    catch (err) {
      reply.code(500).send(err.message);
    }
  });


  /**
   * Get an existing print job from a title
   */
  fastify.get('/getPrintJob', async (request, reply) => {
    try {
      const Collection = database.collection("PrintJob");
      const result = await Collection.find({ "Title": request.query.Title }).toArray();
      reply.code(200).send(result);
    }
    catch (err) {
      reply.code(404).send(err.message);
    }
  });


  /**
  * Get an existing simulation report from a 
  * PrintJob id and Workflow id
  */
  fastify.get('/getSimulationReport', async (request, reply) => {
    const { jobID, workflowID } = request.query;
    const printJob = await database.collection('PrintJob').findOne({ _id: new ObjectId(jobID) });
    if (!printJob) {
      reply.code(404).send("PrintJob not found");
      return;
    }
    const workflow = await database.collection('Workflow').findOne({ _id: new ObjectId(workflowID) });
    if (!workflow) {
      reply.code(404).send("Workflow not found");
      return;
    }
    const simulationReport = await database.collection('SimulationReport').findOne({ PrintJobID: new ObjectId(printJob._id), WorkflowID: new ObjectId(workflow._id) });
    if (!simulationReport) {
      reply.code(404).send("Simulation report not found");
    }
    else {
      reply.code(200).send({ PrintJob: printJob, SimulationReport: simulationReport });
    }
  });


  /**
   * Generate a simulation report (see simulation.js)
   * for a given PrintJob id and Workflow id
   */
  fastify.get('/generateSimulationReport', async (request, reply) => {
    //TODO: Change to a POST
    const { jobID, workflowID } = request.query;

    const printJob = await database.collection('PrintJob').findOne({ _id: new ObjectId(jobID) });
    if (!printJob) {
      reply.code(404).send("PrintJob not found");
      return;
    }

    const workflow = await database.collection('Workflow').findOne({ _id: new ObjectId(workflowID) });
    if (!workflow) {
      reply.code(404).send("Workflow not found");
      return;
    }

    try {
      const simulationReportId = await simulate(printJob, workflow, database);
      const result = await database.collection('SimulationReport').findOne(
        { _id: simulationReportId });
      reply.code(200).send(result);
    }
    catch (err) {
      reply.code(500).send(err.message);
    }
  });


  /**
   * Get a list of all simulation reports
   */
  fastify.get('/getSimulationReportList', async (request, reply) => {
    try {
      const simulationReports = await database.collection('SimulationReport').find();
      if (!simulationReports) {
        reply.code(404).send("No SimulationReports found");
        return;
      }

      const reportList = [];
      for await (const report of simulationReports) {
        report.PrintJobTitle = '';
        report.WorkflowTitle = '';

        const titles = await Promise.all([
          database.collection('PrintJob').findOne({ _id: new ObjectId(report.PrintJobID) }),
          database.collection('Workflow').findOne({ _id: new ObjectId(report.WorkflowID) }),
        ]);

        const printJob = titles[0];
        if (printJob) {
          report.PrintJobTitle = printJob.Title;
        }

        const workflow = titles[1];
        if (workflow) {
          report.WorkflowTitle = workflow.Title;
        }

        reportList.push(report);
      }
      reply.code(200).send(reportList);
    }
    catch (err) {
      reply.code(500).send(err.message);
    }
  });


  /**
   * Get a list of all workflows
   */
  fastify.get('/getWorkflowList', async (request, reply) => {
    const workflows = await database.collection('Workflow').find();
    if (!workflows) {
      reply.code(404).send("No workflows found");
      return;
    }
    const workflowList = [];
    for await (const w of workflows) {
      workflowList.push({ WorkflowID: w._id, Title: w.Title });
    }
    reply.code(200).send(workflowList);
  });


  /**
   * Get a list of all workflow steps
   */
  fastify.get('/getWorkflowStepList', async (request, reply) => {
    try {
      const steps = await database.collection('WorkflowStep').find({}).toArray();
      reply.code(200).send(steps);
    } catch (err) {
      reply.code(500).send(err.message);
    }
  });
}


/**
 * Sets up the POSTs for the server
 * @param {Db} database 
 */
function setupPosts(database) {
  /**
   * Create a new print job
   */
  fastify.post('/createJob', async (request, reply) => {
    try {
      const result = await newPrintJob(database, request.body.Title, request.body.PageCount, request.body.RasterizationProfile);
      reply.code(200).send(result);
    }
    catch (err) {
      reply.code(500).send(err.message);
    }
  });


  /**
   * Create a new workflow
   */
  fastify.post('/createWorkflow', async (request, reply) => {
    try {
      // Map each WorkflowStep to a ObjectID
      let workflowSteps = [];
      try {
        workflowSteps = request.body.WorkflowSteps.map(stepID => new ObjectId(stepID));
      }
      catch (err) {
        reply.code(500).send("Could not map step IDs to ObjectIDs: ", err.message);
        return;
      }
      if (workflowSteps.length == 0) {
        reply.code(500).send("No valid WorkflowSteps provided");
        return;
      }
      // Create a new workflow
      const result = await newWorkflow(database, request.body.Title, workflowSteps);
      reply.code(200).send(result);
    }
    catch (err) {
      reply.code(500).send(err.message);
    }
  });


  /**
   * Create a new workflow step
   */
  fastify.post('/createWorkflowStep', async (request, reply) => {
    try {
      const result = await newWorkflowStep(database, request.body.Title, request.body.PreviousStep, request.body.NextStep, request.body.SetupTime, request.body.TimePerPage);
      reply.code(200).send(result);
    }
    catch (err) {
      reply.code(500).send(err.message);
    }
  });
}

// This allows passing in an alternate port as a command line argument
if (require.main === module) {
  if (process.argv.length > 3 && process.argv[3] == "-l") {
    start("0.0.0.0", process.argv[2], "mongodb://localhost:27017/hp");
  }
  else start();
}

module.exports = { fastify, start };