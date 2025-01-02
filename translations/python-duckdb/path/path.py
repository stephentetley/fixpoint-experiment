# Figure 5 from On Fast Large-Scale Program Analysis in Datalog
# The translation is of the Flix version in this repo.

import os
import duckdb


# merge is no clearer than using SQL directly

def purge_table(con: duckdb.DuckDBPyConnection, table: str) -> bool:
    query = f"DELETE FROM {table};"
    con.execute(query)

def swap_tables(con: duckdb.DuckDBPyConnection, table1: str, table2: str) -> None:
    table_swap = f"{table1}_swap"
    con.execute(f"ALTER TABLE {table1} RENAME TO {table_swap};")
    con.execute(f"ALTER TABLE {table2} RENAME TO {table1};")
    con.execute(f"ALTER TABLE {table_swap} RENAME TO {table2};")

def table_is_empty(con: duckdb.DuckDBPyConnection, table: str) -> bool:
    query = f"SELECT count(1) WHERE EXISTS (SELECT * FROM {table});"
    ans1 = con.execute(query).fetchone()
    return (ans1[0] == 0)
    

con = duckdb.connect()

table_ddl = """
    CREATE OR REPLACE TABLE edge (edge_from INTEGER, edge_to INTEGER);
    CREATE OR REPLACE TABLE path (path_from INTEGER, path_to INTEGER, PRIMARY KEY(path_from, path_to));
    CREATE OR REPLACE TABLE delta_path (path_from INTEGER, path_to INTEGER, PRIMARY KEY(path_from, path_to));
    CREATE OR REPLACE TABLE new_path (path_from INTEGER, path_to INTEGER, PRIMARY KEY(path_from, path_to));
    CREATE OR REPLACE TABLE zresult (path_from INTEGER, path_to INTEGER, PRIMARY KEY(path_from, path_to));
    CREATE OR REPLACE TABLE delta_zresult (path_from INTEGER, path_to INTEGER, PRIMARY KEY(path_from, path_to));
    CREATE OR REPLACE TABLE new_zresult (path_from INTEGER, path_to INTEGER, PRIMARY KEY(path_from, path_to));
"""
con.execute(table_ddl)

# [5,7,9]
data_load = """
    INSERT INTO edge (edge_from, edge_to) VALUES (1, 2), (2, 3), (3, 4);
"""
con.execute(data_load)

# [11] $Result(VarSym(x1), VarSym(x2)) :- Path(VarSym(x1), VarSym(x2)).;
query = """
    INSERT INTO zresult(path_from, path_to)
    SELECT
        t0.path_from AS path_from,
        t0.path_to AS path_to,
    FROM path t0
    ON CONFLICT DO NOTHING;
"""
con.execute(query)

# [15] Path(VarSym(x), VarSym(y)) :- Edge(VarSym(x), VarSym(y)).;
query = """
    INSERT INTO path(path_from, path_to)
    SELECT
        t0.edge_from AS path_from,
        t0.edge_to AS path_to,
    FROM edge t0
    ON CONFLICT DO NOTHING;
"""
con.execute(query)

# [19] Path(VarSym(x), VarSym(z)) :- Path(VarSym(x), VarSym(y)), Edge(VarSym(y), VarSym(z)).;
query = """
    INSERT INTO path(path_from, path_to)
    SELECT
        t0.path_from AS path_from,
        t1.edge_to AS path_to,
    FROM path t0
    JOIN edge t1 ON t1.edge_from == t0.path_to
    ON CONFLICT DO NOTHING;
"""
con.execute(query)

# merge $Result into delta_$Result;
con.execute("INSERT INTO delta_zresult (path_from, path_to) SELECT path_from, path_to FROM zresult ON CONFLICT DO NOTHING;")
# merge Path into delta_Path;
con.execute("INSERT INTO delta_path (path_from, path_to) SELECT path_from, path_to FROM path ON CONFLICT DO NOTHING;")


delta_zresult_empty, delta_path_empty = False, False
while not (delta_zresult_empty and delta_path_empty):
    # purge new_$Result;
    purge_table(con, "new_zresult")
    # purge new_Path;
    purge_table(con, "new_path")

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
    
    swap_tables(con, "delta_zresult",  "new_zresult");
    swap_tables(con, "delta_path", "new_path")

    delta_zresult_empty = table_is_empty(con, "delta_zresult")
    delta_path_empty = table_is_empty(con, "delta_path")
    print(f"empty_deltas: {delta_zresult_empty} {delta_path_empty}")


print("zresult")
con.table("zresult").show()

con.close()
