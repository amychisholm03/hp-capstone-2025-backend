const { test } = require('node:test');
const assert = require('node:assert');
const { fastify, start } = require('../../server.js');

test('GET /', async (t) => {
    t.after(() => fastify.close());
    await start().then(async () => {;
        const response = await fastify.inject({
            method: 'GET',
            url: '/'
        });
        assert.strictEqual(response.statusCode, 200);
        assert.strictEqual(response.payload, 'Hello, client!');
    });
});

/*
test('POST /createJob', async (t) => {
    const mockDatabase = {}; // Mock your database connection here
    const requestBody = {
        Title: 'Test Job',
        PageCount: 10,
        RasterizationProfile: 'Profile1'
    };

    const response = await fastify.inject({
        method: 'POST',
        url: '/createJob',
        payload: requestBody
    });

    assert.strictEqual(response.statusCode, 200);
    assert.strictEqual(response.payload, 'Operation successful\n');
});

test('POST /createWorkflow', async (t) => {
    const mockDatabase = {}; // Mock your database connection here
    const requestBody = {
        Title: 'Test Workflow',
        WorkflowSteps: ['Step1', 'Step2']
    };

    const response = await fastify.inject({
        method: 'POST',
        url: '/createWorkflow',
        payload: requestBody
    });

    assert.strictEqual(response.statusCode, 200);
    assert.strictEqual(response.payload, 'Operation successful\n');
});

test('POST /createWorkflowStep', async (t) => {
    const mockDatabase = {}; // Mock your database connection here
    const requestBody = {
        Title: 'Test Step',
        PreviousStep: 'Step1',
        NextStep: 'Step3',
        SetupTime: 5,
        TimePerPage: 2
    };

    const response = await fastify.inject({
        method: 'POST',
        url: '/createWorkflowStep',
        payload: requestBody
    });

    assert.strictEqual(response.statusCode, 200);
    assert.strictEqual(response.payload, 'Operation successful\n');
});

test('POST /query', async (t) => {
    const mockDatabase = {
        collection: () => ({
            find: () => ({
                toArray: async () => [{ name: 'Test' }]
            })
        })
    };
    const requestBody = {
        CollectionName: 'testCollection',
        Query: {}
    };

    const response = await fastify.inject({
        method: 'POST',
        url: '/query',
        payload: requestBody
    });

    assert.strictEqual(response.statusCode, 200);
    assert.deepStrictEqual(JSON.parse(response.payload), [{ name: 'Test' }]);
});
*/