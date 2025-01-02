# A "design rule" checker to assert a pump system has a child equipment pump
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

data_load = """
    INSERT INTO system (floc, ty, parent) VALUES ('WDC01-WT-SYS01', 'SPMS', 'WCD01-WT');
    INSERT INTO sub_system(floc, ty, parent) VALUES('WDC01-WT-SYS01-KIS01', 'KISK', 'WDC01-WT-SYS01');
    INSERT INTO pump(floc, name) VALUES ('ZAN01-WT-SYS02-PMP01', 'Auto Pump-1');
"""

con.execute(data_load)

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
merge_into(con, src='has_pump', dest='delta_has_pump', cols=['floc'])

delta_has_pump_empty = False
while not (delta_has_pump_empty):
    # purge new_HasPump;
    purge_table(con, "new_has_pump")

    # merge new_HasPump into HasPump;
    merge_into(con, src='new_has_pump', dest='has_pump', cols=['floc'])

    # delta_HasPump := new_HasPump
    bind_table(con, left_table="delta_has_pump", right_table="new_has_pump", cols=['floc'])
    

    delta_has_pump_empty = table_is_empty(con, "delta_has_pump")
    print(f"empty_deltas: {delta_has_pump_empty}")

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
merge_into(con, src='zresult', dest='delta_zresult', cols=['floc'])

# merge NoPump into delta_NoPump;
merge_into(con, src='no_pump', dest='delta_no_pump', cols=['floc'])


delta_zresult_empty, delta_no_pump_empty = False, False
while not (delta_zresult_empty and delta_no_pump_empty):
    # purge new_$Result;
    purge_table(con, "new_zresult")

    # purge new_NoPump;
    purge_table(con, "new_no_pump")

    # $Result(VarSym(x1)) :- NoPump(VarSym(x1)).;
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
    merge_into(con, src='new_zresult', dest='zresult', cols=['floc'])
    

    # merge new_NoPump into NoPump;
    merge_into(con, src='new_no_pump', dest='no_pump', cols=['floc'])

    # delta_$Result := new_$Result;
    bind_table(con, left_table="delta_zresult", right_table="new_zresult", cols=['floc'])

    # delta_NoPump := new_NoPump
    bind_table(con, left_table="delta_no_pump", right_table="new_no_pump", cols=['floc'])
    

    delta_zresult_empty = table_is_empty(con, "delta_zresult")
    delta_no_pump_empty = table_is_empty(con, "delta_no_pump")
    print(f"empty_deltas: {delta_zresult_empty}, {delta_no_pump_empty}")

print("zresult")
con.table("zresult").show()

con.close()
