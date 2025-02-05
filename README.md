<h1>Running the Application</h1>

<h5>Run</h5>
<p>Use the following commands to run the application:</p>
<ul>
  <li><code>cargo run</code>: Runs the server with default parameters.</li>
  <li><code>cargo run l</code>: Runs the server on <code>localhost:5040</code>.</li>
</ul>


<h1>File Structure</h1>
<table>
  <thead>
    <tr>
      <th>File</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>src/api.rs</code></td>
      <td>Sets up REST API routes.</td>
    </tr>
    <tr>
      <td><code>src/database.rs</code></td>
      <td>Interfaces with the database and defines the data structures.</td>
    </tr>
    <tr>
      <td><code>src/main.rs</code></td>
      <td>Runs the server.</td>
    </tr>
    <tr>
      <td><code>src/simulation.rs</code></td>
      <td>Simulates a print job going through a workflow.</td>
    </tr>
   <tr>
      <td><code>src/validation.rs</code></td>
      <td>Validates a workflow before it gets inserted into the database.</td>
    </tr>
    <tr>
      <td><code>tests/</code></td>
      <td>Contains test files.</td>
    </tr>
        <tr>
      <td><code>db/</code></td>
      <td>Contains SQLite database binary file, and SQL</td>
    </tr>
  </tbody>
</table>

<h1>Database Schema</h1>
<p> This project uses SQLite 3. <br> 
SQLite stores data in a single file (.db3)<br>
The database is executed and maintained via the Rusqlite library for Rust.

If you require access to the database to inspect it's contents, or to make changes,`<br>`you may do so via the sqlite3 program. `<br>`

- To access an existing database:
  - sqlite3 /path/to/database.db3
- To execute SQL on a database
  - sqlite3 /path/to/database.db3 < /path/to/file.sql

</p>

<h2>rasterization_profile</h2>
<p>Stores the various rasterization profile options. Currently, only the title is stored, but more fields may be added soon.</p>
<table>
  <thead>
    <tr>
      <th>Attribute</th>
      <th>Type</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>id (pk)</code></td>
      <td>integer</td>
    </tr>
    <tr>
      <td><code>title</code></td>
      <td>text</td>
    </tr>
  </tbody>
</table>

<h2>printjob</h2>
<p>Print jobs are used in conjunction with workflows to run simulations. Factors such as <code>page_count</code> and <code>rasterization_profile</code> will affect simulation times.</p>
<table>
  <thead>
    <tr>
      <th>Attribute</th>
      <th>Type</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>id (pk)</code></td>
      <td>integer</td>
    </tr>
    <tr>
      <td><code>title</code></td>
      <td>text</td>
    </tr>
    <tr>
      <td><code>creation_time</code></td>
      <td>integer</td>
    </tr>
    <tr>
      <td><code>page_count</code></td>
      <td>integer</td>
    </tr>
    <tr>
      <td><code>rasterization_profile_id (fk)</code></td>
      <td>integer</td>
    </tr>
  </tbody>
</table>

<h2>workflow</h2>
<p>Workflows simulate print jobs and define the steps involved. The steps of a workflow are stored in the <code>workflow_step</code> table and assigned to workflows via the <code>assigned_workflow_step</code> table.</p>
<table>
  <thead>
    <tr>
      <th>Attribute</th>
      <th>Type</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>id (pk)</code></td>
      <td>integer</td>
    </tr>
    <tr>
      <td><code>title</code></td>
      <td>text</td>
    </tr>
    <tr>
      <td><code>creation_time</code></td>
      <td>integer</td>
    </tr>
    <tr>
      <td><code>total_time_taken</code></td>
      <td>integer</td>
    </tr>
    <tr>
      <td><code>printjobID (fk)</code></td>
      <td>integer</td>
    </tr>
    <tr>
      <td><code>workflowID (fk)</code></td>
      <td>integer</td>
    </tr>
  </tbody>
</table>

<h2>workflow_step</h2>
<table>
  <thead>
    <tr>
      <th>Attribute</th>
      <th>Type</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>id</code></td>
      <td>integer (pk)</td>
    </tr>
    <tr>
      <td><code>title</code></td>
      <td>text</td>
    </tr>
    <tr>
      <td><code>setup_time</code></td>
      <td>integer</td>
    </tr>
    <tr>
      <td><code>time_per_page</code></td>
      <td>integer</td>
    </tr>
  </tbody>
</table>

<h2>assigned_workflow_step</h2>
<p>Assigns workflow steps to specific workflows.</p>
<table>
  <thead>
    <tr>
      <th>Attribute</th>
      <th>Type</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>id</code></td>
      <td>integer (pk)</td>
    </tr>
    <tr>
      <td><code>workflow_id (fk)</code></td>
      <td>integer</td>
    </tr>
    <tr>
      <td><code>workflow_step_id (fk)</code></td>
      <td>integer</td>
    </tr>
  </tbody>
</table>

<h2>next_workflow_step</h2>
<p>Defines the sequence of steps in a workflow by linking a workflow step to its next step.</p>
<table>
  <thead>
    <tr>
      <th>Attribute</th>
      <th>Type</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>assigned_workflow_step_id</code></td>
      <td>integer (pk, fk)</td>
    </tr>
    <tr>
      <td><code>next_step_id</code></td>
      <td>integer (pk, fk)</td>
    </tr>
  </tbody>
</table>

<h2>prev_workflow_step</h2>
<p>Defines the reverse sequence of steps in a workflow by linking a workflow step to its previous step.</p>
<table>
  <thead>
    <tr>
      <th>Attribute</th>
      <th>Type</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>assigned_workflow_step_id</code></td>
      <td>integer (pk, fk)</td>
    </tr>
    <tr>
      <td><code>prev_step_id</code></td>
      <td>integer (pk, fk)</td>
    </tr>
  </tbody>
</table>

<h2>ran_workflow_step</h2>
<p>
Tracks the execution of workflow steps within a simulation report.<br>
These haven't been implemented yet.
</p>
<table>
  <thead>
    <tr>
      <th>Attribute</th>
      <th>Type</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>workflow_step_id (fk)</code></td>
      <td>integer</td>
    </tr>
    <tr>
      <td><code>simulation_report_id (fk)</code></td>
      <td>integer</td>
    </tr>
    <tr>
      <td><code>time_taken</code></td>
      <td>integer</td>
    </tr>
  </tbody>
</table>

<h1>Other Data Structures</h1>
<p>These could either be hardcoded or stored in a database:</p>
<ul>
  <li><strong>Rules:</strong> A data structure that enforces constraints on data being entered into the database. For example, ensuring specific workflow steps are performed in a particular order (e.g., printing must occur before laminating). These rules can be requested by the frontend to provide real-time feedback when creating resources. The frontend sends the validated data to the backend, where the same rules are enforced.</li>
</ul>

<h1>REST API</h1>

<h2>HTTP Methods</h2>
<p>For more information, visit the following resources:</p>
<ul>
  <li><a href="https://restfulapi.net/">RESTful API Guide</a></li>
  <li><a href="https://restfulapi.net/http-methods/">HTTP Methods</a></li>
</ul>

<h2>GET</h2>
<ul>
  <li><code>GET /[COLL]?opt_param1=example1&opt_param2=example2</code><br>
    Retrieves all documents from a collection matching the given parameters. If no parameters are specified, returns the entire collection.
    <ul>
      <li><strong>200 (OK):</strong> Returns a list of documents (can be empty).</li>
      <li><strong>400 (Bad Request):</strong> Improperly formatted query.</li>
    </ul>
  </li>
  <li><code>GET /[COLL]/:id</code><br>
    Retrieves a specific document by ID.
    <ul>
      <li><strong>200 (OK):</strong> Returns the document.</li>
      <li><strong>400 (Bad Request):</strong> Invalid ID format.</li>
      <li><strong>404 (Not Found):</strong> Document does not exist.</li>
    </ul>
  </li>
</ul>

<h2>POST</h2>
<ul>
  <li><code>POST /RasterizationProfile</code><br>
    Creates a new rasterization profile. Request body includes:
    <ul>
    <li><strong>id</strong></li>  
      <li><strong>title</strong></li>  
    </ul>
    <ul>
    </ul>
  </li>
  <li><code>POST /PrintJob</code><br>
    Creates a new print job. Request body includes:
    <ul>
      <li><strong>Title</strong></li>
      <li><strong>DateCreated</strong></li>
      <li><strong>PageCount</strong></li>
      <li><strong>RasterizationProfile</strong></li>
    </ul>
    <ul>
    </ul>
  </li>
  <li><code>POST /Workflow</code><br>
    Creates a new workflow. Request body includes:
    <ul>
      <li><strong>Title</strong></li>
      <li><strong>WorkflowSteps</strong></li>
    </ul>
    Will return a `422` if invalid workflow (i.e. no workflow steps or cyclic)
  </li>
  <li><code>POST /SimulationReport</code><br>
    Creates a new simulation report. Request body includes:
    <ul>
      <li><strong>pj_id</strong></li>
      <li><strong>wf_id</strong></li>
    </ul>
    <ul>
      <li><strong>201 (Created):</strong> Returns new SimulationReport ID.</li>
    </ul>
  </li>
</ul>

<h2>DELETE</h2>
<ul>
  <li><code>DELETE /RasterizationProfile/:id</code><br>
    Deletes a specific print job by ID.
    <ul>
      <li><strong>204 (No Content):</strong> Successful deletion.</li>
      <li><strong>400 (Bad Request):</strong> Invalid ID format.</li>
      <li><strong>404 (Not Found):</strong> Document does not exist.</li>
      <li><strong>409 (Conflict):</strong> Existing PrintJob rely on this PrintJob.</li>
    </ul>
  </li>
  <li><code>DELETE /PrintJob/:id</code><br>
    Deletes a specific print job by ID.
    <ul>
      <li><strong>204 (No Content):</strong> Successful deletion.</li>
      <li><strong>400 (Bad Request):</strong> Invalid ID format.</li>
      <li><strong>404 (Not Found):</strong> Document does not exist.</li>
      <li><strong>409 (Conflict):</strong> Existing SimulationReports rely on this PrintJob.</li>
    </ul>
  </li>
  <li><code>DELETE /Workflow/:id</code><br>
    Deletes a specific workflow by ID.
    <ul>
      <li><strong>204 (No Content):</strong> Successful deletion.</li>
      <li><strong>400 (Bad Request):</strong> Invalid ID format.</li>
      <li><strong>404 (Not Found):</strong> Document does not exist.</li>
      <li><strong>409 (Conflict):</strong> Existing SimulationReports rely on this Workflow.</li>
    </ul>
  </li>
  <li><code>DELETE /SimulationReport/:id</code><br>
    Deletes a specific simulation report by ID.
    <ul>
      <li><strong>204 (No Content):</strong> Successful deletion
