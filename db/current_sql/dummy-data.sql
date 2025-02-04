-- whip up some of the data that can't be created from the GUI

DELETE FROM rasterization_profile;
DELETE FROM workflow_step;

INSERT INTO rasterization_profile (id, title, profile) VALUES (NULL, 'BW', 'Standard Color Profile');
INSERT INTO rasterization_profile (id, title, profile) VALUES (NULL, 'CMY', 'Standard Color Profile');
INSERT INTO rasterization_profile (id, title, profile) VALUES (NULL, 'CMYK', 'Standard Color Profile');
INSERT INTO rasterization_profile (id, title, profile) VALUES (NULL, 'High Quality (Best Detail)', 'Specialized Profile');
INSERT INTO rasterization_profile (id, title, profile) VALUES (NULL, 'Line Art (Crisp Lines, No Gradients)', 'Specialized Profile');

INSERT INTO workflow_step (id,title,setup_time,time_per_page) VALUES (NULL, 'Preflight',10,7);
INSERT INTO workflow_step (id,title,setup_time,time_per_page) VALUES (NULL, 'Metrics',2,1);
INSERT INTO workflow_step (id,title,setup_time,time_per_page) VALUES (NULL, 'Rasterization',50,16);
INSERT INTO workflow_step (id,title,setup_time,time_per_page) VALUES (NULL, 'Printing',10,7);
INSERT INTO workflow_step (id,title,setup_time,time_per_page) VALUES (NULL, 'Cutting',10,7);
INSERT INTO workflow_step (id,title,setup_time,time_per_page) VALUES (NULL, 'Laminating',10,7);