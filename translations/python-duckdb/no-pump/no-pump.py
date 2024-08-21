# A "design rule" checker to assert a pump system has a child equipment pump

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
    

duckdb_path = 'e:/coding/projects/fixpoint-experiment/translations/python-duckdb/no-pump/no-pump.duckdb'

con = duckdb.connect(database=duckdb_path, read_only=False)

table_ddl = """
    CREATE OR REPLACE TABLE system (floc VARCHAR, ty VARCHAR, parent VARCHAR);
    CREATE OR REPLACE TABLE sub_system (floc VARCHAR, ty VARCHAR, parent VARCHAR);
    CREATE OR REPLACE TABLE pump (floc VARCHAR, name VARCHAR);
    CREATE OR REPLACE TABLE has_pump (floc VARCHAR);
    CREATE OR REPLACE TABLE no_pump (floc VARCHAR);
    CREATE OR REPLACE TABLE zresult (floc VARCHAR, PRIMARY KEY(floc));
    CREATE OR REPLACE TABLE delta_zresult (floc VARCHAR, PRIMARY KEY(floc));
    CREATE OR REPLACE TABLE new_zresult (floc VARCHAR, PRIMARY KEY(floc));
"""

con.execute(table_ddl)

tables_load = """
        INSERT INTO system (floc, ty, parent) VALUES ('WDC01-WT-SYS01', 'SPMS', 'WCD01-WT');
        INSERT INTO sub_system(floc, ty, parent) VALUES('WDC01-WT-SYS01-KIS01', 'KISK', 'WDC01-WT-SYS01');
        INSERT INTO pump(floc, name) VALUES ('ZAN01-WT-SYS02-PMP01', 'Auto Pump-1');
"""

con.execute(tables_load)

# HasPump(VarSym(floc)) :- System(VarSym(floc), BoxedObject((SPMS, Obj -> Obj)), _), SubSystem(VarSym(ssfloc), BoxedObject((PUMP, Obj -> Obj)), VarSym(floc)), Pump(VarSym(ssfloc), _).;
query_HasPump1 = """
    INSERT INTO has_pump(floc)
    SELECT DISTINCT
        t0.floc AS floc,
    FROM system t0
    JOIN sub_system t1 ON t1.parent = t0.floc AND t1.ty = 'PUMP'
    JOIN pump t2 ON t2.floc = t1.floc
    WHERE t0.ty = 'SPMS';
"""
con.execute(query_HasPump1)

# # Path(VarSym(x), VarSym(y)) :- Edge(VarSym(x), VarSym(y)).;
# query = """
#     INSERT INTO path(path_from, path_to)
#     SELECT 
#         t0.edge_from AS path_from,
#         t0.edge_to AS path_to,
#     FROM edge t0
#     ON CONFLICT DO NOTHING;
# """
# con.execute(query)

# # Path(VarSym(x), VarSym(z)) :- Path(VarSym(x), VarSym(y)), Edge(VarSym(y), VarSym(z)).;
# query = """
#     INSERT INTO path(path_from, path_to)
#     SELECT 
#         t0.path_from AS path_from,
#         t0.path_to AS path_to,
#     FROM path t0
#     JOIN edge t1 ON t1.edge_from == t0.path_to
#     ON CONFLICT DO NOTHING;
# """
# con.execute(query)

# # merge $Result into delta_$Result;
# con.execute("INSERT INTO delta_zresult (path_from, path_to) SELECT path_from, path_to FROM zresult ON CONFLICT DO NOTHING;")
# # merge Path into delta_Path;
# con.execute("INSERT INTO delta_path (path_from, path_to) SELECT path_from, path_to FROM path ON CONFLICT DO NOTHING;")


# # loop - use a vacuous condition, actual condition tested for before the `break` statement
# while True:
#     # purge new_$Result;
#     con.execute("DELETE FROM new_zresult;")
#     # purge new_Path;
#     con.execute("DELETE FROM new_path;")

#     # $Result(VarSym(x1), VarSym(x2)) :- Path(VarSym(x1), VarSym(x2)).;
#     query1 = """
#         INSERT INTO new_zresult(path_from, path_to)
#         SELECT 
#             t0.path_from AS path_from,
#             t0.path_to AS path_to,
#         FROM delta_path t0
#         WHERE NOT EXISTS (SELECT * FROM zresult s WHERE s.path_from = t0.path_from AND s.path_to = t0.path_to)
#         ON CONFLICT DO NOTHING;
#     """
#     con.execute(query1)

#     # Path(VarSym(x), VarSym(y)) :- Edge(VarSym(x), VarSym(y)).;
#     # Path(VarSym(x), VarSym(z)) :- Path(VarSym(x), VarSym(y)), Edge(VarSym(y), VarSym(z)).;
#     query1 = """
#         INSERT INTO new_path(path_from, path_to)
#         SELECT 
#             t0.path_from AS path_from,
#             t1.edge_to AS path_to,
#         FROM delta_path t0
#         JOIN edge t1 ON t1.edge_from = t0.path_to AND NOT EXISTS (SELECT * FROM path s WHERE s.path_from = t0.path_from AND s.path_to = t1.edge_to)
#         ON CONFLICT DO NOTHING;
#     """
#     con.execute(query1)

#     # merge new_$Result into $Result;
#     con.execute("INSERT INTO zresult (path_from, path_to) SELECT path_from, path_to FROM new_zresult ON CONFLICT DO NOTHING;")
#     # merge new_Path into Path;
#     con.execute("INSERT INTO path (path_from, path_to) SELECT path_from, path_to FROM new_path ON CONFLICT DO NOTHING;")
    
#     swap("delta_zresult",  "new_zresult", con=con);
#     swap("delta_path", "new_path", con=con)

#     delta_zresult_count = count_tuples("delta_zresult", con=con)
#     delta_path_count = count_tuples("delta_path", con=con)
#     print(f"loop - delta_zresult_count: {delta_zresult_count}, delta_path_count: {delta_path_count},")
#     if delta_zresult_count <= 0 and delta_path_count <= 0:
#         break


print("zresult")
con.table("zresult").show()

con.close()
