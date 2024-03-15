
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
    

duckdb_path = 'e:/coding/python/fixpoint-experiment/python/translations/delivery-date/delivery-date.duckdb'


con = duckdb.connect(database=duckdb_path, read_only=False)

# In RAM program:
# ?ReadyDate  => delta_ready_date
# ?ReadyDate' => new_ready_date

con.execute("CREATE OR REPLACE TABLE part_depends (part VARCHAR, component VARCHAR);")
con.execute("CREATE OR REPLACE TABLE assembly_time (part VARCHAR, days INTEGER);")
con.execute("CREATE OR REPLACE TABLE delivery_date (component VARCHAR, days INTEGER);")
con.execute("CREATE OR REPLACE TABLE ready_date (part VARCHAR, days INTEGER, PRIMARY KEY(part));")
con.execute("CREATE OR REPLACE TABLE delta_ready_date (part VARCHAR, days INTEGER, PRIMARY KEY(part));")
con.execute("CREATE OR REPLACE TABLE new_ready_date (part VARCHAR, days INTEGER, PRIMARY KEY(part));")
con.execute("CREATE OR REPLACE TABLE result (part VARCHAR, days INTEGER, PRIMARY KEY(part));")
con.execute("CREATE OR REPLACE TABLE delta_result (part VARCHAR, days INTEGER, PRIMARY KEY(part));")
con.execute("CREATE OR REPLACE TABLE new_result (part VARCHAR, days INTEGER, PRIMARY KEY(part));")


con.execute("INSERT INTO part_depends (part, component) VALUES ('Car', 'Chassis'), ('Car', 'Engine'), ('Engine', 'Piston'), ('Engine', 'Ignition');")
con.execute("INSERT INTO assembly_time (part, DAYS) VALUES ('Car', 7), ('Engine', 2);")
con.execute("INSERT INTO delivery_date (component, DAYS) VALUES ('Chassis', 2), ('Piston', 1), ('Ignition', 7);")

# ReadyDate(VarSym(part); VarSym(date)) :- DeliveryDate(VarSym(part); VarSym(date)).;
query = """
    INSERT INTO ready_date(part, days)
    SELECT 
        component AS part,
        days AS days,
    FROM delivery_date;
"""
con.execute(query)

# ReadyDate(VarSym(part); <clo>(VarSym(componentDate), VarSym(assemblyTime))) :- PartDepends(VarSym(part), VarSym(component)), AssemblyTime(VarSym(part), VarSym(assemblyTime)), ReadyDate(VarSym(component); VarSym(componentDate)).;
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

con.execute("INSERT INTO delta_ready_date (part, days) SELECT part, days FROM ready_date;")


# loop - use a vacuous condition, actual condition tested for before the `break` statement
while True:
    # purge new_ready_date
    con.execute("DELETE FROM new_ready_date;")

    # ReadyDate(VarSym(part); VarSym(date)) :- DeliveryDate(VarSym(part); VarSym(date)).;
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

    con.execute("INSERT INTO ready_date (part, days) SELECT part, days FROM new_ready_date ON CONFLICT DO UPDATE SET days = EXCLUDED.days;")
    swap("new_ready_date", "delta_ready_date", con=con)

    count = count_tuples("delta_ready_date", con=con)
    print(f"loop - count: {count}")
    if count <= 0:
        break

# T calc result...
query = """
    INSERT INTO result(part, days)
    SELECT 
        part AS part,
        days AS days,
    FROM ready_date;
"""
con.execute(query)
# merge
con.execute("INSERT INTO delta_result (part, days) SELECT part, days FROM result ON CONFLICT DO UPDATE SET days = EXCLUDED.days;")


while True:

    # purge
    con.execute(f"DELETE FROM new_result;")


    query = """
        INSERT INTO new_result(part, days)
        SELECT 
            t0.part AS part,
            t0.days AS days,
        FROM ready_date t0
        EXCEPT
        SELECT 
            t1.part AS part,
            t1.days AS days,
        FROM result t1
    """
    con.execute(query)

    # merge
    con.execute("INSERT INTO result (part, days) SELECT part, days FROM new_result ON CONFLICT DO UPDATE SET days = EXCLUDED.days;")
    swap("new_result", "delta_result", con=con)

    count = count_tuples("delta_result", con=con)
    print(f"loop - count: {count}")
    if count <= 0:
        break


print("result")
con.table("result").show()


con.close()
