WITH RECURSIVE paths(path_from, path_to) AS (
    SELECT 
        b1.edge_from AS path_from,
        b1.edge_to AS path_to,
    FROM edge AS b1
UNION ALL
    SELECT 
        t1.edge_from AS path_from,
        t2.path_to AS path_to,
    FROM edge AS t1
    JOIN paths t2 ON t1.edge_to = t2.path_from
)
SELECT path_from, path_to 
FROM paths
