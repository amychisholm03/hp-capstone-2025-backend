<h6>PrintJob</h6>
id: ObjectID (pk),
Title: string, 
DateCreated: Timestamp, 
PageCount: int, 
RasterizationProfile: string

<h6>Workflow</h6>
id: ObjectID (pk),
Title: string, 
WorkflowSteps: array of WorkflowStep ObjectIDs (fk)

<h6>WorkflowStep</h6>
id: ObjectID (pk),
Title: string, 
PreviousStep: WorkflowStep ObjectID (nullable),
NextStep: WorkflowStep ObjectID (nullable),
SetupTime: int,
TimePerPage: int

<h6>SimulationReport</h6>
id: ObjectID (pk),
PrintJobID: PrintJob ObjectID (fk),
WorkflowID: Workflow ObjectID (fk),
TotalTimeTaken: int,
RasterizationTimeTaken: int