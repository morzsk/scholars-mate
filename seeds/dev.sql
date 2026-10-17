INSERT INTO users (username, score)
VALUES
    ('Iroh', 10000),
    ('Aang', 9000),
    ('Zuko', 8000),
    ('Toph', 7000),
    ('Katara', 6000),
    ('Sokka', 5000),
    ('Azula', 4000),
    ('Suki', 3000),
    ('Appa', 2000),
    ('Momo', 1000)
ON CONFLICT (username) DO NOTHING;
