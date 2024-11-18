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

// SEQUENCED TEST
test('Full simulation report flow', async (ctx) => {
    const jobTitle = "Test Job";
    ctx.jobID = "";
    ctx.workflowID = "";

    // Create a test print job and workflow
    await test('POST /createJob', async () => {
        const response = await fastify.inject({
            method: 'POST',
            url: '/createJob',
            body: {
                Title: jobTitle,
                PageCount: 10,
                RasterizationProfile: 'Black'
            }
        });
        assert.strictEqual(response.statusCode, 200);
    });
    
    await test('POST /createWorkflowStep', async () => {
        const response = await fastify.inject({
            method: 'POST',
            url: '/createWorkflowStep',
            body: {
                Title: 'Test Step 1',
                PreviousStep: '',
                NextStep: '',
                SetupTime: 5,
                TimePerPage: 2
            }
        });
        assert.strictEqual(response.statusCode, 200);
    });

    await test('POST /createWorkflow', async () => {
        const response = await fastify.inject({
            method: 'POST',
            url: '/createWorkflow',
            body: {
                Title: 'Test Workflow',
                WorkflowSteps: ['Test Step 1']
            }
        });
        assert.strictEqual(response.statusCode, 200);
    });

    // Get the print job and workflow id
    await test('GET /getPrintJob', async () => {
        const title = encodeURIComponent(jobTitle);
        const response = await fastify.inject({
            method: 'GET',
            url: `/getPrintJob?Title=${title}`
        });
        assert.strictEqual(response.statusCode, 200);
        const payload = JSON.parse(response.payload);
        assert.ok(payload)
        console.error(payload);

        ctx.workflowID = payload.WorkflowID;
        ctx.jobID = payload.PrintJobID; 
    });

    // Generate the simulation report
    await test('GET /generateSimulationReport', async () => {
        response = await fastify.inject({
            method: 'GET',
            url: '/generateSimulationReport',
            query: {
                jobID: ctx.jobID,
                workflowID: ctx.workflowID
            }
        });
        if (response.statusCode !== 200) {
            console.log("Error: ", response.payload);
        }
        assert.strictEqual(response.statusCode, 200);
        let simulationReport = await JSON.parse(response.payload);
        assert.ok(simulationReport);
        assert.strictEqual(simulationReport.PrintJobID, ctx.jobID);
        assert.strictEqual(simulationReport.WorkflowID, ctx.workflowID);
        console.log("Generated simulation report: ", simulationReport);
    });

    await test('GET /getSimulationReport', async () => {
        // Make sure that the simulation report 
        // can be retrieved
        response = await fastify.inject({
            method: 'GET',
            url: `/getSimulationReport?jobID=${encodeURIComponent(ctx.jobID)}&workflowID=${encodeURIComponent(ctx.workflowID)}`
        });
        assert.strictEqual(response.statusCode, 200);
        const payload = JSON.parse(response.payload);
        assert.ok(payload);
        console.log("Simulation Report: ", payload);
    });
})

// GET API CALLS
test('GET /', async () => {
    const response = await fastify.inject({
        method: 'GET',
        url: '/'
    });
    assert.strictEqual(response.statusCode, 200);
    assert.strictEqual(response.payload, 'Hello, client!');
});

test('GET /query', async () => {
    const CollectionName = encodeURIComponent('PrintJob');
    const Query = encodeURIComponent(JSON.stringify({ Title: "PrintJob 1" }))
    const response = await fastify.inject({
        method: 'GET',
        url: `/query?CollectionName=${CollectionName}&Query=${Query}`
    });
    assert.strictEqual(response.statusCode, 200);
    console.log("Query Results: ", JSON.parse(response.payload));
});

test('GET /getWorkflowStepList', async () => {
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

test('GET /getSimulationReportList', async () => {
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