CREATE SCHEMA IF NOT EXISTS delivery_date;

CREATE OR REPLACE TABLE delivery_date.part_depends (
    part VARCHAR NOT NULL,
    component VARCHAR NOT NULL,
);


CREATE OR REPLACE TABLE delivery_date.assembly_time (
    part VARCHAR NOT NULL, 
    days INTEGER NOT NULL,
);


CREATE OR REPLACE TABLE delivery_date.delivery_date (
    component VARCHAR NOT NULL, 
    days INTEGER NOT NULL,
);


CREATE OR REPLACE TABLE delivery_date.ready_date (
    part VARCHAR NOT NULL, 
    days INTEGER NOT NULL, 
    PRIMARY KEY (part)
);
    

DELETE FROM delivery_date.part_depends;
INSERT INTO delivery_date.part_depends(part, component)
VALUES 
    ('Car', 'Chassis'), 
    ('Car', 'Engine'), 
    ('Engine', 'Piston'), 
    ('Engine', 'Ignition');

DELETE FROM delivery_date.assembly_time;
INSERT INTO delivery_date.assembly_time (part, days) VALUES 
    ('Car', 7), 
    ('Engine', 2);

DELETE FROM delivery_date.delivery_date;
INSERT INTO delivery_date.delivery_date (component, days) VALUES 
    ('Chassis', 2), 
    ('Piston', 1), 
    ('Ignition', 7);

DELETE FROM delivery_date.ready_date;
INSERT INTO delivery_date.ready_date
(SELECT 
    component AS part,
    days AS days,
FROM delivery_date.delivery_date)
UNION 
(    SELECT 
        t0.part AS part,
        max(t1.days + t2.days) AS days,
    FROM 
        delivery_date.part_depends t0
    JOIN delivery_date.assembly_time t1 ON t1.part = t0.part
    JOIN delivery_date.ready_date t2 ON t2.part = t0.component
    GROUP BY t0.part);



WITH RECURSIVE cte(part, days) USING KEY (part) AS (
(SELECT 
    component AS part,
    days AS days,
FROM delivery_date.delivery_date
)
UNION
(SELECT 
    t0.part AS part,
    max(t1.days + t2.days) AS days,
FROM 
    delivery_date.part_depends t0
JOIN delivery_date.assembly_time t1 ON t1.part = t0.part
JOIN delivery_date.ready_date t2 ON t2.part = t0.component
GROUP BY t0.part
))
SELECT part, days
FROM cte
