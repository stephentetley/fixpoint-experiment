
import duckdb

# merge and purge are no clearer than using SQL directly

def swap(table1: str, table2: str, *, con: duckdb.DuckDBPyConnection) -> None:
    table_swap = f"{table1}_swap"
    con.execute(f"ALTER TABLE {table1} RENAME TO {table_swap};")
    con.execute(f"ALTER TABLE {table2} RENAME TO {table1};")
    con.execute(f"ALTER TABLE {table_swap} RENAME TO {table2};")


def count_tuples(table: str, *, con: duckdb.DuckDBPyConnection) -> int:
    ans1 = con.execute(f"SELECT COUNT(*) FROM {table};").fetchone()
    if ans1 is None:
        return 0
    else:
        return ans1[0]
    

duckdb_path = 'e:/coding/python/fixpoint-experiment/translations/python-duckdb/drivable-speed/drivable-speed.duckdb'


con = duckdb.connect(database=duckdb_path, read_only=False)

# Define our own unit type
con.execute("DROP TYPE IF EXISTS unit;")
con.execute("CREATE TYPE unit AS ENUM ('unit');")
con.execute("DROP MACRO IF EXISTS predicate1;")
con.execute("CREATE MACRO predicate1(mph) AS (mph >= 51);")

con.execute("CREATE OR REPLACE TABLE road (source VARCHAR, max_speed INTEGER, destination VARCHAR);")
con.execute("CREATE OR REPLACE TABLE path (source VARCHAR, destination VARCHAR, PRIMARY KEY(source, destination));")
con.execute("CREATE OR REPLACE TABLE delta_path (source VARCHAR, destination VARCHAR, PRIMARY KEY(source, destination));")
con.execute("CREATE OR REPLACE TABLE new_path (source VARCHAR, destination VARCHAR, PRIMARY KEY(source, destination));")
con.execute("CREATE OR REPLACE TABLE zresult (result unit, PRIMARY KEY(result));")
con.execute("CREATE OR REPLACE TABLE delta_zresult (result unit, PRIMARY KEY(result));")
con.execute("CREATE OR REPLACE TABLE new_zresult (result unit, PRIMARY KEY(result));")

con.execute("INSERT INTO road (source, max_speed, destination) VALUES ('Rome', 80, 'Turin'), ('Turin', 70, 'Naples'), ('Naples', 50, 'Florence');")

# $Result(BoxedObject(((), Obj -> Obj))) :- Path(BoxedObject((Rome, Obj -> Obj)), BoxedObject((Florence, Obj -> Obj))).;
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

# Path(VarSym(x), VarSym(y)) :- Road(VarSym(x), VarSym(maximumSpeed), VarSym(y)), <clo>(VarSym(maximumSpeed)).;
query = """
    INSERT INTO path(source, destination)
    SELECT 
        t0.source AS source,
        t0.destination AS destination,
    FROM road t0
    WHERE predicate1(t0.max_speed)
    ON CONFLICT DO NOTHING;
"""
con.execute(query)

# Path(VarSym(x), VarSym(z)) :- Path(VarSym(x), VarSym(y)), Road(VarSym(y), VarSym(maximumSpeed), VarSym(z)), <clo>(VarSym(maximumSpeed)).;
query = """
    INSERT INTO path(source, destination)
    SELECT 
        t0.source AS source,
        t0.destination AS destination,
    FROM path t0
    JOIN road t1 ON t1.source = t0.destination
    WHERE predicate1(t1.max_speed)
    ON CONFLICT DO NOTHING;
"""
con.execute(query)

# merge $Result into delta_$Result;
con.execute("INSERT INTO delta_zresult (result) SELECT result FROM zresult ON CONFLICT DO NOTHING;")
# merge Path into delta_Path;
con.execute("INSERT INTO delta_path (source, destination) SELECT source, destination FROM path ON CONFLICT DO NOTHING;")



while True:
    # purge new_$Result;
    con.execute("DELETE FROM new_zresult;")
    # purge new_Path;
    con.execute("DELETE FROM new_path;")

    # $Result(BoxedObject(((), Obj -> Obj))) :- Path(BoxedObject((Rome, Obj -> Obj)), BoxedObject((Florence, Obj -> Obj))).;
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

    # Path(VarSym(x), VarSym(y)) :- Road(VarSym(x), VarSym(maximumSpeed), VarSym(y)), <clo>(VarSym(maximumSpeed)).;
    # Path(VarSym(x), VarSym(z)) :- Path(VarSym(x), VarSym(y)), Road(VarSym(y), VarSym(maximumSpeed), VarSym(z)), <clo>(VarSym(maximumSpeed)).;
    query = """
        INSERT INTO new_path(source, destination) 
        SELECT 
            t0.source AS source,
            t1.destination AS destination,
        FROM delta_path t0
        JOIN road t1 ON t1.SOURCE = t0.destination
        WHERE predicate1(t1.max_speed) AND NOT EXISTS (SELECT * FROM path t2 WHERE t2.source = t0.source AND t2.destination = t1.destination)
        ON CONFLICT DO NOTHING;
    """
    con.execute(query)

    # merge new_$Result into $Result;
    con.execute("INSERT INTO zresult (result) SELECT result FROM new_zresult ON CONFLICT DO NOTHING;")
    # merge new_Path into Path;
    con.execute("INSERT INTO path (source, destination) SELECT source, destination FROM new_path ON CONFLICT DO NOTHING;")
    swap("delta_zresult", "new_zresult", con=con)
    swap("delta_path", "new_path", con=con)

    delta_zresult_count = count_tuples("delta_zresult", con=con)
    delta_path_count = count_tuples("delta_path", con=con)
    print(f"loop - delta_zresult_count: {delta_zresult_count}, delta_path_count: {delta_path_count},")
    if delta_zresult_count <= 0 and delta_path_count <= 0:
        break


print("zresult")
con.table("zresult").show()

con.close()
