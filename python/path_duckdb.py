# Figure 5 from On Fast Large-Scale Program Analysis in Datalog
# use conda::datafusion-env [Ctrl+Shift+P then `Python: Select Interpreter`]

import duckdb


duckdb_path = 'e:/coding/python/fixpoint-experiment/python/path.duckdb'


def swap(t1: str, t2: str, *, con: duckdb.DuckDBPyConnection) -> None:
    con.execute(f"ALTER TABLE {t1} RENAME TO {t1}_swap;")
    con.execute(f"ALTER TABLE {t2} RENAME TO {t1};")
    con.execute(f"ALTER TABLE {t1}_swap RENAME TO {t2};")


con = duckdb.connect(database=duckdb_path, read_only=False)

con.execute("CREATE OR REPLACE TABLE edge (edge_from INTEGER, edge_to INTEGER);")
con.execute("CREATE OR REPLACE TABLE path (path_from INTEGER, path_to INTEGER, PRIMARY KEY(path_from, path_to));")
con.execute("CREATE OR REPLACE TABLE delta_path (path_from INTEGER, path_to INTEGER, PRIMARY KEY(path_from, path_to));")
con.execute("CREATE OR REPLACE TABLE new_path (path_from INTEGER, path_to INTEGER, PRIMARY KEY(path_from, path_to));")
con.execute("CREATE OR REPLACE TABLE path_join (path_from INTEGER, path_to INTEGER);")


con.execute("INSERT INTO edge (edge_from, edge_to) VALUES (1, 2), (2, 3), (3, 4);")
print("edge")
con.table("edge").show()

# project_into
con.execute("INSERT INTO path (path_from, path_to) SELECT edge_from, edge_to FROM edge;")
print("path")
con.table("path").show()

# merge_into(path, delta_path) 
con.execute("INSERT INTO delta_path (path_from, path_to) SELECT path_from, path_to FROM path;")
print("delta_path")
con.table("delta_path").show()


# loop - use a vacuous condition, actual condition tested for `break`
one_hundred = 100
while one_hundred > 0:
    # purge
    con.execute("DELETE FROM new_path;")


    con.execute("DELETE FROM path_join;")
    con.execute("INSERT INTO path_join (path_from, path_to) SELECT l.edge_from AS path_from, r.path_to AS path_to FROM edge l JOIN delta_path r ON l.edge_to = r.path_from;")
    con.execute("INSERT OR IGNORE INTO new_path (path_from, path_to) SELECT l.path_from, l.path_to FROM path_join l;")


    sc = con.execute("SELECT COUNT(*) FROM new_path;").fetchone()
    if sc is None:
        count_tuples = 0
    else:
        count_tuples = sc[0]
    print("count_tuples: {}".format(count_tuples))

    if count_tuples == 0: 
        break
    
    # merge_into(new_path, path)
    con.execute("INSERT INTO path (path_from, path_to) SELECT path_from, path_to FROM new_path;")
    
    # swap(new_path, delta_path)
    swap("new_path", "delta_path", con=con)


print("Done - path:")
con.table("path").show()

con.close()
