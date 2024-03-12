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

WITH params(i, gridref) AS (
    VALUES (0, 'SE4110093755'), (1, 'NZ9125000100')
), params_split(i, major, minor, east_lower, north_lower) AS (
    SELECT 
        i AS i,
        gridref[1] AS major, 
        gridref[2] AS minor, 
        TRY_CAST(gridref[3:7] AS INTEGER) AS east_lower,
        TRY_CAST(gridref[8:12] AS INTEGER) AS north_lower,
    FROM params
    
), decode_major(i, major_en) AS (
    SELECT i AS i, CASE 
        WHEN major = 'S' THEN {'easting': 0,        'northing': 0}
        WHEN major = 'T' THEN {'easting': 500000,   'northing': 0}
        WHEN major = 'N' THEN {'easting': 0,        'northing': 500000}
        WHEN major = 'O' THEN {'easting': 500000,   'northing': 500000}
        WHEN major = 'H' THEN {'easting': 0,        'northing': 1000000}
    END AS major_en FROM params_split        
), decode_minor(i, minor_en) AS (
    SELECT i AS i, CASE     
        WHEN minor = 'A' THEN {'easting': 0,        'northing': 400000}
        WHEN minor = 'B' THEN {'easting': 100000,   'northing': 400000}
        WHEN minor = 'C' THEN {'easting': 200000,   'northing': 400000}
        WHEN minor = 'D' THEN {'easting': 300000,   'northing': 400000}
        WHEN minor = 'E' THEN {'easting': 400000,   'northing': 400000}
        WHEN minor = 'F' THEN {'easting': 0,        'northing': 300000}
        WHEN minor = 'G' THEN {'easting': 100000,   'northing': 300000}
        WHEN minor = 'H' THEN {'easting': 200000,   'northing': 300000}
        WHEN minor = 'J' THEN {'easting': 300000,   'northing': 300000}
        WHEN minor = 'K' THEN {'easting': 400000,   'northing': 300000}
        WHEN minor = 'L' THEN {'easting': 0,        'northing': 200000}
        WHEN minor = 'M' THEN {'easting': 100000,   'northing': 200000}
        WHEN minor = 'N' THEN {'easting': 200000,   'northing': 200000}
        WHEN minor = 'O' THEN {'easting': 300000,   'northing': 200000}
        WHEN minor = 'P' THEN {'easting': 400000,   'northing': 200000}
        WHEN minor = 'Q' THEN {'easting': 0,        'northing': 100000}
        WHEN minor = 'R' THEN {'easting': 100000,   'northing': 100000}
        WHEN minor = 'S' THEN {'easting': 200000,   'northing': 100000}
        WHEN minor = 'T' THEN {'easting': 300000,   'northing': 100000}
        WHEN minor = 'U' THEN {'easting': 400000,   'northing': 100000}
        WHEN minor = 'V' THEN {'easting': 0,        'northing': 0}
        WHEN minor = 'W' THEN {'easting': 100000,   'northing': 0}
        WHEN minor = 'X' THEN {'easting': 200000,   'northing': 0}
        WHEN minor = 'Y' THEN {'easting': 300000,   'northing': 0}
        WHEN minor = 'Z' THEN {'easting': 400000,   'northing': 0}
    END AS minor_en FROM params_split        
), to_east_north(i, easting, northing) AS (
    SELECT 
        p.i AS i, 
        mj.major_en.easting + mn.minor_en.easting + p.east_lower AS easting, 
        mj.major_en.northing + mn.minor_en.northing + p.north_lower AS northing,
    FROM params_split p, decode_major mj, decode_minor mn
    WHERE p.i = mj.i AND p.i = mn.i
)
SELECT * FROM to_east_north
;