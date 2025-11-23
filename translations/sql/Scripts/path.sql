CREATE SCHEMA IF NOT EXISTS paths;

CREATE OR REPLACE TABLE paths.edge (
    edge_from INTEGER NOT NULL,
    edge_to INTEGER NOT NULL,
);

DELETE FROM paths.edge;
INSERT INTO paths.edge(edge_from, edge_to)
VALUES 
    (1, 2), (2, 3), (3, 4);


WITH RECURSIVE cte(path_from, path_to) USING KEY (path_from, path_to) AS (
    SELECT 
        b1.edge_from AS path_from,
        b1.edge_to AS path_to,
    FROM paths.edge AS b1
UNION
    SELECT 
        t1.edge_from AS path_from,
        t2.path_to AS path_to,
    FROM paths.edge AS t1
    JOIN cte t2 ON t1.edge_to = t2.path_from
)
SELECT path_from, path_to 
FROM cte
