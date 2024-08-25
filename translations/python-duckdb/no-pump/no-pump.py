# A "design rule" checker to assert a pump system has a child equipment pump
import os
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
    

dir_path = os.path.dirname(os.path.realpath(__file__))
duckdb_path = os.path.normpath(os.path.join(dir_path, 'data-no-pump.duckdb'))

con = duckdb.connect(database=duckdb_path, read_only=False)

table_ddl = """
    CREATE OR REPLACE TABLE system (floc VARCHAR, ty VARCHAR, parent VARCHAR);
    CREATE OR REPLACE TABLE sub_system (floc VARCHAR, ty VARCHAR, parent VARCHAR);
    CREATE OR REPLACE TABLE pump (floc VARCHAR, name VARCHAR);
    CREATE OR REPLACE TABLE has_pump (floc VARCHAR, PRIMARY KEY(floc));
    CREATE OR REPLACE TABLE delta_has_pump (floc VARCHAR, PRIMARY KEY(floc));
    CREATE OR REPLACE TABLE new_has_pump (floc VARCHAR, PRIMARY KEY(floc));
    CREATE OR REPLACE TABLE no_pump (floc VARCHAR, PRIMARY KEY(floc));
    CREATE OR REPLACE TABLE delta_no_pump (floc VARCHAR, PRIMARY KEY(floc));
    CREATE OR REPLACE TABLE new_no_pump (floc VARCHAR, PRIMARY KEY(floc));
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
project_into_HasPump1 = """
    INSERT INTO has_pump(floc)
    SELECT DISTINCT
        t0.floc AS floc,
    FROM system t0
    JOIN sub_system t1 ON t1.parent = t0.floc AND t1.ty = 'PUMP'
    JOIN pump t2 ON t2.floc = t1.floc
    WHERE t0.ty = 'SPMS';
"""
con.execute(project_into_HasPump1)

#  merge HasPump into delta_HasPump;
con.execute("INSERT INTO delta_has_pump (floc) SELECT floc FROM has_pump ON CONFLICT DO NOTHING;")

while True:
    # purge new_HasPump;
    con.execute("DELETE FROM new_has_pump;")

    # merge new_HasPump into HasPump;
    con.execute("INSERT INTO has_pump (floc) SELECT floc FROM new_has_pump ON CONFLICT DO NOTHING;")

    # delta_HasPump := new_HasPump
    swap("new_has_pump", "delta_has_pump", con=con)
    

    delta_has_pump_count = count_tuples("delta_has_pump", con=con)
    print(f"loop - delta_has_pump_count: {delta_has_pump_count},")
    if delta_has_pump_count <= 0:
        break

# $Result(VarSym(x1)) :- NoPump(VarSym(x1)).;
project_into_zresult1 = """
    INSERT INTO zresult(floc)
    SELECT DISTINCT
        t0.floc AS floc,
    FROM no_pump t0;
"""
con.execute(project_into_zresult1)

# NoPump(VarSym(floc)) :- System(VarSym(floc), BoxedObject((SPMS, Obj -> Obj)), _), not HasPump(VarSym(floc)).;
project_into_no_pump1 = """
    INSERT INTO no_pump(floc)
    SELECT DISTINCT
        t0.floc AS floc,
    FROM system t0
    WHERE NOT EXISTS (SELECT * FROM has_pump s0 WHERE s0.floc = t0.floc)
    AND t0.ty = 'SPMS';
"""
con.execute(project_into_no_pump1)

# merge $Result into delta_$Result;
con.execute("INSERT INTO delta_zresult (floc) SELECT floc FROM zresult ON CONFLICT DO NOTHING;")

# merge NoPump into delta_NoPump;
con.execute("INSERT INTO delta_no_pump (floc) SELECT floc FROM no_pump ON CONFLICT DO NOTHING;")

while True:
    # purge new_$Result;
    con.execute("DELETE FROM new_zresult;")

    # purge new_NoPump;
    con.execute("DELETE FROM new_no_pump;")

    project_into_new_zresult1 = """
        INSERT INTO new_zresult(floc)
        SELECT 
            t0.floc AS floc,
        FROM delta_no_pump t0
        WHERE NOT EXISTS (SELECT * FROM zresult s0 WHERE s0.floc = t0.floc)
        """
    con.execute(project_into_new_zresult1)

    # NoPump(VarSym(floc)) :- System(VarSym(floc), BoxedObject((SPMS, Obj -> Obj)), _), not HasPump(VarSym(floc)).;
    # merge new_$Result into $Result;
    con.execute("INSERT INTO zresult (floc) SELECT floc FROM new_zresult ON CONFLICT DO NOTHING;")

    # merge new_NoPump into NoPump;
    con.execute("INSERT INTO no_pump (floc) SELECT floc FROM new_no_pump ON CONFLICT DO NOTHING;")

    # delta_$Result := new_$Result;
    swap("new_zresult", "delta_zresult", con=con)

    # delta_NoPump := new_NoPump
    swap("new_no_pump", "delta_no_pump", con=con)
    

    delta_zresult_count = count_tuples("delta_zresult", con=con)
    delta_no_pump_count = count_tuples("delta_no_pump", con=con)
    print(f"loop - delta_zresult_count: {delta_zresult_count}, delta_no_pump_count: {delta_no_pump_count}")
    if delta_zresult_count <= 0 or delta_no_pump_count <= 0:
        break

print("zresult")
con.table("zresult").show()

con.close()
