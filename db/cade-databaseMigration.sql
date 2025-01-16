-- Create a table to define rasterization profiles
CREATE TABLE IF NOT EXISTS rasterization_profile (
   id INTEGER PRIMARY KEY,
   title TEXT NOT NULL
);

-- Create a table to define print jobs
CREATE TABLE IF NOT EXISTS printjob (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    creation_time DATETIME DEFAULT CURRENT_TIMESTAMP,
    page_count INTEGER,
    rasterization_profile_id INTEGER,
    FOREIGN KEY (rasterization_profile_id) REFERENCES rasterization_profile(id)
);

-- Create a table to define workflows
CREATE TABLE IF NOT EXISTS workflow (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL
);

-- Create a table to define simulation reports
CREATE TABLE IF NOT EXISTS simulation_report (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    printjobID INTEGER NOT NULL,
    workflowID INTEGER NOT NULL,
    FOREIGN KEY (printjobID) REFERENCES printjob(id),
    FOREIGN KEY (workflowID) REFERENCES workflow(id)
);

-- Create a table to define workflow steps
CREATE TABLE IF NOT EXISTS workflow_step (
	id INTEGER PRIMARY KEY,
	title TEXT NOT NULL,
	setup_time INTEGER,
	time_per_page INTEGER
);

-- Create a table to track workflow steps which are part of a workflow
CREATE TABLE assigned_workflow_step (
    workflow_id INTEGER, 		-- which workflow this step belongs to.
    workflow_step_id INTEGER, 		-- which type of workflow step this is.
    next_step_id INTEGER, 		-- which workflow step comes next.
    PRIMARY KEY (workflow_id, workflow_step_id, next_step_id),
    FOREIGN KEY (workflow_id) REFERENCES workflow(id),
    FOREIGN KEY (next_step_id) REFERENCES assigned_workflow_step(id)
);

--- Create a table to track workflow step results.
CREATE TABLE ran_workflow_step (
   workflow_step_id INTEGER,
   simulation_report_id INTEGER,
   time_taken INTEGER,
   PRIMARY KEY (workflow_step_id, simulation_report_id),
   FOREIGN KEY (workflow_step_id) REFERENCES assigned_workflow_step(id),
   FOREIGN KEY (simulation_report_id) REFERENCES simulation_report(id)
);
