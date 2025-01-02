
import duckdb

# merge - src and dest columns must have the same names
def merge_into(con, *, src: str, dest:str, cols: list[str]) -> None:
    columns = ", ".join(cols)
    query = f"""
        INSERT INTO {dest}({columns})
        SELECT {columns} 
        FROM {src}
        ANTI JOIN {dest} USING({columns})
    """
    con.execute(query)

def purge_table(con: duckdb.DuckDBPyConnection, table: str) -> bool:
    query = f"DELETE FROM {table};"
    con.execute(query)

def bind_table(con: duckdb.DuckDBPyConnection, left_table: str, right_table: str, cols: list[str]) -> None:
    query = f"DELETE FROM {left_table};"
    con.execute(query)
    columns = ", ".join(cols)
    query = f"""
        INSERT INTO {left_table}({columns})
        SELECT {columns} 
        FROM {right_table};
    """
    con.execute(query)


def table_is_empty(con: duckdb.DuckDBPyConnection, table: str) -> bool:
    query = f"SELECT count(1) WHERE EXISTS (SELECT * FROM {table});"
    ans1 = con.execute(query).fetchone()
    return (ans1[0] == 0)
    

con = duckdb.connect()

# param... 45 is drivable, 55 isn't
drivable_speed = 45

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
merge_into(con, src='zresult', dest='delta_zresult', cols=['result'])

# [30] merge Path into delta_Path;
merge_into(con, src='path', dest='delta_path', cols=['source', 'destination'])


delta_zresult_empty, delta_path_empty = False, False
while not (delta_zresult_empty and delta_path_empty):
    # [32] purge new_$Result;
    purge_table(con, "new_zresult")
    # [33] purge new_Path;
    purge_table(con, "new_path")

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
    merge_into(con, src='new_zresult', dest='zresult', cols=['result'])

    # [50] merge new_Path into Path;
    merge_into(con, src='new_path', dest='path', cols=['source', 'destination'])
    

    # [51] delta_$Result := new_$Result;
    bind_table(con, left_table="delta_zresult", right_table="new_zresult", cols=['result'])
    # [52] delta_Path := new_Path
    bind_table(con, left_table="delta_path", right_table="new_path", cols=['source', 'destination'])

    delta_zresult_empty = table_is_empty(con, "delta_zresult")
    delta_path_empty = table_is_empty(con, "delta_path")
    print(f"empty_deltas: {delta_zresult_empty}, {delta_path_empty}")



print("zresult")
con.table("zresult").show()

con.close()
