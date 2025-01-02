import duckdb

# merge - src and dest columns must have the same names
# Maybe we need lattice merge_into that updates lattice columns...
def merge_into(con, *, src: str, dest:str, cols: list[str]) -> None:
    columns = ", ".join(cols)
    query = f"""
        INSERT INTO {dest}({columns})
        SELECT {columns} 
        FROM {src}
        ANTI JOIN {dest} USING({columns})
    """
    con.execute(query)

def lattice_merge_into(con, *, src: str, dest:str, cols: list[str], lattice_col: str) -> None:
    all_columns = ", ".join(cols + [lattice_col])
    query = f"""
        INSERT INTO {dest}({all_columns})
        SELECT {all_columns} 
        FROM {src}
        ON CONFLICT DO UPDATE SET {lattice_col} = EXCLUDED.{lattice_col};
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