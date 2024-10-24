<h5>Database API sample curl requests</h5>
<h6>Create a new PrintJob</h6>

curl -X POST http://54.200.253.84:80/createJob -H "Content-Type: application/json" -d '{"Title": "PrintJob 2", "PageCount": 9, "RasterizationProfile": ["RP 3"]}'

<h6>Create a new Workflow</h6>

curl -X POST http://54.200.253.84:80/createWorkflow -H "Content-Type: application/json" -d '{"Title": "Workflow 2", "WorkflowSteps": []}'

<h6>Create a new WorkflowStep</h6>

curl -X POST http://54.200.253.84:80/createWorkflowStep -H "Content-Type: application/json" -d '{"Title": "Pizza", "PreviousStep": null, "NextStep": null, "SetupTime": 11, "TimePerPage": 3}'

<h6>Query a collection</h6>

curl -X POST http://54.200.253.84:80/query -H "Content-Type: application/json" -d '{"CollectionName": "PrintJob", "Query": {"Title": "PrintJob 1"}}'

<h6>Invalid createJob example</h6>

curl -X POST http://54.200.253.84:80/createJob -H "Content-Type: application/json" -d '{"Title": "Pie", "PageCount": 9, "RaspberryProfile": ["RP 3"]}'
