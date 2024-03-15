CREATE OR REPLACE TABLE road (
    road_from VARCHAR,
    max_speed INTEGER,
    road_to VARCHAR,    
)
;

INSERT INTO road(road_from, max_speed, road_to)
VALUES 
    ('Rome', 80, 'Turin'),  
    ('Turin', 70, 'Naples'), 
    ('Naples', 50, 'Florence')
;


CREATE OR REPLACE MACRO min_speed() AS (SELECT 30);

WITH RECURSIVE steps(road_from, max_speed, road_to) AS (
    SELECT 
        b1.road_from AS road_from,
        b1.max_speed AS max_speed,
        b1.road_to AS road_to,
    FROM road AS b1
    WHERE b1.max_speed > min_speed()
UNION ALL
    SELECT 
        t1.road_from AS path_from,
        least(t1.max_speed, t2.max_speed) AS max_speed,
        t2.road_to AS path_to,
    FROM road AS t1
    JOIN steps t2 ON t1.road_to = t2.road_from
)
SELECT COUNT(*)
FROM steps
WHERE road_from = 'Rome' and road_to = 'Florence'

