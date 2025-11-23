-- friend_suggestions.sql

CREATE SCHEMA IF NOT EXISTS friend_suggestions;

CREATE OR REPLACE TABLE friend_suggestions.friend (
    person VARCHAR NOT NULL, 
    friend VARCHAR NOT NULL,
);

CREATE OR REPLACE MACRO noequal3(f1, f2, f3) AS (f1 != f2 AND f2 != f3 AND f1 != f3);


DELETE FROM friend_suggestions.friend;
INSERT INTO friend_suggestions.friend (person, friend) 
    VALUES 
        ('George', 'Antonio'), ('George', 'Sarah'), ('George', 'Roberto'), 
        ('Sarah', 'Hisham'), ('Antonio', 'Hisham'), ('Roberto', 'Hisham');

-- Must have three friends that have your not-yet friend as a friend
-- The RAM program doesn't formulate a fixpoint (except for Result which we ignore)

WITH cte(person, newfriend) AS (
    SELECT 
        t0.person AS person,
        t3.friend AS newfriend,
    FROM friend_suggestions.friend t0
    JOIN friend_suggestions.friend t1 ON t1.person = t0.person
    JOIN friend_suggestions.friend t2 ON t2.person = t0.person AND notequal3(t1.friend, t0.friend, t2.friend)
    JOIN friend_suggestions.friend t3 ON t3.person = t0.friend AND NOT EXISTS (SELECT * FROM friend_suggestions.friend s4 WHERE s4.person = t0.person AND s4.friend = t3.friend)
    JOIN friend_suggestions.friend t4 ON t4.person = t1.friend AND t4.friend = t3.friend
    JOIN friend_suggestions.friend t5 ON t5.person = t2.friend AND t5.friend = t3.friend
)
SELECT DISTINCT ON (person, newfriend) * FROM cte;

    