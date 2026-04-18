INSERT INTO users (
    id,
    email,
    full_name,
    password_hash,
    previous_password,
    last_login,
    is_admin,
    is_verified,
    is_password_reset,
    login_attempts,
    last_updated_password,
    avatar_url,
    reference_id,
    created_at,
    updated_at,
    deleted_at
) VALUES
    (
        1,
        'taro.yamada@example.com',
        'Taro Yamada',
        'f4d8a8d2d8dc6c5b12f9e7f4b1d0c3e8a6b3c4d5e6f708192a3b4c5d6e7f8091',
        NULL,
        '2026-04-18T07:30:00+00:00',
        FALSE,
        TRUE,
        FALSE,
        0,
        '2026-04-01T00:00:00+00:00',
        'https://images.unsplash.com/photo-1500648767791-00dcc994a43e',
        NULL,
        '2026-04-01T00:00:00+00:00',
        '2026-04-18T07:30:00+00:00',
        NULL
    ),
    (
        2,
        'hanako.sato@example.com',
        'Hanako Sato',
        'a1c2e3f40516273849a5b6c7d8e9f00112233445566778899aabbccddeeff001',
        NULL,
        '2026-04-17T21:15:00+00:00',
        FALSE,
        TRUE,
        FALSE,
        0,
        '2026-04-02T00:00:00+00:00',
        'https://images.unsplash.com/photo-1494790108377-be9c29b29330',
        NULL,
        '2026-04-02T00:00:00+00:00',
        '2026-04-17T21:15:00+00:00',
        NULL
    );

INSERT INTO body_records (
    id,
    user_id,
    name,
    image_url,
    recorded_at,
    weight,
    body_fat_rate,
    created_by,
    updated_by,
    created_at,
    updated_at,
    deleted_at
) VALUES
    (1, 1, 'Morning check-in', 'https://images.unsplash.com/photo-1517836357463-d25dfeac3438', '2026-04-10T06:30:00+00:00', 72.40, 21.80, 1, 1, '2026-04-10T06:30:00+00:00', '2026-04-10T06:30:00+00:00', NULL),
    (2, 1, 'Morning check-in', 'https://images.unsplash.com/photo-1517836357463-d25dfeac3438', '2026-04-11T06:30:00+00:00', 72.10, 21.50, 1, 1, '2026-04-11T06:30:00+00:00', '2026-04-11T06:30:00+00:00', NULL),
    (3, 1, 'Morning check-in', 'https://images.unsplash.com/photo-1517836357463-d25dfeac3438', '2026-04-12T06:30:00+00:00', 71.90, 21.20, 1, 1, '2026-04-12T06:30:00+00:00', '2026-04-12T06:30:00+00:00', NULL),
    (4, 2, 'Morning check-in', 'https://images.unsplash.com/photo-1490645935967-10de6ba17061', '2026-04-10T06:40:00+00:00', 54.80, 18.40, 2, 2, '2026-04-10T06:40:00+00:00', '2026-04-10T06:40:00+00:00', NULL),
    (5, 2, 'Morning check-in', 'https://images.unsplash.com/photo-1490645935967-10de6ba17061', '2026-04-11T06:40:00+00:00', 54.60, 18.20, 2, 2, '2026-04-11T06:40:00+00:00', '2026-04-11T06:40:00+00:00', NULL);

INSERT INTO meals (
    id,
    user_id,
    name,
    calories,
    meal_type,
    eaten_at,
    image_url,
    created_by,
    updated_by,
    created_at,
    updated_at,
    deleted_at
) VALUES
    (1, 1, 'Chicken salad bowl', 520.00, 'lunch', '2026-04-18T03:00:00+00:00', 'https://images.unsplash.com/photo-1546069901-ba9599a7e63c', 1, 1, '2026-04-18T03:00:00+00:00', '2026-04-18T03:00:00+00:00', NULL),
    (2, 1, 'Yogurt and berries', 220.00, 'snack', '2026-04-18T08:30:00+00:00', 'https://images.unsplash.com/photo-1488477181946-6428a0291777', 1, 1, '2026-04-18T08:30:00+00:00', '2026-04-18T08:30:00+00:00', NULL),
    (3, 1, 'Grilled salmon breakfast', 430.00, 'morning', '2026-04-18T00:30:00+00:00', 'https://images.unsplash.com/photo-1498837167922-ddd27525d352', 1, 1, '2026-04-18T00:30:00+00:00', '2026-04-18T00:30:00+00:00', NULL),
    (4, 2, 'Vegetable pasta', 610.00, 'dinner', '2026-04-17T11:30:00+00:00', 'https://images.unsplash.com/photo-1621996346565-e3dbc646d9a9', 2, 2, '2026-04-17T11:30:00+00:00', '2026-04-17T11:30:00+00:00', NULL),
    (5, 2, 'Fruit smoothie', 180.00, 'snack', '2026-04-17T06:15:00+00:00', 'https://images.unsplash.com/photo-1505252585461-04db1eb84625', 2, 2, '2026-04-17T06:15:00+00:00', '2026-04-17T06:15:00+00:00', NULL);

INSERT INTO diaries (
    id,
    user_id,
    title,
    content,
    image_url,
    created_by,
    updated_by,
    created_at,
    updated_at,
    deleted_at
) VALUES
    (
        1,
        1,
        'Kept the workout streak',
        'Ran 5 km before breakfast and felt lighter than last week. Planning to keep dinner lower in carbs tonight.',
        'https://images.unsplash.com/photo-1518611012118-696072aa579a',
        1,
        1,
        '2026-04-18T09:00:00+00:00',
        '2026-04-18T09:00:00+00:00',
        NULL
    ),
    (
        2,
        1,
        'Hydration improved',
        'Reached the daily water target earlier than usual. Afternoon energy level stayed more stable during work.',
        'https://images.unsplash.com/photo-1502741338009-cac2772e18bc',
        1,
        1,
        '2026-04-17T13:15:00+00:00',
        '2026-04-17T13:15:00+00:00',
        NULL
    ),
    (
        3,
        2,
        'Meal prep worked well',
        'Prepared lunch boxes for two days and avoided ordering takeout. Calories stayed within target without much effort.',
        'https://images.unsplash.com/photo-1512621776951-a57141f2eefd',
        2,
        2,
        '2026-04-17T20:00:00+00:00',
        '2026-04-17T20:00:00+00:00',
        NULL
    );

INSERT INTO columns (
    id,
    title,
    image_url,
    content,
    created_by,
    updated_by,
    created_at,
    updated_at,
    deleted_at
) VALUES
    (
        1,
        'How to build a sustainable morning routine',
        'https://images.unsplash.com/photo-1500530855697-b586d89ba3ee',
        'A strong morning routine starts with consistency, not intensity. Begin with hydration, light movement, and a breakfast that stabilizes energy for the first half of the day.',
        1,
        1,
        '2026-04-12T02:00:00+00:00',
        '2026-04-12T02:00:00+00:00',
        NULL
    ),
    (
        2,
        'Nutrition habits that reduce afternoon crashes',
        'https://images.unsplash.com/photo-1490645935967-10de6ba17061',
        'Balancing protein, fiber, and water intake can reduce the sharp dips in focus that often happen after lunch. Small changes in meal composition make a visible difference over time.',
        2,
        2,
        '2026-04-14T02:00:00+00:00',
        '2026-04-14T02:00:00+00:00',
        NULL
    ),
    (
        3,
        'Tracking body composition without obsessing over it',
        'https://images.unsplash.com/photo-1517836357463-d25dfeac3438',
        'Health tracking becomes useful when it reveals trends, not when it drives daily stress. Use weekly patterns in weight and body fat rate to guide decisions, then adjust slowly.',
        1,
        2,
        '2026-04-16T02:00:00+00:00',
        '2026-04-16T02:00:00+00:00',
        NULL
    );

SELECT setval('users_id_seq', COALESCE((SELECT MAX(id) FROM users), 1), TRUE);
SELECT setval('body_records_id_seq', COALESCE((SELECT MAX(id) FROM body_records), 1), TRUE);
SELECT setval('meals_id_seq', COALESCE((SELECT MAX(id) FROM meals), 1), TRUE);
SELECT setval('diaries_id_seq', COALESCE((SELECT MAX(id) FROM diaries), 1), TRUE);
SELECT setval('columns_id_seq', COALESCE((SELECT MAX(id) FROM columns), 1), TRUE);