-- whip up some of the data that can't be created from the GUI
DROP TABLE IF EXISTS rasterization_profile;
DROP TABLE IF EXISTS workflow_step;

INSERT INTO rasterization_profile (id, title, profile) VALUES (NULL, 'BW', 'Standard Color Profile');
INSERT INTO rasterization_profile (id, title, profile) VALUES (NULL, 'CMY', 'Standard Color Profile');
INSERT INTO rasterization_profile (id, title, profile) VALUES (NULL, 'CMYK', 'Standard Color Profile');
INSERT INTO rasterization_profile (id, title, profile) VALUES (NULL, 'High Quality (Best Detail)', 'Specialized Profile');
INSERT INTO rasterization_profile (id, title, profile) VALUES (NULL, 'Line Art (Crisp Lines, No Gradients)', 'Specialized Profile');

-- updated these to match HP's example workflow
INSERT INTO workflow_step (id,title,setup_time,time_per_page) VALUES (NULL, 'Download File',0,1); -- 0
INSERT INTO workflow_step (id,title,setup_time,time_per_page) VALUES (NULL, 'Preflight',10,20); -- 1
INSERT INTO workflow_step (id,title,setup_time,time_per_page) VALUES (NULL, 'Impose',0,5); -- 2
INSERT INTO workflow_step (id,title,setup_time,time_per_page) VALUES (NULL, 'Analyzer',0,5); -- 3
INSERT INTO workflow_step (id,title,setup_time,time_per_page) VALUES (NULL, 'Color Setup',2,1); -- 4
INSERT INTO workflow_step (id,title,setup_time,time_per_page) VALUES (NULL, 'Rasterization',50,15); -- 5
INSERT INTO workflow_step (id,title,setup_time,time_per_page) VALUES (NULL, 'Loader',100,1); -- 6
INSERT INTO workflow_step (id,title,setup_time,time_per_page) VALUES (NULL, 'Cutting',10,2); -- 7
INSERT INTO workflow_step (id,title,setup_time,time_per_page) VALUES (NULL, 'Laminating',10,5); -- 8