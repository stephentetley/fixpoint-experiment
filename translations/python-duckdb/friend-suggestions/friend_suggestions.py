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

def swap(con: duckdb.DuckDBPyConnection, table1: str, table2: str) -> None:
    table_swap = f"{table1}_swap"
    con.execute(f"ALTER TABLE {table1} RENAME TO {table_swap};")
    con.execute(f"ALTER TABLE {table2} RENAME TO {table1};")
    con.execute(f"ALTER TABLE {table_swap} RENAME TO {table2};")

def table_is_empty(con: duckdb.DuckDBPyConnection, table: str) -> bool:
    query = f"SELECT count(1) WHERE EXISTS (SELECT * FROM {table});"
    ans1 = con.execute(query).fetchone()
    return (ans1[0] == 0)


con = duckdb.connect()

table_ddl = """
    CREATE OR REPLACE TABLE friend (me VARCHAR, friend VARCHAR, PRIMARY KEY(me, friend));
    CREATE OR REPLACE TABLE suggestion (friend VARCHAR, newfriend VARCHAR, PRIMARY KEY(friend, newfriend));
    CREATE OR REPLACE TABLE delta_suggestion (friend VARCHAR, newfriend VARCHAR, PRIMARY KEY(friend, newfriend));
    CREATE OR REPLACE TABLE new_suggestion (friend VARCHAR, newfriend VARCHAR, PRIMARY KEY(friend, newfriend));
    CREATE OR REPLACE TABLE zresult (friend VARCHAR, newfriend VARCHAR, PRIMARY KEY(friend, newfriend));
    CREATE OR REPLACE TABLE delta_zresult (friend VARCHAR, newfriend VARCHAR, PRIMARY KEY(friend, newfriend));
    CREATE OR REPLACE TABLE new_zresult (friend VARCHAR, newfriend VARCHAR, PRIMARY KEY(friend, newfriend));
    DROP MACRO IF EXISTS pred1;
    CREATE MACRO pred1(f1, f2, f3) AS f1 != f2 AND f2 != f3 AND f1 != f3;
"""
con.execute(table_ddl)

data_load = """
    INSERT INTO friend (me, friend) VALUES 
            ('George', 'Antonio'), ('George', 'Sarah'), ('George', 'Roberto'), 
            ('Sarah', 'Hisham'), ('Antonio', 'Hisham'), ('Roberto', 'Hisham');
"""
con.execute(data_load)


# Suggestion(VarSym(me), VarSym(nf)) :- Friend(VarSym(me), VarSym(f1)), Friend(VarSym(me), VarSym(f2)), Friend(VarSym(me), VarSym(f3)), Friend(VarSym(f1), VarSym(nf)), Friend(VarSym(f2), VarSym(nf)), Friend(VarSym(f3), VarSym(nf)), <clo>(VarSym(f2), VarSym(f1), VarSym(f3)), not Friend(VarSym(me), VarSym(nf)).;
# Ideally would not need the DISTINCT ...
query = """
    INSERT INTO suggestion(friend, newfriend)
    SELECT DISTINCT
        t0.me AS friend,
        t3.friend AS newfriend,
    FROM friend t0
    JOIN friend t1 ON t1.me = t0.me
    JOIN friend t2 ON t2.me = t0.me AND pred1(t1.friend, t0.friend, t2.friend)
    JOIN friend t3 ON t3.me = t0.friend AND NOT EXISTS (SELECT * FROM friend s4 WHERE s4.me = t0.me AND s4.friend = t3.friend)
    JOIN friend t4 ON t4.me = t1.friend AND t4.friend = t3.friend
    JOIN friend t5 ON t5.me = t2.friend AND t5.friend = t3.friend;
    """
con.execute(query)

# $Result(VarSym(x), VarSym(y)) :- Suggestion(VarSym(x), VarSym(y)).;
query = """
    INSERT INTO zresult(friend, newfriend)
    SELECT 
        t0.friend AS friend,
        t0.newfriend AS newfriend,
    FROM suggestion t0
    """
con.execute(query)

# merge $Result into delta_$Result;
merge_into(con, src='zresult', dest='delta_zresult', cols=['friend', 'newfriend'])

# merge Suggestion into delta_Suggestion;
merge_into(con, src='suggestion', dest='delta_suggestion', cols=['friend', 'newfriend'])

delta_zresult_empty, delta_suggestion_empty = False, False
while not (delta_zresult_empty and delta_suggestion_empty):
    # purge new_$Result;
    purge_table(con, "new_zresult")
    # purge new_Suggestion;
    purge_table(con, "new_suggestion")

    # Suggestion(VarSym(me), VarSym(nf)) :- Friend(VarSym(me), VarSym(f1)), Friend(VarSym(me), VarSym(f2)), Friend(VarSym(me), VarSym(f3)), Friend(VarSym(f1), VarSym(nf)), Friend(VarSym(f2), VarSym(nf)), Friend(VarSym(f3), VarSym(nf)), <clo>(VarSym(f2), VarSym(f1), VarSym(f3)), not Friend(VarSym(me), VarSym(nf)).;
    # $Result(VarSym(x), VarSym(y)) :- Suggestion(VarSym(x), VarSym(y)).;
    
    query = """
        INSERT INTO new_zresult(friend, newfriend)
        SELECT 
            t0.friend AS friend,
            t0.newfriend AS newfriend,
        FROM delta_suggestion t0
        WHERE NOT EXISTS (SELECT * FROM zresult s0 WHERE s0.friend = t0.friend AND s0.newfriend = t0.newfriend)
        """
    con.execute(query)


    # merge new_$Result into $Result;
    merge_into(con, src='new_zresult', dest='zresult', cols=['friend', 'newfriend'])

    # merge new_Suggestion into Suggestion;
    merge_into(con, src='new_suggestion', dest='suggestion', cols=['friend', 'newfriend'])
    
    swap(con, "new_zresult", "delta_zresult")
    swap(con, "new_suggestion", "delta_suggestion")

    delta_zresult_empty = table_is_empty(con, "delta_zresult")
    delta_suggestion_empty = table_is_empty(con, "delta_suggestion")
    print(f"empty_deltas: {delta_zresult_empty}, {delta_suggestion_empty}")



print("zresult")
con.table("zresult").show()


con.close()
