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





part_depends = pl.DataFrame(data=None, schema={'part': pl.String, 'component': pl.String})
assembly_time = pl.DataFrame(data=None, schema={'part': pl.String, 'days': pl.Int64})
delivery_date = pl.DataFrame(data=None, schema={'component': pl.String, 'days': pl.Int64})
ready_date = pl.DataFrame(data=None, schema={'part': pl.String, 'days': pl.Int64})
delta_ready_date = pl.DataFrame(data=None, schema={'part': pl.String, 'days': pl.Int64});
new_ready_date = pl.DataFrame(data=None, schema={'part': pl.String, 'days': pl.Int64});
zresult = pl.DataFrame(data=None, schema={'part': pl.String, 'days': pl.Int64});
delta_zresult = pl.DataFrame(data=None, schema={'part': pl.String, 'days': pl.Int64});
new_zresult = pl.DataFrame(data=None, schema={'part': pl.String, 'days': pl.Int64});

# [5,7,9,11]
part_depends = project_into([{'part': 'Car', 'component': 'Chassis'}, 
                             {'part': 'Car', 'component': 'Engine'}, 
                             {'part': 'Engine', 'component': 'Piston'},
                             {'part': 'Engine', 'component': 'Ignition'}], 
                             part_depends)

# [13,15]
assembly_time = project_into([{'part': 'Car', 'days': 7}, 
                              {'part': 'Engine', 'days': 2}], 
                              assembly_time)


# [13,15]
delivery_date = project_into([{'component': 'Chassis', 'days': 2}, 
                              {'component': 'Piston', 'days': 1}, 
                              {'component': 'Ignition', 'days': 7}], 
                              delivery_date)


# [23] ReadyDate(VarSym(part); VarSym(date)) :- DeliveryDate(VarSym(part); VarSym(date)).;
ready_date = delivery_date.select(pl.col('component').alias('part'), pl.col('days'))


# [27] ReadyDate(VarSym(part); <clo>(VarSym(componentDate), VarSym(assemblyTime))) :- PartDepends(VarSym(part), VarSym(component)), AssemblyTime(VarSym(part), VarSym(assemblyTime)), ReadyDate(VarSym(component); VarSym(componentDate)).;
temp = part_depends.join(assembly_time, on='part', how='inner')
temp = temp.join(ready_date, left_on='component', right_on='part', how='inner').select(pl.col('part'), pl.sum_horizontal('days', 'days_right').alias('days'))
temp = temp.group_by('part').agg(pl.col('days').alias('days').max())
ready_date = ready_date.vstack(temp)


# [35] merge ReadyDate into delta_ReadyDate;
delta_ready_date = merge_into(ready_date, delta_ready_date)

delta_ready_date_empty = False
while not (delta_ready_date_empty):

    # [37] purge new_ReadyDate;
    new_ready_date = purge(new_ready_date)

    # [38] ReadyDate(VarSym(part); VarSym(date)) :- DeliveryDate(VarSym(part); VarSym(date)).;
    temp = delivery_date.join(ready_date, left_on=['component', 'days'], right_on=['part', 'days'], how='anti')
    new_ready_date = new_ready_date.vstack(temp.select(pl.col('component').alias('part'), pl.col('days')))

    # [44] ReadyDate(VarSym(part); <clo>(VarSym(componentDate), VarSym(assemblyTime))) :- PartDepends(VarSym(part), VarSym(component)), AssemblyTime(VarSym(part), VarSym(assemblyTime)), ReadyDate(VarSym(component); VarSym(componentDate)).;
    temp = part_depends.join(assembly_time, on='part', how='inner')
    temp = temp.join(ready_date, left_on='component', right_on='part', how='inner').select(pl.col('part'), pl.sum_horizontal('days', 'days_right').alias('days'))
    temp = temp.group_by('part').agg(pl.col('days').max())
    temp = temp.join(ready_date, on=['part', 'days'], how='anti')
    new_ready_date = new_ready_date.vstack(temp)

    # [54] merge new_ReadyDate into ReadyDate;
    ready_date = merge_into(new_ready_date, ready_date)
    ready_date = ready_date.group_by('part').agg(pl.col('days').max())

    # [55] delta_ReadyDate := new_ReadyDate
    delta_ready_date = new_ready_date

    delta_ready_date_empty = delta_ready_date.is_empty()
    print(f"empty_deltas: {delta_ready_date_empty}")


# [57] $Result(VarSym(c), VarSym(d)) :- fix ReadyDate(VarSym(c); VarSym(d)).;
zresult = zresult.vstack(ready_date)


# [61] merge $Result into delta_$Result;
delta_zresult = merge_into(zresult, delta_zresult)

delta_zresult_empty = False
while not (delta_zresult_empty):
    new_zresult = purge(new_zresult)
    # [64] $Result(VarSym(c), VarSym(d)) :- fix ReadyDate(VarSym(c); VarSym(d)).;
    temp = ready_date.join(zresult, on=['part', 'days'], how='anti')
    new_zresult = new_zresult.vstack(temp)
    
    # [70] merge new_$Result into $Result;
    zresult = merge_into(new_zresult, zresult)
    
    # [71] delta_$Result := new_$Result
    delta_zresult = new_zresult

    delta_zresult_empty = delta_zresult.is_empty()
    print(f"empty_deltas: {delta_zresult_empty}")

print("zresult...")
print(zresult)

