const { test, before, after } = require('node:test');
const assert = require('node:assert');
const { fastify, start,  } = require('../../server.js');
let testWorkflowId="blah", testPrintJobId="blah";

before(async () => {
    await start("0.0.0.0", 8080, "mongodb://localhost:27017/hp");    
});
after(() => {
    fastify.close();
    process.exit(0);
}
);

test('GET /', async (t) => {
    const response = await fastify.inject({
        method: 'GET',
        url: '/'
    });
    assert.strictEqual(response.statusCode, 200);
    assert.strictEqual(response.payload, 'Hello, client!');
});

test('POST /createJob', async (t) => {
    const response = await fastify.inject({
        method: 'POST',
        url: '/createJob',
        body: {
            Title: 'Test Job',
            PageCount: 10,
            RasterizationProfile: 'Black'
        }
    });
    assert.strictEqual(response.statusCode, 200);
    assert.strictEqual(response.payload, 'Operation successful\n');
});

test('POST /createWorkflow', async (t) => {
    const response = await fastify.inject({
        method: 'POST',
        url: '/createWorkflow',
        body: {
            Title: 'Test Workflow',
            WorkflowSteps: ['Step1', 'Step2']
        }
    });
    assert.strictEqual(response.statusCode, 200);
    assert.strictEqual(response.payload, 'Operation successful\n');
});

test('POST /createWorkflowStep', async (t) => {
    const response = await fastify.inject({
        method: 'POST',
        url: '/createWorkflowStep',
        body: {
            Title: 'Test Step',
            PreviousStep: 'Step1',
            NextStep: 'Step2',
            SetupTime: 5,
            TimePerPage: 2
        }
    });
    assert.strictEqual(response.statusCode, 200);
    assert.strictEqual(response.payload, 'Operation successful\n');
});

test('GET /query', async (t) => {
    const CollectionName = encodeURIComponent('PrintJob');
    const Query = encodeURIComponent(JSON.stringify({Title: "PrintJob 1"}))
    const response = await fastify.inject({
        method: 'GET',
        url:`/query?CollectionName=${CollectionName}&Query=${Query}`
    });
    assert.strictEqual(response.statusCode, 200);
    console.log("Query Results: ", JSON.parse(response.payload));
});

test('GET /getPrintJob', async (t) => {
    const Title = encodeURIComponent("PrintJob 1");
    const response = await fastify.inject({
        method: 'GET',
        url:`/getPrintJob?Title=${Title}`
    });
    const payload = JSON.parse(response.payload);
    assert.strictEqual(response.statusCode, 200);
    assert(Array.isArray(payload));
    console.log("getPrintJob Results: ", JSON.parse(response.payload));
});

test('GET /getWorkflowList', async (t) => {
    const response = await fastify.inject({
        method: 'GET',
        url: '/getWorkflowList'
    });
    assert.strictEqual(response.statusCode, 200);
    const payloadList = response.payload;
    if (!payloadList) console.error("payload is null");
    else console.log("All workflows: ", payloadList);
});

/*

TODO: Make this word with IDs instead

test('GET /getSimulationReport', async (t) => {
    const title = 'PrintJob 1';
    const workflow = 'Workflow 1';
    const response = await fastify.inject({
        method: 'GET',
        url: `/getSimulationReport?title=${encodeURIComponent(title)}&workflow=${encodeURIComponent(workflow)}`
    });
    assert.strictEqual(response.statusCode, 200);
    const payload = JSON.parse(response.payload);
    console.log("Simulation Report:", payload);
});
*/

test('GET /getSimulationReportList', async (t) => {
    const response = await fastify.inject({
        method: 'GET',
        url: '/getSimulationReportList'
    });
    assert.strictEqual(response.statusCode, 200);
    const payloadList = JSON.parse(response.payload);
    if (!payloadList) console.error("payload is null");
    else console.log("All simulation reports: ", payloadList);

    // Predefined globals for testing..
    testPrintJobId = payloadList[0].PrintJobID;
    testWorkflowId = payloadList[0].WorkflowID;
});

test('GET /generateSimulationReport', async (t) => {
    const response = await fastify.inject({
        method: 'GET',
        url: '/generateSimulationReport',
        query: {
            jobID: testPrintJobId,
            workflowID: testWorkflowId
        }
    });
    if (response.statusCode !== 200) {
        console.log("Error: ", response.payload);
    }
    assert.strictEqual(response.statusCode, 200);
    let simulationReport = await JSON.parse(response.payload);
    assert.strictEqual(simulationReport.PrintJobID, testPrintJobId);
    assert.strictEqual(simulationReport.WorkflowID, testWorkflowId);
    console.log("Generated simulation report: ", simulationReport);
});

test('GET /getWorkflowStepList', async (t) => {
    const response = await fastify.inject({
        method: 'GET',
        url: '/getWorkflowStepList'
    });
    assert.strictEqual(response.statusCode, 200);
    const payloadList = response.payload;
    if (!payloadList) console.error("payload is null");
    else console.log("All workflow steps: ", payloadList);
});