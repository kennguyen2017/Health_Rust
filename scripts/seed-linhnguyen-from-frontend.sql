BEGIN;

INSERT INTO users (
    email,
    full_name,
    password_hash,
    is_admin,
    is_verified,
    is_password_reset,
    login_attempts,
    created_at,
    updated_at
) VALUES (
    'quyengoldtimehn@gmail.com',
    'linhnguyen',
    '0000000000000000000000000000000000000000000000000000000000000000',
    FALSE,
    TRUE,
    FALSE,
    0,
    '2026-05-01T00:00:00+00:00',
    '2026-05-21T23:25:00+00:00'
)
ON CONFLICT (email) DO UPDATE
SET
    full_name = EXCLUDED.full_name,
    updated_at = EXCLUDED.updated_at,
    is_verified = EXCLUDED.is_verified;

DELETE FROM user_action_logs
WHERE user_id = (SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com');

DELETE FROM challenges
WHERE user_id = (SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com');

DELETE FROM exercise_records
WHERE user_id = (SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com');

DELETE FROM diaries
WHERE user_id = (SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com');

DELETE FROM meals
WHERE user_id = (SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com');

DELETE FROM month_charts
WHERE user_id = (SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com');

DELETE FROM week_charts
WHERE user_id = (SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com');

DELETE FROM day_charts
WHERE user_id = (SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com');

DELETE FROM year_charts
WHERE user_id = (SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com');

DELETE FROM body_records
WHERE user_id = (SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com');

DELETE FROM columns
WHERE created_by = (SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com');

INSERT INTO body_records (
    user_id,
    name,
    image_url,
    recorded_at,
    weight,
    body_fat_rate,
    created_by,
    updated_by,
    created_at,
    updated_at
)
SELECT
    user_id,
    name,
    image_url,
    recorded_at,
    weight,
    body_fat_rate,
    user_id,
    user_id,
    recorded_at,
    recorded_at
FROM (
    VALUES
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'Monthly body record 01', './画像/d01.jpg', '2026-01-21T07:00:00+00:00'::timestamptz, 66.0::numeric, 25.0::numeric),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'Monthly body record 02', './画像/d01.jpg', '2026-02-21T07:00:00+00:00'::timestamptz, 65.8::numeric, 24.9::numeric),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'Monthly body record 03', './画像/d01.jpg', '2026-03-21T07:00:00+00:00'::timestamptz, 65.5::numeric, 24.8::numeric),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'Monthly body record 04', './画像/d01.jpg', '2026-04-21T07:00:00+00:00'::timestamptz, 65.2::numeric, 24.7::numeric),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'Monthly body record 05', './画像/d01.jpg', '2026-05-21T07:00:00+00:00'::timestamptz, 65.1::numeric, 24.6::numeric),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'Monthly body record 06', './画像/d01.jpg', '2026-06-21T07:00:00+00:00'::timestamptz, 64.9::numeric, 24.4::numeric),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'Monthly body record 07', './画像/d01.jpg', '2026-07-21T07:00:00+00:00'::timestamptz, 64.8::numeric, 24.2::numeric),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'Monthly body record 08', './画像/d01.jpg', '2026-08-21T07:00:00+00:00'::timestamptz, 64.6::numeric, 24.0::numeric),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'Monthly body record 09', './画像/d01.jpg', '2026-09-21T07:00:00+00:00'::timestamptz, 64.7::numeric, 24.1::numeric),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'Monthly body record 10', './画像/d01.jpg', '2026-10-21T07:00:00+00:00'::timestamptz, 64.5::numeric, 23.9::numeric),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'Monthly body record 11', './画像/d01.jpg', '2026-11-21T07:00:00+00:00'::timestamptz, 64.4::numeric, 23.7::numeric),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'Monthly body record 12', './画像/d01.jpg', '2026-12-21T07:00:00+00:00'::timestamptz, 64.2::numeric, 23.5::numeric)
) AS body_data(user_id, name, image_url, recorded_at, weight, body_fat_rate);

INSERT INTO month_charts (user_id, name, recorded_at, weight, body_fat_rate)
SELECT
    user_id,
    name,
    recorded_at,
    weight,
    body_fat_rate
FROM (
    VALUES
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '1', '2026-01-21T07:00:00+00:00'::timestamptz, 66.0::numeric, 25.0::numeric),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '2', '2026-02-21T07:00:00+00:00'::timestamptz, 65.8::numeric, 24.9::numeric),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '3', '2026-03-21T07:00:00+00:00'::timestamptz, 65.5::numeric, 24.8::numeric),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '4', '2026-04-21T07:00:00+00:00'::timestamptz, 65.2::numeric, 24.7::numeric),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '5', '2026-05-21T07:00:00+00:00'::timestamptz, 65.1::numeric, 24.6::numeric),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '6', '2026-06-21T07:00:00+00:00'::timestamptz, 64.9::numeric, 24.4::numeric),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '7', '2026-07-21T07:00:00+00:00'::timestamptz, 64.8::numeric, 24.2::numeric),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '8', '2026-08-21T07:00:00+00:00'::timestamptz, 64.6::numeric, 24.0::numeric),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '9', '2026-09-21T07:00:00+00:00'::timestamptz, 64.7::numeric, 24.1::numeric),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '10', '2026-10-21T07:00:00+00:00'::timestamptz, 64.5::numeric, 23.9::numeric),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '11', '2026-11-21T07:00:00+00:00'::timestamptz, 64.4::numeric, 23.7::numeric),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '12', '2026-12-21T07:00:00+00:00'::timestamptz, 64.2::numeric, 23.5::numeric)
) AS month_data(user_id, name, recorded_at, weight, body_fat_rate);

INSERT INTO meals (
    user_id,
    name,
    calories,
    meal_type,
    eaten_at,
    image_url,
    created_by,
    updated_by,
    created_at,
    updated_at
)
SELECT
    user_id,
    name,
    calories,
    meal_type,
    eaten_at,
    image_url,
    user_id,
    user_id,
    eaten_at,
    eaten_at
FROM (
    VALUES
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'Morning meal', 380.0::numeric, 'morning', '2026-05-21T00:30:00+00:00'::timestamptz, './画像/m01.jpg'),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'Lunch meal', 520.0::numeric, 'lunch', '2026-05-21T03:00:00+00:00'::timestamptz, './画像/l03.jpg'),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'Dinner meal', 610.0::numeric, 'dinner', '2026-05-21T11:30:00+00:00'::timestamptz, './画像/d01.jpg'),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'Snack meal', 180.0::numeric, 'snack', '2026-05-21T08:30:00+00:00'::timestamptz, './画像/l01.jpg')
) AS meal_data(user_id, name, calories, meal_type, eaten_at, image_url);

INSERT INTO exercise_records (
    user_id,
    title,
    performed_at,
    exercise_type,
    calories,
    image_url,
    created_by,
    updated_by,
    created_at,
    updated_at
)
SELECT
    user_id,
    title,
    performed_at,
    exercise_type,
    calories,
    NULL,
    user_id,
    user_id,
    performed_at,
    performed_at
FROM (
    VALUES
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '家事全般（立位・軽い）', '2026-05-21T06:30:00+00:00'::timestamptz, '10 min', 26),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'ストレッチ', '2026-05-21T07:00:00+00:00'::timestamptz, '15 min', 18),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'ウォーキング', '2026-05-21T07:30:00+00:00'::timestamptz, '30 min', 72),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '筋トレ', '2026-05-21T08:00:00+00:00'::timestamptz, '20 min', 88),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'ジョギング', '2026-05-21T08:30:00+00:00'::timestamptz, '25 min', 160),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'ヨガ', '2026-05-21T09:00:00+00:00'::timestamptz, '20 min', 42),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '自転車', '2026-05-21T09:30:00+00:00'::timestamptz, '30 min', 92),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '踏み台昇降', '2026-05-21T10:00:00+00:00'::timestamptz, '15 min', 54),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'ダンス', '2026-05-21T10:30:00+00:00'::timestamptz, '20 min', 78),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '肩甲骨エクササイズ', '2026-05-21T11:00:00+00:00'::timestamptz, '10 min', 22),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'スクワット', '2026-05-21T11:30:00+00:00'::timestamptz, '8 min', 36),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '体幹トレーニング', '2026-05-21T12:00:00+00:00'::timestamptz, '12 min', 44)
) AS exercise_data(user_id, title, performed_at, exercise_type, calories);

INSERT INTO diaries (
    user_id,
    title,
    content,
    image_url,
    created_by,
    updated_by,
    created_at,
    updated_at
)
SELECT
    user_id,
    title,
    content,
    NULL,
    user_id,
    user_id,
    created_at,
    created_at
FROM (
    VALUES
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '2026.05.21 diary', '朝の散歩で気分が整った。食事は野菜中心で、夜は軽いストレッチを10分。', '2026-05-21T23:25:00+00:00'::timestamptz),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '2026.05.20 diary', '今日は昼食をサンドイッチにして、間食は控えめ。水分補給を意識できた。', '2026-05-20T22:10:00+00:00'::timestamptz),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '2026.05.19 diary', '仕事の合間に立ち上がる回数を増やした。夕食後に軽い体幹トレーニング。', '2026-05-19T21:42:00+00:00'::timestamptz),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '2026.05.18 diary', '体脂肪率が少し下がっていた。継続して無理のない範囲で運動したい。', '2026-05-18T23:00:00+00:00'::timestamptz),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '2026.05.17 diary', 'ランチを和食にしたら午後の集中が安定した。明日も塩分は控えめでいく。', '2026-05-17T22:20:00+00:00'::timestamptz),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '2026.05.16 diary', '週末なので歩数は多め。夜は軽くストレッチして睡眠の質を整えたい。', '2026-05-16T21:05:00+00:00'::timestamptz),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '2026.05.15 diary', '外食だったので朝と夜を調整。水を意識して飲めたのは良かった。', '2026-05-15T22:45:00+00:00'::timestamptz),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '2026.05.14 diary', '筋トレ後にたんぱく質をしっかり取れた。疲労感は少なく回復も順調。', '2026-05-14T23:12:00+00:00'::timestamptz)
) AS diary_data(user_id, title, content, created_at);

INSERT INTO columns (
    title,
    image_url,
    content,
    created_by,
    updated_by,
    created_at,
    updated_at
)
SELECT
    title,
    image_url,
    content,
    user_id,
    user_id,
    created_at,
    created_at
FROM (
    VALUES
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '魚を中心にした朝食のすすめ', './画像/column-1.jpg', 'Published at 2026.05.21 23:25. Tags: #魚 #朝ごはん', '2026-05-21T23:25:00+00:00'::timestamptz),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '睡眠の質を上げる3つの習慣', './画像/column-2.jpg', 'Published at 2026.05.20 22:10. Tags: #睡眠 #生活習慣', '2026-05-20T22:10:00+00:00'::timestamptz),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '彩り野菜で整える1日', './画像/column-3.jpg', 'Published at 2026.05.19 21:42. Tags: #野菜 #栄養', '2026-05-19T21:42:00+00:00'::timestamptz),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '短時間トレーニングの効果', './画像/column-4.jpg', 'Published at 2026.05.18 20:18. Tags: #運動 #時短', '2026-05-18T20:18:00+00:00'::timestamptz),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'ハーブティーでリラックス', './画像/column-5.jpg', 'Published at 2026.05.17 18:40. Tags: #リラックス #お茶', '2026-05-17T18:40:00+00:00'::timestamptz),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'ビタミンの上手な取り方', './画像/column-6.jpg', 'Published at 2026.05.16 18:00. Tags: #ビタミン #サプリ', '2026-05-16T18:00:00+00:00'::timestamptz),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'ジュースより果物を選ぶ理由', './画像/column-7.jpg', 'Published at 2026.05.15 17:25. Tags: #果物 #糖質', '2026-05-15T17:25:00+00:00'::timestamptz),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '和食プレートでバランス改善', './画像/column-8.jpg', 'Published at 2026.05.14 16:10. Tags: #和食 #バランス', '2026-05-14T16:10:00+00:00'::timestamptz),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '朝の散歩が代謝に効く理由', './画像/MyRecommend-1.jpg', 'Published at 2026.05.13 08:45. Tags: #散歩 #代謝', '2026-05-13T08:45:00+00:00'::timestamptz),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '自宅で続ける軽い筋トレ入門', './画像/MyRecommend-2.jpg', 'Published at 2026.05.12 19:15. Tags: #筋トレ #初心者', '2026-05-12T19:15:00+00:00'::timestamptz),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '日記習慣でメンタルを整える', './画像/MyRecommend-3.jpg', 'Published at 2026.05.11 21:00. Tags: #日記 #メンタル', '2026-05-11T21:00:00+00:00'::timestamptz),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'ランチの組み合わせを見直す', './画像/l01.jpg', 'Published at 2026.05.10 12:20. Tags: #ランチ #栄養', '2026-05-10T12:20:00+00:00'::timestamptz),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), 'サンドイッチでも栄養を取るコツ', './画像/l02.jpg', 'Published at 2026.05.09 09:50. Tags: #サンドイッチ #時短', '2026-05-09T09:50:00+00:00'::timestamptz),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '和朝食で塩分を抑える工夫', './画像/m01.jpg', 'Published at 2026.05.08 07:25. Tags: #朝食 #減塩', '2026-05-08T07:25:00+00:00'::timestamptz),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '食べ過ぎた翌日の調整方法', './画像/m02.jpg', 'Published at 2026.05.07 08:10. Tags: #調整 #習慣', '2026-05-07T08:10:00+00:00'::timestamptz),
        ((SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'), '軽い有酸素運動の始め方', './画像/column-4.jpg', 'Published at 2026.05.06 18:35. Tags: #有酸素 #運動', '2026-05-06T18:35:00+00:00'::timestamptz)
) AS column_data(user_id, title, image_url, content, created_at);

INSERT INTO user_action_logs (
    user_id,
    action,
    data,
    options,
    status,
    created_by,
    updated_by,
    created_at,
    updated_at
) VALUES (
    (SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'),
    'top_page_summary',
    'Seeded from frontend topData.js',
    jsonb_build_object(
        'date', '05/21',
        'achievement_rate', 75,
        'burned_kcal', 230,
        'exercise_minutes', 60
    ),
    'seeded',
    (SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'),
    (SELECT id FROM users WHERE email = 'quyengoldtimehn@gmail.com'),
    '2026-05-21T23:25:00+00:00',
    '2026-05-21T23:25:00+00:00'
);

COMMIT;