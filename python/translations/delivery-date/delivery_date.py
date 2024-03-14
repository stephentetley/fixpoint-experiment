
import duckdb





duckdb_path = 'e:/coding/python/fixpoint-experiment/python/translations/delivery-date/delivery-date.duckdb'


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
    


con = duckdb.connect(database=duckdb_path, read_only=False)

# In RAM program:
# ?ReadyDate  => delta_ready_date
# ?ReadyDate' => new_ready_date

con.execute("CREATE OR REPLACE TABLE part_depends (part VARCHAR, component VARCHAR);")
con.execute("CREATE OR REPLACE TABLE assembly_time (part VARCHAR, days INTEGER);")
con.execute("CREATE OR REPLACE TABLE delivery_date (component VARCHAR, days INTEGER);")
con.execute("CREATE OR REPLACE TABLE ready_date (part VARCHAR, days INTEGER);")
con.execute("CREATE OR REPLACE TABLE delta_ready_date (part VARCHAR, days INTEGER);")
con.execute("CREATE OR REPLACE TABLE new_ready_date (part VARCHAR, days INTEGER);")

# path1_sql = """
#     INSERT INTO new_path (path_from, path_to) 
#     WITH cte1 AS (SELECT 
#             t1.edge_from AS path_from,
#             t2.path_to AS path_to,
#         FROM edge AS t1
#         JOIN delta_path t2 ON t1.edge_to = t2.path_from)
#     SELECT path_from, path_to FROM cte1
#     WHERE NOT EXISTS (
#         SELECT path_from, path_to
#         FROM path AS t2
#         WHERE t2.path_from = cte1.path_from AND t2.path_to = cte1.path_to
#     )
# """

con.execute("INSERT INTO part_depends (part, component) VALUES ('Car', 'Chassis'), ('Car', 'Engine'), ('Engine', 'Piston'), ('Engine', 'Ignition');")
print("part_depends")
con.table("part_depends").show()

con.execute("INSERT INTO assembly_time (part, DAYS) VALUES ('Car', 7), ('Engine', 2);")
print("assembly_time")
con.table("assembly_time").show()

con.execute("INSERT INTO delivery_date (component, DAYS) VALUES ('Chassis', 2), ('Piston', 1), ('Ignition', 7);")
print("delivery_date")
con.table("delivery_date").show()

# ReadyDate(VarSym(part); VarSym(date)) :- DeliveryDate(VarSym(part); VarSym(date)).;
sp1 = """
    INSERT INTO ready_date(part, days)
    SELECT 
        component AS part,
        days AS days,
    FROM delivery_date;
"""
con.execute(sp1)
print("ready_date")
con.table("ready_date").show()

# ReadyDate(VarSym(part); <clo>(VarSym(componentDate), VarSym(assemblyTime))) :- PartDepends(VarSym(part), VarSym(component)), AssemblyTime(VarSym(part), VarSym(assemblyTime)), ReadyDate(VarSym(component); VarSym(componentDate)).;
sp2 = """
    INSERT INTO ready_date(part, days)
    SELECT 
        t0.part AS part,
        greatest(t2.days, t1.days) AS days,
    FROM part_depends t0
    JOIN assembly_time t1 ON t1.part = t0.part
    JOIN ready_date t2 ON t2.part = t0.component
"""
con.execute(sp2)
print("ready_date")
con.table("ready_date").show()

con.execute(f"INSERT INTO delta_ready_date (part, days) SELECT part, days FROM ready_date;")
print("delta_ready_date")
con.table("delta_ready_date").show()

# loop - use a vacuous condition, actual condition tested for before the `break` statement
while True:

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
    print("new_ready_date")
    con.table("new_ready_date").show()

    sp4 = """
        INSERT INTO new_ready_date(part, days)
        SELECT 
            t0.part AS part,
            greatest(t2.days, t1.days) AS days,
        FROM part_depends t0
        JOIN assembly_time t1 ON t1.part = t0.part
        JOIN ready_date t2 ON t2.part = t0.component
        EXCEPT
        SELECT 
            t3.part AS part,
            t3.days AS days,
        FROM ready_date t3
    """
    con.execute(sp4)
    print("new_ready_date")
    con.table("new_ready_date").show()



    print("loop")
    break



# TODO



con.close()
