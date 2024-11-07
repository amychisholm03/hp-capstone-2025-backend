<h5>Resources<h5>
MongoDB shell cheat sheet
https://www.slingacademy.com/article/mongodb-shell-commands-the-complete-cheat-sheet/#Advanced_Querying

<h5>Database API sample curl requests</h5>
<h6>Sample GET request</h6>

These two curl requests do the same thing:
curl -X GET "http://54.200.253.84:80/query?CollectionName=PrintJob&Query=%7B%22Title%22%3A%22PrintJob%201%22%7D"

curl -G -X GET http://54.200.253.84:80/query --data-urlencode "CollectionName=PrintJob" --data-urlencode "Query={\"Title\":\"PrintJob 1\"}"

Sample code used to send a request
```javascript
const CollectionName = encodeURIComponent('PrintJob');
const Query = encodeURIComponent(JSON.stringify({Title: "PrintJob 1"}))
const response = await fastify.inject({
    method: 'GET',
    url:`/query?CollectionName=${CollectionName}&Query=${Query}`
});
```

<h6>Create a new PrintJob</h6>

curl -X POST http://54.200.253.84:80/createJob -H "Content-Type: application/json" -d '{"Title": "PrintJob 2", "PageCount": 9, "RasterizationProfile": ["RP 3"]}'

<h6>Create a new Workflow</h6>

curl -X POST http://54.200.253.84:80/createWorkflow -H "Content-Type: application/json" -d '{"Title": "Workflow 2", "WorkflowSteps": []}'

<h6>Create a new WorkflowStep</h6>

curl -X POST http://54.200.253.84:80/createWorkflowStep -H "Content-Type: application/json" -d '{"Title": "Pizza", "PreviousStep": null, "NextStep": null, "SetupTime": 11, "TimePerPage": 3}'

<h6>Invalid createJob example</h6>

curl -X POST http://54.200.253.84:80/createJob -H "Content-Type: application/json" -d '{"Title": "Pie", "PageCount": 9, "RaspberryProfile": ["RP 3"]}'
