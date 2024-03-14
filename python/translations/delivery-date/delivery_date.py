
import duckdb





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

con.execute(f"INSERT INTO delta_ready_date (part, days) SELECT part, days FROM ready_date;")


# loop - use a vacuous condition, actual condition tested for before the `break` statement
while True:
    print("until...")
    print("delta_ready_date")
    con.table("delta_ready_date").show()
    print("assembly_time")
    con.table("assembly_time").show()
    print("delivery_date")
    con.table("delivery_date").show()
    print("part_depends")
    con.table("part_depends").show()
    print("ready_date")
    con.table("ready_date").show()


    con.execute(f"DELETE FROM new_ready_date;")

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
    print("new_ready_date")
    con.table("new_ready_date").show()

    con.execute("INSERT INTO ready_date (part, days) SELECT part, days FROM new_ready_date ON CONFLICT DO UPDATE SET days = EXCLUDED.days;")
    con.execute("ALTER TABLE new_ready_date RENAME TO new_ready_date_swap;")
    con.execute("ALTER TABLE delta_ready_date RENAME TO new_ready_date;")
    con.execute("ALTER TABLE new_ready_date_swap RENAME TO delta_ready_date;")

    count = 0
    ans1 = con.execute("SELECT COUNT(*) FROM delta_ready_date;").fetchone()
    if ans1 is not None:
        count = ans1[0]
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
con.execute("INSERT INTO delta_result (part, days) SELECT part, days FROM result ON CONFLICT DO UPDATE SET days = EXCLUDED.days;")

while True:

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


    con.execute("INSERT INTO result (part, days) SELECT part, days FROM new_result ON CONFLICT DO UPDATE SET days = EXCLUDED.days;")
    con.execute("ALTER TABLE new_result RENAME TO new_result_swap;")
    con.execute("ALTER TABLE delta_result RENAME TO new_result;")
    con.execute("ALTER TABLE new_result_swap RENAME TO delta_result;")

    count = 0
    ans1 = con.execute("SELECT COUNT(*) FROM delta_result;").fetchone()
    if ans1 is not None:
        count = ans1[0]
    print(f"loop - count: {count}")
    if count <= 0:
        break


print("result")
con.table("result").show()


con.close()
