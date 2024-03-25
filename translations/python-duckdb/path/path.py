# Figure 5 from On Fast Large-Scale Program Analysis in Datalog
# The translation is of the Flix version in this repo.

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
    

duckdb_path = 'e:/coding/python/fixpoint-experiment/translations/python-duckdb/path/path.duckdb'

con = duckdb.connect(database=duckdb_path, read_only=False)

con.execute("CREATE OR REPLACE TABLE edge (edge_from INTEGER, edge_to INTEGER);")
con.execute("CREATE OR REPLACE TABLE path (path_from INTEGER, path_to INTEGER, PRIMARY KEY(path_from, path_to));")
con.execute("CREATE OR REPLACE TABLE delta_path (path_from INTEGER, path_to INTEGER, PRIMARY KEY(path_from, path_to));")
con.execute("CREATE OR REPLACE TABLE new_path (path_from INTEGER, path_to INTEGER, PRIMARY KEY(path_from, path_to));")
con.execute("CREATE OR REPLACE TABLE zresult (path_from INTEGER, path_to INTEGER, PRIMARY KEY(path_from, path_to));")
con.execute("CREATE OR REPLACE TABLE delta_zresult (path_from INTEGER, path_to INTEGER, PRIMARY KEY(path_from, path_to));")
con.execute("CREATE OR REPLACE TABLE new_zresult (path_from INTEGER, path_to INTEGER, PRIMARY KEY(path_from, path_to));")

con.execute("INSERT INTO edge (edge_from, edge_to) VALUES (1, 2), (2, 3), (3, 4);")

# $Result(VarSym(x1), VarSym(x2)) :- Path(VarSym(x1), VarSym(x2)).;
query = """
    INSERT INTO zresult(path_from, path_to)
    SELECT 
        t0.path_from AS path_from,
        t0.path_to AS path_to,
    FROM path t0
    ON CONFLICT DO NOTHING;
"""
con.execute(query)

# Path(VarSym(x), VarSym(y)) :- Edge(VarSym(x), VarSym(y)).;
query = """
    INSERT INTO path(path_from, path_to)
    SELECT 
        t0.edge_from AS path_from,
        t0.edge_to AS path_to,
    FROM edge t0
    ON CONFLICT DO NOTHING;
"""
con.execute(query)

# Path(VarSym(x), VarSym(z)) :- Path(VarSym(x), VarSym(y)), Edge(VarSym(y), VarSym(z)).;
query = """
    INSERT INTO path(path_from, path_to)
    SELECT 
        t0.path_from AS path_from,
        t0.path_to AS path_to,
    FROM path t0
    JOIN edge t1 ON t1.edge_from == t0.path_to
    ON CONFLICT DO NOTHING;
"""
con.execute(query)

# merge $Result into delta_$Result;
con.execute("INSERT INTO delta_zresult (path_from, path_to) SELECT path_from, path_to FROM zresult ON CONFLICT DO NOTHING;")
# merge Path into delta_Path;
con.execute("INSERT INTO delta_path (path_from, path_to) SELECT path_from, path_to FROM path ON CONFLICT DO NOTHING;")


# loop - use a vacuous condition, actual condition tested for before the `break` statement
while True:
    # purge new_$Result;
    con.execute("DELETE FROM new_zresult;")
    # purge new_Path;
    con.execute("DELETE FROM new_path;")

    # $Result(VarSym(x1), VarSym(x2)) :- Path(VarSym(x1), VarSym(x2)).;
    query1 = """
        INSERT INTO new_zresult(path_from, path_to)
        SELECT 
            t0.path_from AS path_from,
            t0.path_to AS path_to,
        FROM delta_path t0
        WHERE NOT EXISTS (SELECT * FROM zresult s WHERE s.path_from = t0.path_from AND s.path_to = t0.path_to)
        ON CONFLICT DO NOTHING;
    """
    con.execute(query1)

    # Path(VarSym(x), VarSym(y)) :- Edge(VarSym(x), VarSym(y)).;
    # Path(VarSym(x), VarSym(z)) :- Path(VarSym(x), VarSym(y)), Edge(VarSym(y), VarSym(z)).;
    query1 = """
        INSERT INTO new_path(path_from, path_to)
        SELECT 
            t0.path_from AS path_from,
            t1.edge_to AS path_to,
        FROM delta_path t0
        JOIN edge t1 ON t1.edge_from = t0.path_to AND NOT EXISTS (SELECT * FROM path s WHERE s.path_from = t0.path_from AND s.path_to = t1.edge_to)
        ON CONFLICT DO NOTHING;
    """
    con.execute(query1)

    # merge new_$Result into $Result;
    con.execute("INSERT INTO zresult (path_from, path_to) SELECT path_from, path_to FROM new_zresult ON CONFLICT DO NOTHING;")
    # merge new_Path into Path;
    con.execute("INSERT INTO path (path_from, path_to) SELECT path_from, path_to FROM new_path ON CONFLICT DO NOTHING;")
    
    swap("delta_zresult",  "new_zresult", con=con);
    swap("delta_path", "new_path", con=con)

    delta_zresult_count = count_tuples("delta_zresult", con=con)
    delta_path_count = count_tuples("delta_path", con=con)
    print(f"loop - delta_zresult_count: {delta_zresult_count}, delta_path_count: {delta_path_count},")
    if delta_zresult_count <= 0 and delta_path_count <= 0:
        break


print("zresult")
con.table("zresult").show()

con.close()
