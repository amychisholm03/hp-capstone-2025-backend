<h5>Run</h5>
run `cargo run` in root directory for default(server) parameters<br>
`cargo run l` to run on localhost:5040

<h5>Files</h5>
src/api.rs    		Sets up REST API routes<br>
src/database.rs    	Interfaces with the database and defines the data structures<br>
src/main.rs    		Runs the server<br>
src/simulation.rs   Simulates a printjob going through a workflow<br>
tests/    			Tests<br>


<h5>Database Schemas</h5>

	PrintJob
		id (pk)
		Title: string
		DateCreated: number
		PageCount: integer
		RasterizationProfileID: integer (fk)

	Workflow
		id (pk)
		Title: String
		WorkflowSteps: List of tuples
			wfs_id (fk)
			Prev: List of indices
			Next: List of indices

	WorkflowStep
		id (pk)
		Title: string
		SetupTime: number
		TimePerPage: number

	SimulationReport
		id (pk)
		pj_id (fk)
		wf_id (fk)
		CreationTime: number
		TotalTime: number
		StepTimes: List of tuples
			wfs_id (fk)
			StepTime: number


<h5>Other Data Structures (Could be hardcoded or put in a database)</h5>
	Rules: A data structure that applies further constraints on the data being put into the database. For example, what workflow steps must be done before a later step, like printing must be done before laminating. These rules could be requested by the frontend to allow for more responsive feedback when creating a new resource, then the frontend would send that data to the backend, and the backend would verify the data using the same rules.


<h5>REST API</h5>
https://restfulapi.net/<br>
https://restfulapi.net/http-methods/
	
	GET
		/[COLL]?opt_param1=example1&opt_param2=example2
			Retrieves all documents from a collection matching the given parameters. If none are specified, it returns the entire collection
			200(OK): Returns list of documents, can be empty if there are no matches
			400(Bad Request): Improperly formatted query

		/[COLL]/:id
			Retrieves the document with the specified ID
			200(OK): Returns the document
			400(BAD REQUEST): ID is invalid
			404(Not Found): Document doesn't exist

	POST
		/PrintJob
			Request body includes Title, DateCreated, PageCount, RasterizationProfile
			201(Created): Returns new PrintJob ID*
			//TODO: Failure Codes

		/Workflow
			Request body includes Title, WorkflowSteps
			201(Created): Returns new Workflow ID*
			//TODO: Failure Codes

		/SimulationReport
			Request body includes pj_id, wf_id
			201(Created): Returns new SimulationReport ID*
			//TODO: Failure Codes

	DELETE
		/PrintJob/:id
			204(No Content): Successful deletion, no additional data returned
			400(BAD REQUEST): ID is invalid
			404(Not Found): 404 Not Found
			409(Conflict): Existing SimulationReports rely on this PrintJob, can't delete
				//TODO: Should this return a list of SimulationReport IDs? Or should the frontend do a seperate GET?

		/Workflow/:id
			204(No Content): Successful deletion, no additional data returned
			400(BAD REQUEST): ID is invalid
			404(Not Found): 404 Not Found
			409(Conflict): Existing SimulationReports rely on this PrintJob, can't delete
				//TODO: Should this return a list of SimulationReport IDs? Or should the frontend do a seperate GET?

		/SimulationReport/:id
			204(No Content): Successful deletion, no additional data returned
			400(BAD REQUEST): ID is invalid
			404(Not Found): 404 Not Found


<h5>API calls to consider</h5>
	Bulk operations (eg. POST /PrintJob/bulk, GET /PrintJob/bulk): Creates or retrieves multiple documents at once.<br>
	Pagination (eg. GET /PrintJob?page=1&limit=20): May be a good idea if data set gets large<br>
	Health (GET /health): Checks if the API is running<br>


<h5>Not Implementing</h5>
	PUT/PATCH: Replaces an existing document/some of its fields. I don't think these makes sense for our use case as updating PrintJobs and Workflows would invalidate any simulation reports that rely on them. If the frontend wants to "update" one of these, it should first delete the old one, then send the new one with POST. For SimulationReports and WorkflowSteps, the frontend shouldn't be able to modify these.
<br>
<br>  
*from RESTful API: the response SHOULD be HTTP response code 201 (Created) and contain an entity that describes the status of the request and refers to the new resource, and a Location header.
