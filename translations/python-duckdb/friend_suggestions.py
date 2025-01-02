import duckdb
import ram_machine.prelude as ram



con = duckdb.connect()

table_ddl = """
    CREATE OR REPLACE TABLE friend (me VARCHAR, friend VARCHAR);
    CREATE OR REPLACE TABLE suggestion (friend VARCHAR, newfriend VARCHAR);
    CREATE OR REPLACE TABLE delta_suggestion (friend VARCHAR, newfriend VARCHAR);
    CREATE OR REPLACE TABLE new_suggestion (friend VARCHAR, newfriend VARCHAR);
    CREATE OR REPLACE TABLE zresult (friend VARCHAR, newfriend VARCHAR);
    CREATE OR REPLACE TABLE delta_zresult (friend VARCHAR, newfriend VARCHAR);
    CREATE OR REPLACE TABLE new_zresult (friend VARCHAR, newfriend VARCHAR);
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


# [5] Suggestion(VarSym(me), VarSym(nf)) :- Friend(VarSym(me), VarSym(f1)), Friend(VarSym(me), VarSym(f2)), Friend(VarSym(me), VarSym(f3)), Friend(VarSym(f1), VarSym(nf)), Friend(VarSym(f2), VarSym(nf)), Friend(VarSym(f3), VarSym(nf)), <clo>(VarSym(f2), VarSym(f1), VarSym(f3)), not Friend(VarSym(me), VarSym(nf)).;
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

# [23] $Result(VarSym(x), VarSym(y)) :- Suggestion(VarSym(x), VarSym(y)).;
query = """
    INSERT INTO zresult(friend, newfriend)
    SELECT 
        t0.friend AS friend,
        t0.newfriend AS newfriend,
    FROM suggestion t0
    """
con.execute(query)

# [27] merge $Result into delta_$Result;
ram.merge_into(con, src='zresult', dest='delta_zresult', cols=['friend', 'newfriend'])

# [28] merge Suggestion into delta_Suggestion;
ram.merge_into(con, src='suggestion', dest='delta_suggestion', cols=['friend', 'newfriend'])

delta_zresult_empty, delta_suggestion_empty = False, False
while not (delta_zresult_empty and delta_suggestion_empty):
    # [30] purge new_$Result;
    ram.purge_table(con, "new_zresult")
    # [31] purge new_Suggestion;
    ram.purge_table(con, "new_suggestion")

    # [32] Suggestion(VarSym(me), VarSym(nf)) :- Friend(VarSym(me), VarSym(f1)), Friend(VarSym(me), VarSym(f2)), Friend(VarSym(me), VarSym(f3)), Friend(VarSym(f1), VarSym(nf)), Friend(VarSym(f2), VarSym(nf)), Friend(VarSym(f3), VarSym(nf)), <clo>(VarSym(f2), VarSym(f1), VarSym(f3)), not Friend(VarSym(me), VarSym(nf)).;
    # [33] $Result(VarSym(x), VarSym(y)) :- Suggestion(VarSym(x), VarSym(y)).;
    query = """
        INSERT INTO new_zresult(friend, newfriend)
        SELECT 
            t0.friend AS friend,
            t0.newfriend AS newfriend,
        FROM delta_suggestion t0
        ANTI JOIN zresult USING (friend, newfriend);
        """
    con.execute(query)


    # [39] merge new_$Result into $Result;
    ram.merge_into(con, src='new_zresult', dest='zresult', cols=['friend', 'newfriend'])

    # [40] merge new_Suggestion into Suggestion;
    ram.merge_into(con, src='new_suggestion', dest='suggestion', cols=['friend', 'newfriend'])
    
    # [41] delta_$Result := new_$Result;
    ram.bind_table(con, left_table="delta_zresult", right_table="new_zresult", cols=['friend', 'newfriend'])
    
    # [42] delta_Suggestion := new_Suggestion
    ram.bind_table(con, left_table="delta_suggestion", right_table="new_suggestion", cols=['friend', 'newfriend'])

    delta_zresult_empty = ram.table_is_empty(con, "delta_zresult")
    delta_suggestion_empty = ram.table_is_empty(con, "delta_suggestion")
    print(f"empty_deltas: {delta_zresult_empty}, {delta_suggestion_empty}")


print("zresult")
con.table("zresult").show()

con.close()
