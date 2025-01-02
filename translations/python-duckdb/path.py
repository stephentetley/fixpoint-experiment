# Figure 5 from On Fast Large-Scale Program Analysis in Datalog
# The translation is of the Flix version in this repo.

import duckdb
import ram_machine.prelude as ram


con = duckdb.connect()

table_ddl = """
    CREATE OR REPLACE TABLE edge (edge_from INTEGER, edge_to INTEGER);
    CREATE OR REPLACE TABLE path (path_from INTEGER, path_to INTEGER);
    CREATE OR REPLACE TABLE delta_path (path_from INTEGER, path_to INTEGER);
    CREATE OR REPLACE TABLE new_path (path_from INTEGER, path_to INTEGER);
    CREATE OR REPLACE TABLE zresult (path_from INTEGER, path_to INTEGER);
    CREATE OR REPLACE TABLE delta_zresult (path_from INTEGER, path_to INTEGER);
    CREATE OR REPLACE TABLE new_zresult (path_from INTEGER, path_to INTEGER);
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
    FROM path t0;
"""
con.execute(query)

# [15] Path(VarSym(x), VarSym(y)) :- Edge(VarSym(x), VarSym(y)).;
query = """
    INSERT INTO path(path_from, path_to)
    SELECT
        t0.edge_from AS path_from,
        t0.edge_to AS path_to,
    FROM edge t0;
"""
con.execute(query)

# [19] Path(VarSym(x), VarSym(z)) :- Path(VarSym(x), VarSym(y)), Edge(VarSym(y), VarSym(z)).;
query = """
    INSERT INTO path(path_from, path_to)
    SELECT
        t0.path_from AS path_from,
        t1.edge_to AS path_to,
    FROM path t0
    JOIN edge t1 ON t1.edge_from == t0.path_to;
"""
con.execute(query)

# [25] merge $Result into delta_$Result;
ram.merge_into(con, src='zresult', dest='delta_zresult', cols=["path_from", "path_to"])

# [26] merge Path into delta_Path;
ram.merge_into(con, src='path', dest='delta_path', cols=["path_from", "path_to"])


delta_zresult_empty, delta_path_empty = False, False
while not (delta_zresult_empty and delta_path_empty):
    # [29] purge new_$Result;
    ram.purge_table(con, "new_zresult")
    # [29] purge new_Path;
    ram.purge_table(con, "new_path")

    # [30] $Result(VarSym(x1), VarSym(x2)) :- Path(VarSym(x1), VarSym(x2)).;
    query1 = """
        INSERT INTO new_zresult(path_from, path_to)
        SELECT 
            t0.path_from AS path_from,
            t0.path_to AS path_to,
        FROM delta_path t0
        ANTI JOIN zresult USING (path_from, path_to);
    """
    con.execute(query1)

    # [36] Path(VarSym(x), VarSym(y)) :- Edge(VarSym(x), VarSym(y)).;
    # [37] Path(VarSym(x), VarSym(z)) :- Path(VarSym(x), VarSym(y)), Edge(VarSym(y), VarSym(z)).;
    query1 = """
        INSERT INTO new_path(path_from, path_to)
        SELECT 
            t0.path_from AS path_from,
            t1.edge_to AS path_to,
        FROM delta_path t0
        JOIN edge t1 ON t1.edge_from = t0.path_to 
        ANTI JOIN path t2 ON (t2.path_from = t0.path_from AND t2.path_to = t1.edge_to);
    """
    con.execute(query1)

    # [45] merge new_$Result into $Result;
    ram.merge_into(con, src='new_zresult', dest='zresult', cols=["path_from", "path_to"])

    # [46] merge new_Path into Path;
    ram.merge_into(con, src='new_path', dest='path', cols=["path_from", "path_to"])

    # [47] delta_$Result := new_$Result;
    ram.bind_table(con, left_table="delta_zresult", right_table="new_zresult", cols=["path_from", "path_to"])

    # [48] delta_Path := new_Path
    ram.bind_table(con, left_table="delta_path", right_table="new_path", cols=["path_from", "path_to"])


    delta_zresult_empty = ram.table_is_empty(con, "delta_zresult")
    delta_path_empty = ram.table_is_empty(con, "delta_path")
    print(f"empty_deltas: {delta_zresult_empty} {delta_path_empty}")


print("zresult")
con.table("zresult").show()

con.close()
