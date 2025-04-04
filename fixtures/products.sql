INSERT INTO products(id, name, price, active, deactivate_after_timestamp)
VALUES 
  (1, 'Enabled',                  700 ,         true,  NULL),
  (2, 'No aliases',               1200,         true,  NULL),
  (3, 'Inactive',                 200,          false, NULL),
  (4, 'Deactivated by Timestamp', 30000,        true,  '2024-09-01'),
  (5, 'Expensive',                100000,       true,  NULL),
  (6, 'Overflow trigger',         100000000000, true,  NULL);

INSERT INTO room_products(room_id, product_id)
VALUES
  (1, 1),
  (1, 2),
  (1, 3),
  (1, 4),
  (1, 5),
  (1, 6);

