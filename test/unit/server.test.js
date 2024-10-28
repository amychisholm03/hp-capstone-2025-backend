const { test } = require('node:test');
const assert = require('node:assert');
const { fastify, start } = require('../../server.js');

test.before(async () => {
    await start();    
});
test.after(() => {
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

test('POST /query', async (t) => {
    const response = await fastify.inject({
        method: 'POST',
        url: '/query',
        headers: {
            'Content-Type': 'application/json'
        },
        body: '{"CollectionName": "PrintJob", "Query": {"Title": "PrintJob 1"}}'
    });
    assert.strictEqual(response.statusCode, 200);
    console.log(response.payload);
});

test('GET /getSimulationReport', async (t) => {
    // parameters
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
