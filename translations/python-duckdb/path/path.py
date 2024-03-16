# Figure 5 from On Fast Large-Scale Program Analysis in Datalog

import duckdb


duckdb_path = 'e:/coding/python/fixpoint-experiment/translations/python-duckdb/path/path.duckdb'


def swap(table1: str, table2: str, *, con: duckdb.DuckDBPyConnection) -> None:
    table_swap = f"{table1}_swap"
    con.execute(f"ALTER TABLE {table1} RENAME TO {table_swap};")
    con.execute(f"ALTER TABLE {table2} RENAME TO {table1};")
    con.execute(f"ALTER TABLE {table_swap} RENAME TO {table2};")

def purge(table: str, *, con: duckdb.DuckDBPyConnection) -> None:
    con.execute(f"DELETE FROM {table};")

def merge_into(table_from: str, table_into:str, columns: list[str], *, con: duckdb.DuckDBPyConnection) -> None:
    cols = ", ".join(columns)
    con.execute(f"INSERT INTO {table_into} ({cols}) SELECT {cols} FROM {table_from};")

def count_tuples(table: str, *, con: duckdb.DuckDBPyConnection) -> int:
    ans1 = con.execute(f"SELECT COUNT(*) FROM {table};").fetchone()
    if ans1 is None:
        return 0
    else:
        return ans1[0]
    


con = duckdb.connect(database=duckdb_path, read_only=False)

con.execute("CREATE OR REPLACE TABLE edge (edge_from INTEGER, edge_to INTEGER);")
con.execute("CREATE OR REPLACE TABLE path (path_from INTEGER, path_to INTEGER);")
con.execute("CREATE OR REPLACE TABLE delta_path (path_from INTEGER, path_to INTEGER);")
con.execute("CREATE OR REPLACE TABLE new_path (path_from INTEGER, path_to INTEGER);")

path1_sql = """
    INSERT INTO new_path (path_from, path_to) 
    WITH cte1 AS (SELECT 
            t1.edge_from AS path_from,
            t2.path_to AS path_to,
        FROM edge AS t1
        JOIN delta_path t2 ON t1.edge_to = t2.path_from)
    SELECT path_from, path_to FROM cte1
    WHERE NOT EXISTS (
        SELECT path_from, path_to
        FROM path AS t2
        WHERE t2.path_from = cte1.path_from AND t2.path_to = cte1.path_to
    )
"""

con.execute("INSERT INTO edge (edge_from, edge_to) VALUES (1, 2), (2, 3), (3, 4);")
print("edge")
con.table("edge").show()

# project_into
con.execute("INSERT INTO path (path_from, path_to) SELECT edge_from, edge_to FROM edge;")
print("path")
con.table("path").show()


merge_into("path", "delta_path", ["path_from", "path_to"], con=con)


# loop - use a vacuous condition, actual condition tested for before the `break` statement
while True:
    # purge
    purge("new_path", con=con)


    con.execute(path1_sql)
    # print("new_path")
    # con.table("new_path").show()


    tcount = count_tuples("new_path", con=con)
    print(f"count_tuples: {tcount}")
    if tcount == 0: 
        break
    
    merge_into("new_path", "path", ["path_from", "path_to"], con=con)
    swap("new_path", "delta_path", con=con)


print("Done - path:")
con.table("path").show()

con.close()
