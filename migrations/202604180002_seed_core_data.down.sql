DELETE FROM columns WHERE id IN (1, 2, 3);
DELETE FROM diaries WHERE id IN (1, 2, 3);
DELETE FROM meals WHERE id IN (1, 2, 3, 4, 5);
DELETE FROM body_records WHERE id IN (1, 2, 3, 4, 5);
DELETE FROM users WHERE id IN (1, 2);

SELECT setval('users_id_seq', COALESCE((SELECT MAX(id) FROM users), 1), TRUE);
SELECT setval('body_records_id_seq', COALESCE((SELECT MAX(id) FROM body_records), 1), TRUE);
SELECT setval('meals_id_seq', COALESCE((SELECT MAX(id) FROM meals), 1), TRUE);
SELECT setval('diaries_id_seq', COALESCE((SELECT MAX(id) FROM diaries), 1), TRUE);
SELECT setval('columns_id_seq', COALESCE((SELECT MAX(id) FROM columns), 1), TRUE);