DROP TABLE IF EXISTS workflow;

-- Create a table to define rasterization profiles
CREATE TABLE IF NOT EXISTS rasterization_profile (
   id INTEGER PRIMARY KEY,
   title TEXT NOT NULL,
   profile TEXT NOT NULL
);

-- Create a table to define print jobs
CREATE TABLE IF NOT EXISTS printjob (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    creation_time INTEGER,
    page_count INTEGER,
    rasterization_profile_id INTEGER,
    FOREIGN KEY (rasterization_profile_id) REFERENCES rasterization_profile(id)
);

-- Create a table to define workflows
CREATE TABLE IF NOT EXISTS workflow (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS workflow_step (
    id INTEGER PRIMARY KEY
);

-- Create a table to define simulation reports
CREATE TABLE IF NOT EXISTS simulation_report (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    creation_time INTEGER,
    total_time_taken INTEGER,
    printjobID INTEGER NOT NULL,
    workflowID INTEGER NOT NULL,
    FOREIGN KEY (printjobID) REFERENCES printjob(id),
    FOREIGN KEY (workflowID) REFERENCES workflow(id)
);

-- A workflow step which is assigned to a specific workflow
CREATE TABLE IF NOT EXISTS assigned_workflow_step (
   id INTEGER,
   workflow_id INTEGER,
   workflow_step_id INTEGER,
   PRIMARY KEY (id),
   FOREIGN KEY (workflow_id) REFERENCES workflow(id),
   FOREIGN KEY (workflow_step_id) REFERENCES workflow_step(id)
);

-- Create a table to track how long each simulated step took
CREATE TABLE IF NOT EXISTS simulation_report_step_time (
  simulation_report_id INTEGER,
  assigned_workflow_step_id INTEGER,
  step_time INTEGER,
  PRIMARY KEY (simulation_report_id, assigned_workflow_step_id),
  FOREIGN KEY (simulation_report_id) REFERENCES simulation_report(id),
  FOREIGN KEY (assigned_workflow_step_id) REFERENCES assigned_workflow_step(id)
);

-- Create a table to track workflow steps which are part of a workflow
CREATE TABLE IF NOT EXISTS next_workflow_step (
    assigned_workflow_step_id INTEGER,  -- the id of the assigned workflow step this is 
    next_step_id INTEGER,                 -- which workflow step comes next.
    PRIMARY KEY (assigned_workflow_step_id, next_step_id),
    FOREIGN KEY (next_step_id) REFERENCES assigned_workflow_step(id),
    FOREIGN KEY (assigned_workflow_step_id) REFERENCES assigned_workflow_step(id)
);

CREATE TABLE IF NOT EXISTS prev_workflow_step (
    assigned_workflow_step_id INTEGER,  -- the id of the assigned workflow step this is 
    prev_step_id INTEGER,                 -- which workflow step came last.
    PRIMARY KEY (assigned_workflow_step_id, prev_step_id),
    FOREIGN KEY (prev_step_id) REFERENCES assigned_workflow_step(id),
    FOREIGN KEY (assigned_workflow_step_id) REFERENCES assigned_workflow_step(id)
);

--- Create a table to track workflow step results.
CREATE TABLE IF NOT EXISTS ran_workflow_step (
   workflow_step_id INTEGER,
   simulation_report_id INTEGER,
   time_taken INTEGER,
   PRIMARY KEY (workflow_step_id, simulation_report_id),
   FOREIGN KEY (workflow_step_id) REFERENCES assigned_workflow_step(id),
   FOREIGN KEY (simulation_report_id) REFERENCES simulation_report(id)
);

--- Workflow Step Paramaters
CREATE TABLE IF NOT EXISTS rasterization_params (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    assigned_workflow_step_id INTEGER,
    num_of_RIPs INTEGER,
    FOREIGN KEY (assigned_workflow_step_id) REFERENCES assigned_workflow_step(id)
);

-- Create table to keep track of user login information
CREATE TABLE IF NOT EXISTS user (
    user_id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL, -- use a secure hashing function for passwords, should be handled by backend
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
