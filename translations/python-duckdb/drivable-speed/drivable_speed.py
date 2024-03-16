
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
    

duckdb_path = 'e:/coding/python/fixpoint-experiment/translations/python-duckdb/drivable-speed/drivable-speed.duckdb'


con = duckdb.connect(database=duckdb_path, read_only=False)


con.execute("CREATE OR REPLACE TABLE road (source VARCHAR, max_speed INTEGER, destination VARCHAR);")
con.execute("CREATE OR REPLACE TABLE path (source VARCHAR, destination VARCHAR);")
con.execute("CREATE OR REPLACE TABLE delta_path (source VARCHAR, destination VARCHAR);")
con.execute("CREATE OR REPLACE TABLE new_path (source VARCHAR, destination VARCHAR);")
con.execute("CREATE OR REPLACE TABLE result (source VARCHAR, destination VARCHAR);")
con.execute("CREATE OR REPLACE TABLE delta_result (source VARCHAR, destination VARCHAR);")
con.execute("CREATE OR REPLACE TABLE new_result (source VARCHAR, destination VARCHAR);")

con.execute("INSERT INTO road (source, max_speed, destination) VALUES ('Rome', 80, 'Turin'), ('Turin', 70, 'Naples'), ('Naples', 50, 'Florence');")

# TODO


con.close()
