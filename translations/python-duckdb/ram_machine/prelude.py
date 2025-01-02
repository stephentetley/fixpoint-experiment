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