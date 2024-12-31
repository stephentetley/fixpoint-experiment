# Figure 5 from On Fast Large-Scale Program Analysis in Datalog
# Note for me - use conda::datafusion-env [Ctrl+Shift+P then `Python: Select Interpreter`]
# VS-code [Ctrl+Shift+P then `Python: Select Interpreter`]


import polars as pl

def project_into(xs, df: pl.DataFrame) -> pl.DataFrame:
    scm = df.schema
    tail = pl.DataFrame(data=xs, schema=scm)
    return df.vstack(tail)

# merge_into merges tables with same schema
def merge_into(df: pl.DataFrame, dest: pl.DataFrame) -> pl.DataFrame:
    c1 = df.schema.keys()
    df1 = df.select(c1)
    df1 = dest.vstack(df1)
    return df1.unique()

def swap(df1: pl.DataFrame, df2: pl.DataFrame) -> tuple[pl.DataFrame, pl.DataFrame]:
    return (df2, df1)


def purge(df: pl.DataFrame) -> pl.DataFrame:
    scm = df.schema
    return pl.DataFrame(data=None, schema=scm)


edge = pl.DataFrame(data=None, schema={"edge_from": pl.Int64, "edge_to": pl.Int64})
path = pl.DataFrame(data=None, schema={"path_from": pl.Int64, "path_to": pl.Int64})
delta_path = pl.DataFrame(data=None, schema=path.schema)
new_path = pl.DataFrame(data=None, schema=path.schema)
zresult = pl.DataFrame(data=None, schema=path.schema)
delta_zresult = pl.DataFrame(data=None, schema=path.schema)
new_zresult = pl.DataFrame(data=None, schema=path.schema)


# [5,7,9]
edge = project_into([{'edge_from': 1, 'edge_to': 2}, 
                     {'edge_from': 2, 'edge_to': 3}, 
                     {'edge_from': 3, 'edge_to': 4}], 
                    edge)

# [11] $Result(VarSym(x1), VarSym(x2)) :- Path(VarSym(x1), VarSym(x2)).;
zresult = path.select(pl.col('path_from'), pl.col('path_to'))

# [15] Path(VarSym(x), VarSym(y)) :- Edge(VarSym(x), VarSym(y)).;
path = path.vstack(edge.select(pl.col('edge_from').alias('path_from'), pl.col('edge_to').alias('path_to')))
                        


# [19] Path(VarSym(x), VarSym(z)) :- Path(VarSym(x), VarSym(y)), Edge(VarSym(y), VarSym(z)).;
temp = path.join(edge, left_on='path_to', right_on='edge_from', how='inner')
path = path.vstack(temp.select(pl.col('path_to').alias('path_from'), pl.col('edge_to').alias('path_to'))) 

# [25] merge $Result into delta_$Result;
delta_zresult = merge_into(zresult, delta_zresult)

# [26] merge Path into delta_Path;
delta_path = merge_into(path, delta_path)

# loop - use a vacuous condition, actual condition tested for `break`
while True:

    new_path = purge(new_path)

    # [28] purge new_$Result;
    new_zresult = purge(new_zresult)

    # [29] purge new_Path;
    new_path = purge(new_path)
    
    # [30] $Result(VarSym(x1), VarSym(x2)) :- Path(VarSym(x1), VarSym(x2)).;
    temp = delta_path.join(zresult, on=['path_from', 'path_to'], how='anti')
    new_zresult = new_zresult.vstack(temp)

    # [36] Path(VarSym(x), VarSym(y)) :- Edge(VarSym(x), VarSym(y)).;
    # [37] Path(VarSym(x), VarSym(z)) :- Path(VarSym(x), VarSym(y)), Edge(VarSym(y), VarSym(z)).;
    
    temp = delta_path.join(edge, left_on='path_to', right_on='edge_from', how='inner').select(pl.col('path_from'), pl.col('edge_to').alias('path_to'))
    temp = temp.join(path, on=['path_from', 'path_to'], how='anti')
    new_path = new_path.vstack(temp)
    
    # [45] merge new_$Result into $Result;
    zresult = merge_into(new_zresult, zresult)

    # [46] merge new_Path into Path;
    path = merge_into(new_path, path)

    # [47] delta_$Result := new_$Result;
    delta_zresult = new_zresult

    # [48] delta_Path := new_Path
    delta_path = new_path

    delta_zresult_count = delta_zresult.select(pl.len()).item()
    delta_path_count = delta_path.select(pl.len()).item()

    print("count_tuples: {}".format(delta_zresult_count + delta_path_count))

    if delta_zresult_count == 0 and delta_path_count == 0: 
        break
    

print("Done - path:")
print(path)

