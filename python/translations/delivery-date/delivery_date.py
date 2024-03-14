
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


con.execute("INSERT INTO part_depends (part, component) VALUES ('Car', 'Chassis'), ('Car', 'Engine'), ('Engine', 'Piston'), ('Engine', 'Ignition');")

con.execute("INSERT INTO assembly_time (part, DAYS) VALUES ('Car', 7), ('Engine', 2);")


con.execute("INSERT INTO delivery_date (component, DAYS) VALUES ('Chassis', 2), ('Piston', 1), ('Ignition', 7);")

# ReadyDate(VarSym(part); VarSym(date)) :- DeliveryDate(VarSym(part); VarSym(date)).;
sp1 = """
    INSERT INTO ready_date(part, days)
    SELECT 
        component AS part,
        days AS days,
    FROM delivery_date;
"""
con.execute(sp1)

# ReadyDate(VarSym(part); <clo>(VarSym(componentDate), VarSym(assemblyTime))) :- PartDepends(VarSym(part), VarSym(component)), AssemblyTime(VarSym(part), VarSym(assemblyTime)), ReadyDate(VarSym(component); VarSym(componentDate)).;
sp2 = """
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
con.execute(sp2)

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
    sp3 = """
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
    con.execute(sp3)

    sp4 = """
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
    con.execute(sp4)
    print("new_ready_date")
    con.table("new_ready_date").show()

    con.execute("INSERT INTO ready_date (part, days) SELECT part, days FROM new_ready_date ON CONFLICT DO UPDATE SET days = EXCLUDED.days;")
    con.execute(f"ALTER TABLE new_ready_date RENAME TO new_ready_date_swap;")
    con.execute(f"ALTER TABLE delta_ready_date RENAME TO new_ready_date;")
    con.execute(f"ALTER TABLE new_ready_date_swap RENAME TO delta_ready_date;")

    count = 0
    ans1 = con.execute("SELECT COUNT(*) FROM delta_ready_date;").fetchone()
    if ans1 is not None:
        count = ans1[0]


    print(f"loop - count: {count}")
    
    if count <= 0:
        break



# TODO - calc result...



con.close()
