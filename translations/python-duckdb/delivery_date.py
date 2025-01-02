import duckdb
import ram_machine.prelude as ram


con = duckdb.connect()

# In RAM program:
# ?ReadyDate  => delta_ready_date
# ?ReadyDate' => new_ready_date
table_ddl = """
    CREATE OR REPLACE TABLE part_depends (part VARCHAR, component VARCHAR);
    CREATE OR REPLACE TABLE assembly_time (part VARCHAR, days INTEGER);
    CREATE OR REPLACE TABLE delivery_date (component VARCHAR, days INTEGER);
    CREATE OR REPLACE TABLE ready_date (part VARCHAR, days INTEGER);
    CREATE OR REPLACE TABLE delta_ready_date (part VARCHAR, days INTEGER);
    CREATE OR REPLACE TABLE new_ready_date (part VARCHAR, days INTEGER);
    CREATE OR REPLACE TABLE zresult (part VARCHAR, days INTEGER);
    CREATE OR REPLACE TABLE delta_zresult (part VARCHAR, days INTEGER);
    CREATE OR REPLACE TABLE new_zresult (part VARCHAR, days INTEGER);
"""
con.execute(table_ddl)

data_load = """
    INSERT INTO part_depends (part, component) VALUES 
        ('Car', 'Chassis'), 
        ('Car', 'Engine'), 
        ('Engine', 'Piston'), 
        ('Engine', 'Ignition');

    INSERT INTO assembly_time (part, days) VALUES 
        ('Car', 7), 
        ('Engine', 2);

    INSERT INTO delivery_date (component, days) VALUES 
        ('Chassis', 2), 
        ('Piston', 1), 
        ('Ignition', 7);
"""
con.execute(data_load)

# [23] ReadyDate(VarSym(part); VarSym(date)) :- DeliveryDate(VarSym(part); VarSym(date)).;
query = """
    INSERT INTO ready_date(part, days)
    SELECT 
        component AS part,
        days AS days,
    FROM delivery_date;
"""
con.execute(query)

# [27] ReadyDate(VarSym(part); <clo>(VarSym(componentDate), VarSym(assemblyTime))) :- PartDepends(VarSym(part), VarSym(component)), AssemblyTime(VarSym(part), VarSym(assemblyTime)), ReadyDate(VarSym(component); VarSym(componentDate)).;
query = """
    INSERT INTO ready_date(part, days)
    SELECT 
        t0.part AS part,
        max(t1.days + t2.days) AS days,
    FROM 
        part_depends t0,
        assembly_time t1,
        ready_date t2,
    WHERE t1.part = t0.part AND t2.part = t0.component
    GROUP BY t0.part
"""
con.execute(query)

# [35] merge ReadyDate into delta_ReadyDate;
ram.merge_into(con, src='ready_date', dest='delta_ready_date', cols=['part', 'days'])



delta_ready_date_empty = False
while not (delta_ready_date_empty):
    # [37] urge new_ready_date
    ram.purge_table(con, "new_ready_date")

    # [38] ReadyDate(VarSym(part); VarSym(date)) :- DeliveryDate(VarSym(part); VarSym(date)).;
    query = """
        INSERT INTO new_ready_date(part, days)
        SELECT 
            t0.component AS part,
            t0.days AS days,
        FROM delivery_date t0
        EXCEPT
        SELECT 
            t1.part AS part,
            t1.days AS days,
        FROM ready_date t1
    """
    con.execute(query)

    # [44] ReadyDate(VarSym(part); <clo>(VarSym(componentDate), VarSym(assemblyTime))) :- PartDepends(VarSym(part), VarSym(component)), AssemblyTime(VarSym(part), VarSym(assemblyTime)), ReadyDate(VarSym(component); VarSym(componentDate)).;
    query = """
        INSERT INTO new_ready_date(part, days)
        SELECT 
            t0.part AS part,
            max(t1.days + t2.days) AS days,
        FROM 
            part_depends t0,
            assembly_time t1,
            ready_date t2,
        WHERE t1.part = t0.part AND t2.part = t0.component
        GROUP BY t0.part
        EXCEPT
        SELECT 
            t3.part AS part,
            t3.days AS days,
        FROM ready_date t3
    """
    con.execute(query)

    # [54] merge new_ReadyDate into ReadyDate;
    ram.merge_into(con, src='new_ready_date', dest='ready_date', cols=['part', 'days'])
    
    # [55] delta_ReadyDate := new_ReadyDate
    ram.bind_table(con, left_table="delta_ready_date", right_table="new_ready_date", cols=['part', 'days'])

    delta_ready_date_empty = ram.table_is_empty(con, "delta_ready_date")
    print(f"empty_deltas: {delta_ready_date_empty}")


# [57] $Result(VarSym(c), VarSym(d)) :- fix ReadyDate(VarSym(c); VarSym(d)).;
query = """
    INSERT INTO zresult(part, days)
    SELECT 
        part AS part,
        days AS days,
    FROM ready_date;
"""
con.execute(query)

# [61] merge $Result into delta_$Result;
ram.merge_into(con, src='zresult', dest='delta_zresult', cols=['part', 'days'])



delta_zresult_empty = False
while not (delta_zresult_empty):

    # [63] purge new_$Result;
    ram.purge_table(con, "new_zresult")

    # [64] $Result(VarSym(c), VarSym(d)) :- fix ReadyDate(VarSym(c); VarSym(d)).;
    query = """
        INSERT INTO new_zresult(part, days)
        SELECT 
            t0.part AS part,
            t0.days AS days,
        FROM ready_date t0
        EXCEPT
        SELECT 
            t1.part AS part,
            t1.days AS days,
        FROM zresult t1
    """
    con.execute(query)

    # [70] merge new_$Result into $Result;
    ram.merge_into(con, src='new_zresult', dest='zresult', cols=['part', 'days'])
    
    # [71] delta_$Result := new_$Result
    ram.bind_table(con, left_table="delta_zresult", right_table="new_zresult", cols=['part', 'days'])
    

    delta_zresult_empty = ram.table_is_empty(con, "delta_zresult")
    print(f"empty_deltas: {delta_zresult_empty}")


print("zresult")
con.table("zresult").show()

print('^ Currently has an error - zresult should have single entry for each part')
con.close()
