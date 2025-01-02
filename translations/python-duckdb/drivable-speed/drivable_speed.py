import os
import duckdb

# merge and purge are no clearer than using SQL directly

def swap(con: duckdb.DuckDBPyConnection, table1: str, table2: str) -> None:
    table_swap = f"{table1}_swap"
    con.execute(f"ALTER TABLE {table1} RENAME TO {table_swap};")
    con.execute(f"ALTER TABLE {table2} RENAME TO {table1};")
    con.execute(f"ALTER TABLE {table_swap} RENAME TO {table2};")


def table_is_empty(con: duckdb.DuckDBPyConnection, table: str) -> bool:
    query = f"SELECT count(1) WHERE EXISTS (SELECT * FROM {table});"
    ans1 = con.execute(query).fetchone()
    return (ans1[0] == 0)
    

con = duckdb.connect()

# param... 45 is drivable, 55 isn't
drivable_speed = 55

print(f"Is drivable at {drivable_speed}? ... check result table has an entry:")

# Define our own unit type
table_ddl = f"""
    DROP TYPE IF EXISTS unit;
    CREATE TYPE unit AS ENUM ('unit');
    DROP MACRO IF EXISTS arg_speed;
    DROP MACRO IF EXISTS pred1;
    CREATE MACRO arg_speed() AS (SELECT {drivable_speed});
    CREATE MACRO pred1(mph) AS (mph >= arg_speed());

    CREATE OR REPLACE TABLE road (source VARCHAR, max_speed INTEGER, destination VARCHAR);
    CREATE OR REPLACE TABLE path (source VARCHAR, destination VARCHAR, PRIMARY KEY(source, destination));
    CREATE OR REPLACE TABLE delta_path (source VARCHAR, destination VARCHAR, PRIMARY KEY(source, destination));
    CREATE OR REPLACE TABLE new_path (source VARCHAR, destination VARCHAR, PRIMARY KEY(source, destination));
    CREATE OR REPLACE TABLE zresult (result unit, PRIMARY KEY(result));
    CREATE OR REPLACE TABLE delta_zresult (result unit, PRIMARY KEY(result));
    CREATE OR REPLACE TABLE new_zresult (result unit, PRIMARY KEY(result));
"""
con.execute(table_ddl)

data_load = """
    INSERT INTO road (source, max_speed, destination) VALUES 
        ('Rome', 80, 'Turin'), 
        ('Turin', 70, 'Naples'), 
        ('Naples', 50, 'Florence');
"""
con.execute(data_load)

# [11] $Result(BoxedObject(((), Obj -> Obj))) :- Path(BoxedObject((Rome, Obj -> Obj)), BoxedObject((Florence, Obj -> Obj))).;
query = """
    INSERT INTO zresult(result)
    SELECT 
        'unit' AS result
    FROM road t0
    WHERE t0.source = 'Rome'
    AND t0.destination = 'Florence'
    ON CONFLICT DO NOTHING;
"""
con.execute(query)

# [15] Path(VarSym(x), VarSym(y)) :- Road(VarSym(x), VarSym(maximumSpeed), VarSym(y)), <clo>(VarSym(maximumSpeed)).;
query = """
    INSERT INTO path(source, destination)
    SELECT 
        t0.source AS source,
        t0.destination AS destination,
    FROM road t0
    WHERE pred1(t0.max_speed)
    ON CONFLICT DO NOTHING;
"""
con.execute(query)

# [21] Path(VarSym(x), VarSym(z)) :- Path(VarSym(x), VarSym(y)), Road(VarSym(y), VarSym(maximumSpeed), VarSym(z)), <clo>(VarSym(maximumSpeed)).;
query = """
    INSERT INTO path(source, destination)
    SELECT 
        t0.source AS source,
        t1.destination AS destination,
    FROM path t0
    JOIN road t1 ON t1.source = t0.destination AND pred1(t1.max_speed)
    ON CONFLICT DO NOTHING;
"""
con.execute(query)

# [29] merge $Result into delta_$Result;
con.execute("INSERT INTO delta_zresult (result) SELECT result FROM zresult ON CONFLICT DO NOTHING;")
# [30] merge Path into delta_Path;
con.execute("INSERT INTO delta_path (source, destination) SELECT source, destination FROM path ON CONFLICT DO NOTHING;")


delta_zresult_empty, delta_path_empty = False, False
while not (delta_zresult_empty and delta_path_empty):
    # [32] purge new_$Result;
    con.execute("DELETE FROM new_zresult;")
    # [33] purge new_Path;
    con.execute("DELETE FROM new_path;")

    # [34] $Result(BoxedObject(((), Obj -> Obj))) :- Path(BoxedObject((Rome, Obj -> Obj)), BoxedObject((Florence, Obj -> Obj))).;
    query = """
        INSERT INTO new_zresult(result) 
        SELECT 
            'unit' AS result,
        FROM path t0
        WHERE NOT EXISTS (SELECT * FROM zresult)
        AND t0.source == 'Rome'
        AND t0.destination == 'Florence'
        ON CONFLICT DO NOTHING;
    """
    con.execute(query)

    # [40] Path(VarSym(x), VarSym(y)) :- Road(VarSym(x), VarSym(maximumSpeed), VarSym(y)), <clo>(VarSym(maximumSpeed)).;
    # [41] Path(VarSym(x), VarSym(z)) :- Path(VarSym(x), VarSym(y)), Road(VarSym(y), VarSym(maximumSpeed), VarSym(z)), <clo>(VarSym(maximumSpeed)).;
    query = """
        INSERT INTO new_path(source, destination) 
        SELECT 
            t0.source AS source,
            t1.destination AS destination,
        FROM delta_path t0
        JOIN road t1 ON t1.SOURCE = t0.destination AND pred1(t1.max_speed) AND NOT EXISTS (SELECT * FROM path s0 WHERE s0.source = t0.source AND s0.destination = t1.destination)
        ON CONFLICT DO NOTHING;
    """
    con.execute(query)

    # [49] merge new_$Result into $Result;
    con.execute("INSERT INTO zresult (result) SELECT result FROM new_zresult ON CONFLICT DO NOTHING;")
    # [50] merge new_Path into Path;
    con.execute("INSERT INTO path (source, destination) SELECT source, destination FROM new_path ON CONFLICT DO NOTHING;")
    # [51] delta_$Result := new_$Result;
    swap(con, "delta_zresult", "new_zresult")
    # [52] delta_Path := new_Path
    swap(con, "delta_path", "new_path")

    delta_zresult_empty = table_is_empty(con, "delta_zresult")
    delta_path_empty = table_is_empty(con, "delta_path")
    print(f"empty_deltas: {delta_zresult_empty}, {delta_path_empty}")



print("zresult")
con.table("zresult").show()

con.close()
