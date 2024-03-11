WITH params(i, easting, northing) AS (
    VALUES (0, 441100, 493755), (1, 491250, 500100)
), params_minor(i, easting, northing) AS (
    SELECT i, easting % 500000, northing % 500000 FROM params
), find_major(i, major) AS (
    SELECT i AS i, CASE 
        WHEN easting >= 0       AND easting < 500000    AND northing >= 0       AND northing < 500000   THEN 'S' 
        WHEN easting >= 500000  AND easting < 1000000   AND northing >= 0       AND northing < 500000   THEN 'T' 
        WHEN easting >= 0       AND easting < 500000    AND northing >= 500000  AND northing < 1000000  THEN 'N' 
        WHEN easting >= 500000  AND easting < 1000000   AND northing >= 500000  AND northing < 1000000  THEN 'O'
        WHEN easting >= 0       AND easting < 500000    AND northing >= 1000000 AND northing < 1500000  THEN 'H'
        WHEN easting >= 500000  AND easting < 1000000   AND northing >= 1000000 AND northing < 1500000  THEN 'J'
    END AS major FROM params
), find_minor(i, minor) AS (
    SELECT i AS i, CASE 
        WHEN easting >= 0       AND easting < 100000     AND northing >= 0      AND northing < 100000   THEN 'V'
        WHEN easting >= 100000  AND easting < 200000     AND northing >= 0      AND northing < 100000   THEN 'W'
        WHEN easting >= 200000  AND easting < 300000     AND northing >= 0      AND northing < 100000   THEN 'X'
        WHEN easting >= 300000  AND easting < 400000     AND northing >= 0      AND northing < 100000   THEN 'Y'
        WHEN easting >= 400000  AND easting < 500000     AND northing >= 0      AND northing < 100000   THEN 'Z'
        WHEN easting >= 0       AND easting < 100000     AND northing >= 100000 AND northing < 200000   THEN 'Q'
        WHEN easting >= 100000  AND easting < 200000     AND northing >= 100000 AND northing < 200000   THEN 'R'
        WHEN easting >= 200000  AND easting < 300000     AND northing >= 100000 AND northing < 200000   THEN 'S'
        WHEN easting >= 300000  AND easting < 400000     AND northing >= 100000 AND northing < 200000   THEN 'T'
        WHEN easting >= 400000  AND easting < 500000     AND northing >= 100000 AND northing < 200000   THEN 'U'
        WHEN easting >= 0       AND easting < 100000     AND northing >= 200000 AND northing < 300000   THEN 'L'
        WHEN easting >= 100000  AND easting < 200000     AND northing >= 200000 AND northing < 300000   THEN 'M'
        WHEN easting >= 200000  AND easting < 300000     AND northing >= 200000 AND northing < 300000   THEN 'N'
        WHEN easting >= 300000  AND easting < 400000     AND northing >= 200000 AND northing < 300000   THEN 'O'
        WHEN easting >= 400000  AND easting < 500000     AND northing >= 200000 AND northing < 300000   THEN 'P'
        WHEN easting >= 0       AND easting < 100000     AND northing >= 300000 AND northing < 400000   THEN 'F'
        WHEN easting >= 100000  AND easting < 200000     AND northing >= 300000 AND northing < 400000   THEN 'G'
        WHEN easting >= 200000  AND easting < 300000     AND northing >= 300000 AND northing < 400000   THEN 'H'
        WHEN easting >= 300000  AND easting < 400000     AND northing >= 300000 AND northing < 400000   THEN 'J'
        WHEN easting >= 400000  AND easting < 500000     AND northing >= 300000 AND northing < 400000   THEN 'K'
        WHEN easting >= 0       AND easting < 100000     AND northing >= 400000 AND northing < 500000   THEN 'A'
        WHEN easting >= 100000  AND easting < 200000     AND northing >= 400000 AND northing < 500000   THEN 'B'
        WHEN easting >= 200000  AND easting < 300000     AND northing >= 400000 AND northing < 500000   THEN 'C'
        WHEN easting >= 300000  AND easting < 400000     AND northing >= 400000 AND northing < 500000   THEN 'D'
        WHEN easting >= 400000  AND easting < 500000     AND northing >= 400000 AND northing < 500000   THEN 'E'
    END AS minor FROM params_minor
), to_osgb36(i, osgb36) AS (
    SELECT p.i AS i, format('{:s}{:s}{:05}{:05}', mj.major, mn.minor, p.easting % 100000, p.northing % 100000) AS osgb36
    FROM params p, find_major mj, find_minor mn
    WHERE p.i = mj.i AND p.i = mn.i
)

SELECT * FROM to_osgb36
;
