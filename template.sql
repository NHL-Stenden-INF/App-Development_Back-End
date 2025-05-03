DROP TABLE IF EXISTS users;
CREATE TABLE users
(
    id       INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name     TEXT,
    email    TEXT,
    password TEXT,
    points   INTEGER DEFAULT 0
);

DROP TABLE IF EXISTS games;
CREATE TABLE games
(
    id      INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name    TEXT,
    rewards INTEGER,
    data    BLOB
);

DROP TABLE IF EXISTS friends;
CREATE TABLE friends
(
    id        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id_1 INTEGER,
    user_id_2 INTEGER,
    FOREIGN KEY (user_id_1) REFERENCES users(id),
    FOREIGN KEY (user_id_2) REFERENCES users(id)
);

DROP TABLE IF EXISTS user_completed_games;
CREATE TABLE user_completed_games
(
    id            INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    game_id       INTEGER,
    user_id       INTEGER,
    is_completed  INTEGER(1),
    FOREIGN KEY (game_id) REFERENCES  games(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

