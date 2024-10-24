INSERT INTO rooms(id, name, active, deactivate_after_timestamp)
VALUES 
  (1, 'Enabled',                  true,  NULL),
  (2, 'Inactive',                 false, NULL),
  (3, 'Deactivated by Timestamp', true,  '2024-09-01');
