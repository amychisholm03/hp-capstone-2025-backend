const { test, before, after } = require('node:test');
const assert = require('node:assert');
const { fastify, start, } = require('../../server.js');

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
    const Query = encodeURIComponent(JSON.stringify({ Title: "PrintJob 1" }))
    const response = await fastify.inject({
        method: 'GET',
        url: `/query?CollectionName=${CollectionName}&Query=${Query}`
    });
    assert.strictEqual(response.statusCode, 200);
    console.log("Query Results: ", JSON.parse(response.payload));
});

test('GET /getPrintJob', async (t) => {
    const Title = encodeURIComponent("PrintJob 1");
    const response = await fastify.inject({
        method: 'GET',
        url: `/getPrintJob?Title=${Title}`
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

test('GET /getSimulationReport', async (t) => {
    // Get an existing simulation report
    let response = await fastify.inject({
        method: 'GET',
        url: '/getSimulationReportList'
    });
    assert.strictEqual(response.statusCode, 200);
    const payloadList = JSON.parse(response.payload);
    assert.ok(payloadList);
    const testPrintJobId = payloadList[0].PrintJobID;
    const testWorkflowId = payloadList[0].WorkflowID;

    // Make sure that the simulation report 
    // can be retrieved
    response = await fastify.inject({
        method: 'GET',
        url: `/getSimulationReport?jobID=${encodeURIComponent(testPrintJobId)}&workflowID=${encodeURIComponent(testWorkflowId)}`
    });
    assert.strictEqual(response.statusCode, 200);
    const payload = JSON.parse(response.payload);
    assert.ok(payload);
    console.log("Simulation Report: ", payload);
});

test('GET /getSimulationReportList', async (t) => {
    const response = await fastify.inject({
        method: 'GET',
        url: '/getSimulationReportList'
    });
    assert.strictEqual(response.statusCode, 200);
    const payloadList = JSON.parse(response.payload);
    assert.ok(payloadList);
    assert.notEqual(payloadList.length, 0);
    console.log("All simulation reports: ", payloadList);
});

test('GET /generateSimulationReport', async (t) => {
    // Get an existing simulation report
    let response = await fastify.inject({
        method: 'GET',
        url: '/getSimulationReportList'
    });
    assert.strictEqual(response.statusCode, 200);
    const payloadList = JSON.parse(response.payload);
    assert.ok(payloadList);
    const testPrintJobId = payloadList[0].PrintJobID;
    const testWorkflowId = payloadList[0].WorkflowID;

    // Regenerate a new simulation report based
    // on that print job and workflow
    response = await fastify.inject({
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
    assert.ok(simulationReport);
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
    assert.ok(payloadList);
    assert.notEqual(payloadList.length, 0);
    console.log("All workflow steps: ", payloadList);
});