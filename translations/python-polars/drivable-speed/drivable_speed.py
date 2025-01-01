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



# param... 45 is drivable, 55 isn't
drivable_speed = 45



print(f"Is Rome to Florence drivable at {drivable_speed}? ... check result table has an entry:")

minimum_speed = drivable_speed
unit_type = pl.Enum(['Unit'])

road = pl.DataFrame(data=None, schema={'source': pl.String, 'max_speed': pl.Int64 ,'destination': pl.String})
path = pl.DataFrame(data=None, schema={'source': pl.String,'destination': pl.String})
delta_path = pl.DataFrame(data=None, schema={'source': pl.String,'destination': pl.String})
new_path = pl.DataFrame(data=None, schema={'source': pl.String,'destination': pl.String})
zresult = pl.DataFrame(data=None, schema={'drivable': unit_type})
delta_zresult = pl.DataFrame(data=None, schema={'drivable': unit_type})
new_zresult = pl.DataFrame(data=None, schema={'drivable': unit_type})


# [5,7,9]
road = project_into([{'source': 'Rome', 'max_speed': 80, 'destination': 'Turin'}, 
                     {'source': 'Turin', 'max_speed': 70, 'destination': 'Naples'},
                     {'source': 'Naples', 'max_speed': 50, 'destination': 'Florence'}], 
                    road)

# [11] $Result(BoxedObject(((), Obj -> Obj))) :- Path(BoxedObject((Rome, Obj -> Obj)), BoxedObject((Florence, Obj -> Obj))).;
temp = road.filter((pl.col('source') == 'Rome') & (pl.col('destination') == 'Florence')).with_columns(drivable=pl.lit('Unit').cast(unit_type))
zresult = zresult.vstack(temp.select(pl.col('drivable')))

# [15] Path(VarSym(x), VarSym(y)) :- Road(VarSym(x), VarSym(maximumSpeed), VarSym(y)), <clo>(VarSym(maximumSpeed)).;
temp = road.filter((pl.col('max_speed') >= minimum_speed)).select(['source', 'destination'])
path = path.vstack(temp)

# [21] Path(VarSym(x), VarSym(z)) :- Path(VarSym(x), VarSym(y)), Road(VarSym(y), VarSym(maximumSpeed), VarSym(z)), <clo>(VarSym(maximumSpeed)).;
temp = road.filter((pl.col('max_speed') >= minimum_speed))
temp = path.join(temp, left_on='destination', right_on='source', how='inner').select(pl.col('source'), pl.col('destination_right').alias('destination'))
path = path.vstack(temp)

# [29] merge $Result into delta_$Result;
delta_zresult = merge_into(zresult, delta_zresult)
# [30] merge Path into delta_Path;
delta_path = merge_into(path, delta_path)


delta_zresult_empty, delta_path_empty = False, False
while not (delta_zresult_empty and delta_path_empty):
    # [32] purge new_$Result;
    new_zresult = purge(new_zresult)

    # [33] purge new_Path;
    new_path = purge(new_path)

    # [34] $Result(BoxedObject(((), Obj -> Obj))) :- Path(BoxedObject((Rome, Obj -> Obj)), BoxedObject((Florence, Obj -> Obj))).;
    if zresult.is_empty():
        temp = path.filter((pl.col('source') == 'Rome') & (pl.col('destination') == 'Florence')).with_columns(drivable=pl.lit('Unit').cast(unit_type))
        new_zresult = new_zresult.vstack(temp.select(pl.col('drivable')))


    # [40] Path(VarSym(x), VarSym(y)) :- Road(VarSym(x), VarSym(maximumSpeed), VarSym(y)), <clo>(VarSym(maximumSpeed)).;
    # [41] Path(VarSym(x), VarSym(z)) :- Path(VarSym(x), VarSym(y)), Road(VarSym(y), VarSym(maximumSpeed), VarSym(z)), <clo>(VarSym(maximumSpeed)).;
    temp = delta_path.join(road, left_on='destination', right_on='source', how='inner')
    temp = temp.filter((pl.col('max_speed') >= minimum_speed)).join(road, left_on=['source', 'destination_right'], right_on=['source', 'destination'], how='anti')
    new_path = new_path.vstack(temp.select(pl.col('source'), pl.col('destination_right').alias('destination')))


    # [40] merge new_$Result into $Result;
    zresult = merge_into(new_zresult, zresult)
    
    # [50] merge new_Path into Path;
    path = merge_into(new_path, path)
    
    # [51] delta_$Result := new_$Result;
    delta_zresult = new_zresult

    # [52] delta_Path := new_Path
    delta_path = new_path

    delta_zresult_empty = delta_zresult.is_empty()
    delta_path_empty = delta_path.is_empty()

    print(f"empty_deltas: {delta_zresult_empty} {delta_path_empty}")

    

print("zresult...")
print(zresult)