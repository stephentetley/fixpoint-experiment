# Figure 5 from On Fast Large-Scale Program Analysis in Datalog
# use conda::datafusion-env [Ctrl+Shift+P then `Python: Select Interpreter`]

import polars as pl


edge = pl.DataFrame(data=None, schema={"edge_from": pl.Int64, "edge_to": pl.Int64})
path = pl.DataFrame(data=None, schema={"path_from": pl.Int64, "path_to": pl.Int64})
delta_path = pl.DataFrame(data=None, schema=path.schema)
new_path = pl.DataFrame(data=None, schema=path.schema)

print(edge)

def project_into(xs, df: pl.DataFrame) -> pl.DataFrame:
    scm = df.schema
    tail = pl.DataFrame(data=xs, schema=scm)
    return df.vstack(tail)

# merge_into merges tables with same schema
def merge_into(df: pl.DataFrame, dest: pl.DataFrame) -> pl.DataFrame:
    c1 = df.schema.keys()
    df1 = df.select(c1)
    return dest.vstack(df1)

def swap(df1: pl.DataFrame, df2: pl.DataFrame) -> tuple[pl.DataFrame, pl.DataFrame]:
    return (df2, df1)


def purge(df: pl.DataFrame) -> pl.DataFrame:
    scm = df.schema
    return pl.DataFrame(data=None, schema=scm)

edge = project_into([[1], [2]], edge)
edge = project_into([[2], [3]], edge)

print(edge)


for row in edge.iter_rows(named=False): 
    path = project_into([[row[0], row[1]]], path)
                        

print("path:")
print(path)

# delta_path.vstack(path.select(["path_from", "path_to"]))
delta_path = merge_into(path, delta_path) 

print("delta_path:")
print(delta_path)


# loop - use a vacuous condition, actual condition tested for `break`
one_hundred = 100
while one_hundred > 0:
    new_path = purge(new_path)

    
    path_join = edge.join(delta_path, left_on="edge_to", right_on="path_from", how="inner").rename({"edge_from": "path_from"}).drop(["edge_to"])
    path_join = path_join.join(path, on=["path_from", "path_to"], how="anti")
    print("path_join:")
    print(path_join)

    for row in path_join.iter_rows(named=False): 
        path = project_into([[row[0], row[1]]], path)


    count_tuples = new_path.select(pl.len()).item()
    print("count_tuples: {}".format(count_tuples))

    if count_tuples == 0: 
        break
    
    path = merge_into(new_path, path)
    new_path, delta_path = swap(new_path, delta_path)

print("Done - path:")
print(path)

