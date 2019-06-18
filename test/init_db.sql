CREATE USER time_fix_stat_user WITH password 'time_fix_stat';
ALTER USER time_fix_stat_user WITH superuser;

CREATE DATABASE time_fix_stat
    WITH OWNER = time_fix_stat_user
    ENCODING = 'UTF8'
    TABLESPACE = pg_default
    LC_COLLATE = 'en_US.utf8'
    LC_CTYPE = 'en_US.utf8'
    CONNECTION LIMIT = -1;

\connect time_fix_stat;

CREATE SCHEMA IF NOT EXISTS time_fix_stat
    AUTHORIZATION time_fix_stat_user;

CREATE TABLE IF NOT EXISTS time_fix_stat.time_fix_stat
(
    id            SERIAL PRIMARY KEY,
    user_id       INTEGER NOT NULL,
    activity_name TEXT    NOT NULL,
    day           DATE    NOT NULL,
    minute        INTEGER NOT NULL
);