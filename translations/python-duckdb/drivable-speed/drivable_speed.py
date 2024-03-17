
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
con.execute("DROP TYPE unit;")
con.execute("CREATE TYPE unit AS ENUM ('unit');")
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
    WHERE t0.max_speed > 30
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
    WHERE t1.max_speed > 30
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

    # TODO 
    break

print("zresult")
con.table("zresult").show()

con.close()
