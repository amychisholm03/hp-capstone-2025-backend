/**
 * @typedef {Object} PrintJob
 * @prop {ObjectId} _id
 * @prop {string} title
 * @prop {Timestamp} DateCreated
 * @prop {int} PageCount
 * @prop {string} RasterizationProfile
 */

/**
 * @typedef {Object} WorkflowStep
 * @prop {ObjectId} _id
 * @prop {string} Title
 * @prop {ObjectId} PreviousStep
 * @prop {ObjectId} NextStep
 * @prop {int} SetupTime
 * @prop {int} TimePerPage
 */

/** 
 * @typedef {Object} Workflow
 * @prop {ObjectId} _id
 * @prop {string} Title
 * @prop {[WorkflowStep]} WorkflowSteps
 */

/**
 * @typedef {Object} SimulationReport
 * @prop {ObjectId} _id
 * @prop {ObjectId} PrintJobID
 * @prop {ObjectId} WorkflowID
 * @prop {int} TotalTimeTaken
 * @prop {int} RasterizationTimeTaken
 * @prop {int} CreationTime
 */

exports.unused = {};