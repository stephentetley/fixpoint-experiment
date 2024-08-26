/*
 * Copyright 2024 Stephen Tetley
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

mod SqlSyntax.Pretty {

    use Either.{Left, Right}
    use Text.PrettyPrint.Doc
    use Text.PrettyPrint.{<<>>, <^>, empty, text, parens, brackets, sep, intersperse, equals, comma, space, tupled, optionalTupled, 
        dot, char, int32, squotes, conditionalParens}


    use SqlSyntax.Datatypes.{ShowParens, ShowAsKeyword, ShowColumnKeyword, ShowDatabaseKeyword,
        Expression, Literal, BinOp, Collation, 
        CaseStmt,
        Statement, 
        AlterTableStmt, AlterTableClause, AlterColumnOptions, SetData,
        AlterViewStmt, AlterViewClause,
        AttachStmt, AttachOption, AttachType,
        CreateSchemaStmt, 
        CreateTableStmt, OrReplace, Temporary, IfNotExists, IfExists, ColumnSpec, GeneratedColumn, GeneratedAlways, VirtualOrStored, ColumnConstraint, TableConstraint, 
        DeleteStmt,
        DetachStmt, DetachQualifier,
        DropStmt, 
        QualifiedName, DropQualifier, CascadeOrRestrict, 
        InsertStmt, InsertQualifier, InsertColumnQualifier, InsertValuesClause, OnConflictClause, DoUpdateOrDoNothing, 
        ReturningClause, ReturnExpression,
        SelectStmt, SelectClause, DistinctClause, SelectExpr, StarOptions, LambdaFilter, ColumnExpression,
        ValuesClause, FromClause, FromExpressionOrJoin, FromExpression, JoinClause, 
        TableOrSubquery, JoinDesignation, JoinCondition, NaturalOrAsOf, JoinType, OuterJoin, 
        TableAlias, TableSample, 
        PercentOrRow, SamplingMethod, 
        WhereClause, 
        GroupByClause, Grouping, CubeOrRollup, 
        HavingClause, 
        WindowClause, WindowDefinition, PartitionByClause, FrameSpec, RangeOrRows, FrameBody, FrameStart, FrameStop, WindowExclude,
        QualifyClause, 
        OrderByClause, OrderDesignation, SortOrder, NullsPosition,
        LimitClause, 
        UseStmt,
        DataType, IntegerType, FloatingPointType, TimestampPrecision
        }


    // TEMP - add to flix-pretty?
    // def optionalIntersperse(sepd: Doc, docs: List[Doc]): Doc = match Text.PrettyPrint.removeEmpties(docs) {
    //     case Nil => empty()
    //     case xs  => intersperse(sepd, xs)
    // }

    /// TEMP - add to flix-pretty - need to check the operator intersperse uses instead of manually adding a space (", ")
    def commaSpaceSep(docs: List[Doc]): Doc = intersperse(text(", "), docs)

    def someOrEmpty(x: Option[a], f: a -> Doc): Doc = match x { 
        case Some(a) => f(a)
        case None    => empty()
    }

    def showParens(x: ShowParens, d: Doc): Doc = match x {
        case ShowParens.ShowParens  => parens(d)
        case ShowParens.NoParens    => d
    }

    def ppShowAsKeyword(x: ShowAsKeyword): Doc = match x {
        case ShowAsKeyword.ShowAs  => text("AS")
        case ShowAsKeyword.NoAs    => empty()
    }

    def ppShowColumnKeyword(x: ShowColumnKeyword): Doc = match x {
        case ShowColumnKeyword.ShowColumn  => text("COLUMN")
        case ShowColumnKeyword.NoColumn    => empty() 
    }


    def ppShowDatabaseKeyword(x: ShowDatabaseKeyword): Doc = match x {
        case ShowDatabaseKeyword.ShowDatabase   => text("DATABASE")
        case ShowDatabaseKeyword.NoDatabase     => empty() 
    }

    pub def ppQualifiedName(x: QualifiedName): Doc = match x {
        case QualifiedName.QualifiedName(scm, name) => text(scm ++ "." ++ name)
        case QualifiedName.UnqualifiedName(name)    => text(name)
    }

    // TODO Binary will need precedence handling / parens...
    pub def ppExpression(x: Expression): Doc = match x {
        case Expression.Identifier(s)           => text(s)
        case Expression.Literal(lit)            => ppLiteral(lit)
        case Expression.Binary(e1, op, e2)      => ppExpression(e1) <<>> ppBinOp(op) <<>> ppExpression(e2)
        case Expression.Case(stmt)              => ppCaseStmt(stmt)
        case Expression.Cast(expr, ty)          => text("CAST") ++ parens(ppExpression(expr) <<>> text("AS") <<>> ppDataType(ty))
        case Expression.TryCast(expr, ty)       => text("TRY_CAST") ++ parens(ppExpression(expr) <<>> text("AS") <<>> ppDataType(ty))
        case Expression.Collation(expr, coll)   => ppExpression(expr) <<>> text("COLLATE") <<>> ppCollation(coll)
    }

    pub def ppLiteral(x: Literal): Doc = match x {
        case Literal.Null       => text("NULL")
        case Literal.Integer(i) => int32(i)
        case Literal.Varchar(s) => squotes(text(s))
    }

    def ppBinOp(x: BinOp): Doc = match x {
        case BinOp.LessThan             => char('<')
        case BinOp.GreaterThan          => char('>')
        case BinOp.LessThanEquals       => text("<=")
        case BinOp.GreaterThanEquals    => text(">=")
        case BinOp.Equals               => char('=')
        case BinOp.NotEquals            => text("!=")
    }

    pub def ppCaseStmt(x: CaseStmt): Doc = match x {
        case CaseStmt.CaseStmt(optSwitch, caseList, optElse) => {
            let switchOn = someOrEmpty(optSwitch, ppExpression);
            let makeCaseStep = match (patt, ans) -> text("WHEN") <<>> ppExpression(patt) <<>> text("THEN") <<>> ppExpression(ans);
            let cases = List.map(makeCaseStep, caseList);
            let caseElse = someOrEmpty(optElse, expr -> text("ELSE") <<>> ppExpression(expr));
            text("CASE") <^> switchOn <<>> sep(cases) <^> caseElse <<>> text("END")
        }
    }

    def ppCollation(x: Collation): Doc = match x {
        case Collation.NoCase       => text("NOCASE")
        case Collation.NoAccent     => text("NOCASE")
        case Collation.NFC          => text("NOCASE")
        case Collation.Extension(s) => text(s)
    }


    pub def ppStatement(x: Statement): Doc = match x {
        case Statement.AlterTable(stmt)     => ppAlterTableStmt(stmt)
        case Statement.AlterView(stmt)      => ppAlterViewStmt(stmt)
        case Statement.Attach(stmt)         => ppAttachStmt(stmt)
        case Statement.CreateSchema(stmt)   => ppCreateSchemaStmt(stmt)
        case Statement.CreateTable(stmt)    => ppCreateTableStmt(stmt)
        case Statement.Delete(stmt)         => ppDeleteStmt(stmt)
        case Statement.Drop(stmt)           => ppDropStmt(stmt)
        case Statement.Detach(stmt)         => ppDetachStmt(stmt)
        case Statement.Insert(stmt)         => ppInsertStmt(stmt)
        case Statement.Select(stmt)         => ppSelectStmt(stmt)
        case Statement.Use(stmt)            => ppUseStmt(stmt)
    }


    pub def ppAlterTableStmt(x: AlterTableStmt): Doc= match x {
        case AlterTableStmt.AlterTableStmt(qname, alterClause) => 
            text("ALTER TABLE") <<>> ppQualifiedName(qname) <<>> ppAlterTableClause(alterClause)
    }

    def ppAlterTableClause(x: AlterTableClause): Doc = match x {
        case AlterTableClause.Add(showColumn, optIfNotExists, name, sqltype, colConstraints) => {
            let ifNotExists = someOrEmpty(optIfNotExists, ppIfNotExists);
            let columnConstraints = sep(List.map(ppColumnConstraint, colConstraints));
            text("ADD") <^> ppShowColumnKeyword(showColumn) <^> ifNotExists <^> text(name) <<>> ppDataType(sqltype) <^> columnConstraints
        }
        case AlterTableClause.Drop(showColumn, optIfExists, name)   => {
            let ifExists = someOrEmpty(optIfExists, ppIfExists);
            text("DROP") <^> ppShowColumnKeyword(showColumn) <^> ifExists <^> text(name)
        }
        case AlterTableClause.Alter(showColumn, name, alterOpts)     => 
            text("ALTER") <^> ppShowColumnKeyword(showColumn) <^> text(name) <<>> ppAlterColumnOptions(alterOpts)
        case AlterTableClause.Rename(showColumn, olds, news)        =>
            text("RENAME") <^> ppShowColumnKeyword(showColumn) <^> text(olds) <<>> text("TO") <<>> text(news)
        case AlterTableClause.RenameTo(name)                        => 
            text("RENAME TO") <<>> text(name)
    }



    def ppAlterColumnOptions(x: AlterColumnOptions): Doc = match x {
        case AlterColumnOptions.SetData(optSetData, sqltype, optCollationName, optUsing) => {
            let setData = someOrEmpty(optSetData, ppSetData);
            let collate = someOrEmpty(optCollationName, s -> text("COLLATE") <<>> text(s));
            let usingExpr = someOrEmpty(optUsing, expr -> text("USING") <<>> ppExpression(expr));
            setData <^> text("TYPE") <<>> ppDataType(sqltype) <^> collate <^> usingExpr
        }
        case AlterColumnOptions.SetDefault(expr)       => text("SET DEFAULT") <<>> ppExpression(expr)
        case AlterColumnOptions.DropDefault            => text("DROP DEFAULT")
    }

    def ppSetData(x: SetData): Doc = match x {
        case SetData.SetData    => text("SET DATA")
    }

    pub def ppAlterViewStmt(x: AlterViewStmt): Doc= match x {
        case AlterViewStmt.AlterViewStmt(qname, alterClause) => 
            text("ALTER VIEW") <<>> ppQualifiedName(qname) <<>> ppAlterViewClause(alterClause)
    }

    def ppAlterViewClause(x: AlterViewClause): Doc = match x {
        case AlterViewClause.RenameTo(name) => text("RENAME TO") <<>> text(name)
    }


    pub def ppAttachStmt(x: AttachStmt): Doc = match x {
        case AttachStmt.AttachStmt(showDatabase, optIfNotExists, path, optDbAlias, optOptions) => {
            let ifNotExists = someOrEmpty(optIfNotExists, ppIfNotExists);
            let dbAlias = someOrEmpty(optDbAlias, s -> text("ALIAS") <<>> text(s));
            let attachOptions = someOrEmpty(optOptions, opts -> optionalTupled(List.map(ppAttachOption , opts)));
            text("ATTACH") <^> ppShowDatabaseKeyword(showDatabase) <^> ifNotExists 
                <^> text(path) <^> dbAlias <^> attachOptions
        }
    }

    def ppAttachOption(x: AttachOption): Doc = match x {
        case AttachOption.ReadOnly(None)            => text("READ_ONLY")
        case AttachOption.ReadOnly(Some(b))         => text("READ_ONLY") <<>> if (b) text("true") else text("false")
        case AttachOption.AttachType(attachType)    => text("TYPE") <<>> ppAttachType(attachType)
    }

    def ppAttachType(x: AttachType): Doc = match x {
        case AttachType.Sqlite => text("sqlite")
    }

    pub def ppCreateSchemaStmt(x: CreateSchemaStmt): Doc = match x {
        case CreateSchemaStmt.CreateSchemaStmt(optIfNotExists, name) => {
            let ifNotExists = someOrEmpty(optIfNotExists, ppIfNotExists);
            text("CREATE SCHEMA") <^> ifNotExists <^> text(name)
        }
    }

    pub def ppCreateTableStmt(x: CreateTableStmt): Doc = match x {
        case CreateTableStmt.CreateTableStmt(optReplace, tempB, optIfNotExists, qname, cols, tblConstrs) => {
            let orReplace = someOrEmpty(optReplace, ppOrReplace);
            let temporary = someOrEmpty(tempB, ppTemporary);
            let ifNotExists = someOrEmpty(optIfNotExists, ppIfNotExists);
            let colsDs = List.map(match (n,ty) -> text(n) <<>> ppColumnSpec(ty), cols);
            let tblConstrsD = sep(List.map(ppTableConstraint, tblConstrs));
            text("CREATE") <^> orReplace <^> temporary <<>> text("TABLE") <^> ifNotExists <<>> ppQualifiedName(qname) 
                <<>> optionalTupled(colsDs ++ List#{tblConstrsD})
        }
    }

    def ppOrReplace(x: OrReplace): Doc = match x {
        case OrReplace.OrReplace => text("OR REPLACE")
    }

    def ppTemporary(x: Temporary): Doc = match x {
        case Temporary.Temporary    => text("TEMPORARY")
        case Temporary.Temp         => text("TEMP")
    }

    def ppIfNotExists(x: IfNotExists): Doc = match x {
        case IfNotExists.IfNotExists => text("IF NOT EXISTS")
    }

    def ppIfExists(x: IfExists): Doc = match x {
        case IfExists.IfExists => text("IF EXISTS")
    }

    def ppColumnSpec(x: ColumnSpec): Doc = match x {
        case ColumnSpec.Column(ty, colConstraints)     => { 
            let columnConstraints = sep(List.map(ppColumnConstraint, colConstraints));
            ppDataType(ty) <^> columnConstraints
        }
        case ColumnSpec.GeneratedColumn(col)    => ppGeneratedColumn(col)
    }

    def ppGeneratedColumn(x: GeneratedColumn): Doc = match x { 
        case GeneratedColumn.GeneratedColumn(optType, optGenAlways, expr, optVoS) => {
            let sqltype = someOrEmpty(optType, ppDataType);
            let generatedAlways = someOrEmpty(optGenAlways, ppGeneratedAlways);
            let virtualOrStored = someOrEmpty(optVoS, ppVirtualOrStored);
            sqltype <^> generatedAlways <^> text("AS") <<>> parens(ppExpression(expr)) <^> virtualOrStored
        }
    }


    def ppGeneratedAlways(x: GeneratedAlways): Doc = match x {
        case GeneratedAlways.GeneratedAlways => text("GENERATED ALWAYS")
    }


    def ppVirtualOrStored(x: VirtualOrStored): Doc = match x {
        case VirtualOrStored.Virtual    => text("VIRTUAL")
        case VirtualOrStored.Stored     => text("STORED")
    }

    def ppColumnConstraint(x: ColumnConstraint): Doc = match x {
        case ColumnConstraint.PrimaryKey            => text("PRIMARY KEY")
        case ColumnConstraint.AllowNull(b)          => if (b) text("NULL") else text("NOT NULL")
        case ColumnConstraint.Unique                => text("UNIQUE")
        case ColumnConstraint.Check(expr)           => text("CHECK") <<>> parens(ppExpression(expr)) 
        case ColumnConstraint.Default(Left(lit))    => text("DEFAULT") <<>> ppLiteral(lit)
        case ColumnConstraint.Default(Right(expr))  => text("DEFAULT") <<>> parens(ppExpression(expr))
        case ColumnConstraint.References(ft, names) => text("REFERENCES") <<>> text(ft) <<>> tupled(List.map(text, names))
        case ColumnConstraint.Collate(cname)        => text("COLLATE") <<>> text(cname)
    }

    def ppTableConstraint(x: TableConstraint): Doc = match x {
        case TableConstraint.PrimaryKey(cols)               => text("PRIMARY KEY") <<>> tupled(List.map(text, cols))
        case TableConstraint.Unique(cols)                   => text("UNIQUE") <<>> tupled(List.map(text, cols))
        case TableConstraint.Check(expr)                    => text("CHECK") <<>> parens(ppExpression(expr)) 
        case TableConstraint.ForeignKey(cols, ft, fcols)    => 
            text("FOREIGN KEY") <<>> tupled(List.map(text, cols)) <<>> text("REFERENCES") <<>> text(ft) <<>> tupled(List.map(text, fcols))
    }

    pub def ppDeleteStmt(x: DeleteStmt): Doc = match x {
        case DeleteStmt.DeleteStmt(qname, optWhere) => 
            let whereClause = someOrEmpty(optWhere, ppExpression);
        text("DELETE FROM") <<>> ppQualifiedName(qname) <^> whereClause
    }

    pub def ppDetachStmt(x: DetachStmt): Doc = match x {
        case DetachStmt.DetachStmt(optQual, name) => 
            let qualifier = someOrEmpty(optQual, ppDetachQualifier);
        text("DETACH") <^> qualifier <^> text(name)
    }


    def ppDetachQualifier(x: DetachQualifier): Doc = match x {
        case DetachQualifier.DetachDatabase(optIfExists) => {
            let ifExists = someOrEmpty(optIfExists, ppIfExists);
            text("DATABASE") <^> ifExists
        }
    }


    pub def ppDropStmt(x: DropStmt): Doc = match x {
        case DropStmt.DropStmt(tyqual, optIfExists, qname, optCascadeRestrict) => 
            let ifExists = someOrEmpty(optIfExists, ppIfExists);
            let cascadeRestrict = someOrEmpty(optCascadeRestrict, ppCascadeOrRestrict);
            text("DROP") <<>> ppDropQualifier(tyqual) <^> ifExists <<>> ppQualifiedName(qname) <^> cascadeRestrict
    }

    def ppDropQualifier(x: DropQualifier): Doc = match x {
        case DropQualifier.Function     => text("FUNCTION")
        case DropQualifier.Index        => text("SEQUENCE")
        case DropQualifier.Macro        => text("MACRO")
        case DropQualifier.MacroTable   => text("MACRO") <<>> text("TABLE")
        case DropQualifier.Schema       => text("SCHEMA")
        case DropQualifier.Sequence     => text("SEQUENCE")
        case DropQualifier.Table        => text("TABLE")
        case DropQualifier.View         => text("VIEW")
        case DropQualifier.Type         => text("TYPE")
    }
    
    def ppCascadeOrRestrict(x: CascadeOrRestrict): Doc = match x {
        case CascadeOrRestrict.Cascade  => text("CASCADE")
        case CascadeOrRestrict.Restrict => text("RESTRICT")
    }

    pub def ppInsertStmt(x:  InsertStmt): Doc = match x {
        case InsertStmt.InsertStmt(optInsQual, qname, optAsName, cols, optColQual, valClause, ocClause, optRetClause) => 
            let insQual = someOrEmpty(optInsQual, ppInsertQualifier);
            let asName = someOrEmpty(optAsName, name -> text("AS") <<>> ppQualifiedName(name));
            let colsD = optionalTupled(List.map(text, cols));
            let colQual = someOrEmpty(optColQual, ppInsertColumnQualifier);
            let onConflict = someOrEmpty(ocClause, ppOnConflictClause);
            let retClause = someOrEmpty(optRetClause, ppInsertReturningClause);
            text("INSERT") <^> insQual <<>> text("INTO") 
                <<>> ppQualifiedName(qname) 
                <^> asName 
                <^> colsD <^> colQual
                <<>> ppInsertValuesClause(valClause)
                <^> onConflict
                <^> retClause
    }

    def ppInsertQualifier(x: InsertQualifier): Doc = match x {
        case InsertQualifier.OrReplace  => text("OR REPLACE")
        case InsertQualifier.OrIgnore   => text("OR IGNORE")
    }

    def ppInsertColumnQualifier(x: InsertColumnQualifier): Doc = match x {
        case InsertColumnQualifier.ByPosition   => text("BY POSITION")
        case InsertColumnQualifier.ByName       => text("BY NAME")
    }


    def ppInsertValuesClause(x: InsertValuesClause): Doc = match x {
        case InsertValuesClause.Values(vals)    => ppValuesClause(vals)
        case InsertValuesClause.Select(selStmt) => ppSelectStmt(selStmt)
        case InsertValuesClause.DefaultValues   => text("DEFAULT VALUES")
    }

    def ppValuesClause(x: ValuesClause): Doc = match x {
        case ValuesClause.ValuesClause(xss)     => {
            let ppValuesList = xs -> optionalTupled(List.map(ppExpression, xs));
            text("VALUES") <<>> commaSpaceSep(List.map(ppValuesList, xss))
        }
    }

    def ppOnConflictClause(x: OnConflictClause): Doc = match x {
        case OnConflictClause.OnConflictClause(cols, optWhereExpr, dudn) => {
            let columns = optionalTupled(List.map(text, cols));
            let whereExpr = someOrEmpty(optWhereExpr, expr -> text("WHERE") <<>> ppExpression(expr));
            text("ON CONFLICT") <^> columns <^> whereExpr <<>> ppDoUpdateOrDoNothing(dudn)
        }
    }


    def ppDoUpdateOrDoNothing(x: DoUpdateOrDoNothing): Doc = match x {
        case DoUpdateOrDoNothing.DoUpdate(setExprs, optWhereExpr)   => {
            let setCols = commaSpaceSep(List.map(match (col, expr) -> text(col) <<>> equals() <<>> ppExpression(expr), setExprs));
            let whereExpr = someOrEmpty(optWhereExpr, expr -> text("WHERE") <<>> ppExpression(expr));
            text("DO UPDATE") <<>> text("SET") <<>> setCols <^> whereExpr
        }
        case DoUpdateOrDoNothing.DoNothing                          => text("DO NOTHING")
    }

    def ppInsertReturningClause(x: ReturningClause): Doc = match x {
        case ReturningClause.ReturningClause(retExprs) => 
            text("RETURNING") <<>> commaSpaceSep(List.map(ppReturnExpression, retExprs))
    }

    def ppReturnExpression(x: ReturnExpression): Doc = match x {
        case ReturnExpression.ReturnStar                                        => text("*")
        case ReturnExpression.ReturnExpression(expr, None)                      => ppExpression(expr)
        case ReturnExpression.ReturnExpression(expr, Some(showAs, exprAlias))   => 
            ppExpression(expr) <^> ppShowAsKeyword(showAs) <^> text(exprAlias)
    }



    pub def ppSelectStmt(x: SelectStmt): Doc = match x {
        case SelectStmt.SelectStmt(selClause, fromClause, optWhere, optGroupBy, optHaving, optWindow, optQualify, optOrderBy, optLimit) => {
            ppSelectClause(selClause) 
                <<>> ppFromClause(fromClause) 
                <^> someOrEmpty(optWhere, ppWhereClause)
                <^> someOrEmpty(optGroupBy, ppGroupByClause)
                <^> someOrEmpty(optHaving, ppHavingClause)
                <^> someOrEmpty(optWindow, ppWindowClause)
                <^> someOrEmpty(optQualify, ppQualifyClause)
                <^> someOrEmpty(optOrderBy, ppOrderByClause)
                <^> someOrEmpty(optLimit, ppLimitClause)
        }
    }

    def ppSelectClause(x: SelectClause): Doc = match x {
        case SelectClause.SelectClause(optDistinct, sels) => {
            let distinct = someOrEmpty(optDistinct, ppDistinctClause);
            let selectExprs = commaSpaceSep(List.map(ppSelectExpr, sels));
            text("SELECT") <^> distinct <<>> selectExprs
        }
    }

    def ppDistinctClause(x: DistinctClause): Doc = match x {
        case DistinctClause.Distinct(None)         => text("DISTINCT")
        case DistinctClause.Distinct(Some(exprs))  => text("DISTINCT") <<>> text("ON") <<>> tupled(List.map(ppExpression, exprs))
        case DistinctClause.All                    => text("ALL")
    }

    def ppSelectExpr(x: SelectExpr): Doc = match x {
        case SelectExpr.SelectExpr(expr, None)                  => ppExpression(expr) 
        case SelectExpr.SelectExpr(expr, Some(showAs, nameAlias))  => 
            ppExpression(expr) <^> ppShowAsKeyword(showAs) <^> text(nameAlias)
        case SelectExpr.SelectStar(optTable, starOpts)          => {
            let tableName = someOrEmpty(optTable, s -> text(s) ++ dot());
            tableName <^> char('*') <<>> ppStarOptions(starOpts)
        }
        case SelectExpr.SelectColumns(colExpr)                  => text("COLUMNS") <<>> parens(ppColumnExpression(colExpr))
        
    }

    def ppStarOptions(x: StarOptions): Doc = match x {
        case StarOptions.StarOptions(optExcludes, optReplaces) => {
            let excludes = someOrEmpty(optExcludes, xs -> text("EXCLUDE") <<>> tupled(List.map(text, xs)));
            let makeReplace = match (expr, name) -> ppExpression(expr) <<>> text("AS") <<>> text(name); 
            let replaces = someOrEmpty(optReplaces, xs -> text("REPLACE") <<>> tupled(List.map(makeReplace, xs)));
            excludes <^> replaces
        }
    }

    def ppColumnExpression(x: ColumnExpression): Doc = match x {
        case ColumnExpression.ColumnStar(optTable, starOpts)    => {
            let tableName = someOrEmpty(optTable, s -> text(s) ++ dot());
            tableName <^> char('*') <<>> ppStarOptions(starOpts)
        }
        case ColumnExpression.ColumnRegex(regex)                => squotes(text(regex))
        case ColumnExpression.ColumnLambda(ident, lam)          => { 
            
            text(ident) <<>> text("->") <<>> ppLambdaFilter(lam)
        }
    }


    def ppLambdaFilter(x: LambdaFilter): Doc = match x {
        case LambdaFilter.LambdaFilter(expr) => ppExpression(expr)
    }

    def ppFromClause(x: FromClause): Doc = match x {
        case FromClause.FromClause(exprs) => { 
            let exprList = commaSpaceSep(List.map(ppFromExpressionOrJoin, exprs));
            text("FROM") <<>> exprList
        }
    }

    def ppFromExpressionOrJoin(x: FromExpressionOrJoin): Doc = match x {
        case FromExpressionOrJoin.FromExpression(expr, optAlias, optSample) => {
            let tableAlias = someOrEmpty(optAlias, ppTableAlias);
            let sample = someOrEmpty(optSample, ppTableSample);
            ppFromExpression(expr) <^> tableAlias <^> sample
        }
        case FromExpressionOrJoin.Join(useParens, joinClause, optAlias) => {
            let joinAlias = someOrEmpty(optAlias, s -> text("AS") <<>> text(s));
            showParens(useParens, ppJoinClause(joinClause) <^> joinAlias)
        }
    }

    def ppFromExpression(x: FromExpression): Doc = match x {
        case FromExpression.TableReference(qname)       => ppQualifiedName(qname)
        case FromExpression.TableFunction(qname, exprs) => ppQualifiedName(qname) ++ tupled(List.map(ppExpression, exprs))
        case FromExpression.Subquery(stmt)              => parens(ppSelectStmt(stmt))
        case FromExpression.Values(valsClause)          => parens(ppValuesClause(valsClause))
    }

    def ppJoinClause(x: JoinClause): Doc = match x { 
        case JoinClause.JoinClause(tableA, optJoinDes, tableB, optJoinCond) => {
            let designator = someOrEmpty(optJoinDes, ppJoinDesignation);
            let joinCondition = someOrEmpty(optJoinCond, ppJoinCondition);
            ppTableOrSubquery(tableA) <^> designator <<>> text("JOIN") <<>> ppTableOrSubquery(tableB) <^> joinCondition
        }
    }

    def ppTableOrSubquery(x: TableOrSubquery): Doc = match x { 
        case TableOrSubquery.Table(name)    => text(name)
        case TableOrSubquery.Subquery(stmt) => ppSelectStmt(stmt)
    }


    def ppJoinDesignation(x: JoinDesignation): Doc = match x { 
        case JoinDesignation.Join(optNatAsof, joinType) => someOrEmpty(optNatAsof, ppNaturalOrAsOf) <^> ppJoinType(joinType)
        case JoinDesignation.Positional                 => text("POSITIONAL")
        case JoinDesignation.Cross                      => text("CROSS")
    }

    def ppJoinCondition(x: JoinCondition): Doc = match x { 
        case JoinCondition.On(expr)     => text("ON") <<>> ppExpression(expr)
        case JoinCondition.Using(cols)  => text("USING") <<>> tupled(List.map(text, cols))
    }

    def ppNaturalOrAsOf(x: NaturalOrAsOf): Doc = match x { 
        case NaturalOrAsOf.Natural    => text("NATURAL")
        case NaturalOrAsOf.AsOf       => text("ASOF")
    }

    def ppJoinType(x: JoinType): Doc = match x { 
        case JoinType.Inner         => text("OUTER")
        case JoinType.Left(opt)     => text("LEFT") <^> someOrEmpty(opt, ppOuterJoin)
        case JoinType.Right(opt)    => text("RIGHT") <^> someOrEmpty(opt, ppOuterJoin)
        case JoinType.Full(opt)     => text("FULL") <^> someOrEmpty(opt, ppOuterJoin)
    }

    def ppOuterJoin(x: OuterJoin): Doc = match x { 
        case OuterJoin.Outer => text("OUTER")
    }

    def ppTableAlias(x: TableAlias): Doc = match x {
        case TableAlias.TableAlias(showAs, tableAlias, optCols) => {
            let columns = someOrEmpty(optCols, cols -> optionalTupled(List.map(text, cols)));
            ppShowAsKeyword(showAs) <^> text(tableAlias) <^> columns
        }
    }

    def ppTableSample(x: TableSample): Doc = match x { 
        case TableSample.SizeFirst(sz, optUnit, optMethod)              => {
            let sampleUnit = someOrEmpty(optUnit, ppPercentOrRowCount);
            let sampleMethod = someOrEmpty(optMethod, 
                match (s, optCount) -> parens(ppSamplingMethod(s) <^> someOrEmpty(optCount, i -> comma() <<>> int32(i))));
            int32(sz) <^> sampleUnit <^> sampleMethod
        }
        case TableSample.FunctionStyle(optMethod, sz, optUnit, optSeed) => {
            let sampleMethod = someOrEmpty(optMethod, ppSamplingMethod);
            let sampleUnit = someOrEmpty(optUnit, ppPercentOrRowCount);
            let seed = someOrEmpty(optSeed, i -> text("REPEATABLE") <<>> parens(int32(i)));
            sampleMethod <^> parens(int32(sz) <^> sampleUnit) <^> seed
        }
    }


    def ppPercentOrRowCount(x: PercentOrRow): Doc = match x {
        case PercentOrRow.Percent       => text("PERCENT")
        case PercentOrRow.PercentSymbol => char('%')
        case PercentOrRow.Row           => text("ROWS")
    }

    def ppSamplingMethod(x: SamplingMethod): Doc = match x {
        case SamplingMethod.Reservoir   => text("reservoir")
        case SamplingMethod.Bernoulli   => text("bernoulli")
        case SamplingMethod.System      => text("system")
    }


    def ppWhereClause(x: WhereClause): Doc = match x {
        case WhereClause.WhereClause(expr) => text("WHERE") <<>> ppExpression(expr)
    }


    def ppGroupByClause(x: GroupByClause): Doc = match x {
        case GroupByClause.GroupBy(xs)  => text("GROUP BY") <<>> commaSpaceSep(List.map(ppGrouping, xs))
        case GroupByClause.GroupByAll   => text("GROUP BY") <<>> text("ALL")
    }

    def ppGrouping(x: Grouping): Doc = match x {
        case Grouping.GroupingExpr(expr)            => ppExpression(expr)
        case Grouping.GroupingSets(xss)             => tupled(List.map(tupled, List.map(List.map(ppExpression), xss)))
        case Grouping.GroupingSimple(cor, exprs)    => ppCubeOrRollup(cor) <<>> tupled(List.map(ppExpression, exprs))
    }

    def ppCubeOrRollup(x: CubeOrRollup): Doc = match x {
        case CubeOrRollup.Cube   => text("CUBE")
        case CubeOrRollup.Rollup => text("ROLLUP")
    }

    def ppHavingClause(x: HavingClause): Doc = match x {
        case HavingClause.HavingClause(expr) => text("HAVING") <<>> ppExpression(expr)
    }

    def ppWindowClause(x: WindowClause): Doc = match x {
        case WindowClause.WindowClause(winDecls) => {
            let makeWindowDecl = match (name, defn) -> text(name) <<>> text("AS") <<>> ppWindowDefinition(defn);
            text("WINDOW") <<>> commaSpaceSep(List.map(makeWindowDecl, winDecls))
        }
    }

    def ppWindowDefinition(x: WindowDefinition): Doc = match x {
        case WindowDefinition.WindowDefinition(optBaseName, optPartitionBy, optOrderBy, frameSpec) => {
            let baseName = someOrEmpty(optBaseName, text);
            let partitionBy = someOrEmpty(optPartitionBy, ppPartitionByClause);
            let orderBy = someOrEmpty(optOrderBy, ppOrderByClause);
            parens(baseName <^> partitionBy <^> orderBy <<>> ppFrameSpec(frameSpec))
        }

    }

    def ppPartitionByClause(x: PartitionByClause): Doc = match x {
        case PartitionByClause.PartitionByClause(exprs) => text("PARTITION BY") <<>> commaSpaceSep(List.map(ppExpression, exprs))
    }

    def ppFrameSpec(x: FrameSpec): Doc = match x {
        case FrameSpec.FrameSpec(ror, frameBody, optExclude) => {
            let exclude = someOrEmpty(optExclude, ppWindowExclude);
            ppRangeOrRows(ror) <<>> ppFrameBody(frameBody) <^> exclude
        }
    }

    def ppRangeOrRows(x: RangeOrRows): Doc = match x {
        case RangeOrRows.Range  => text("RANGE")
        case RangeOrRows.Rows   => text("ROWS")
    }

    def ppFrameBody(x: FrameBody): Doc = match x {
        case FrameBody.Between(frameStart, frameStop) => 
            text("BETWEEN") <<>> ppFrameStart(frameStart) <<>> text("AND") <<>> ppFrameStop(frameStop) 
        case FrameBody.UnboundedPreceding             => text("UNBOUNDED PRECEDING")
        case FrameBody.Preceding(expr)                => ppExpression(expr) <<>> text("PRECEDING")
        case FrameBody.CurrentRow                     => text("CURRENT ROW")
    }


    def ppFrameStart(x: FrameStart): Doc = match x {
        case FrameStart.UnboundedPreceding => text("UNBOUNDED FOLLOWING")
        case FrameStart.Preceding(expr)    => ppExpression(expr) <<>> text("PRECEDING")
        case FrameStart.CurrentRow         => text("CURRENT ROW")
        case FrameStart.Following(expr)    => ppExpression(expr) <<>> text("FOLLOWING")
    }

    def ppFrameStop(x: FrameStop): Doc = match x {
        case FrameStop.Preceding(expr)    => ppExpression(expr) <<>> text("PRECEDING")
        case FrameStop.CurrentRow         => text("CURRENT ROW")
        case FrameStop.Following(expr)    => ppExpression(expr) <<>> text("FOLLOWING")
        case FrameStop.UnboundedFollowing => text("UNBOUNDED FOLLOWING")
    }

    def ppWindowExclude(x: WindowExclude): Doc = text("EXCLUDE") <<>> match x {
        case WindowExclude.CurrentRow   => text("CURRENT ROW")
        case WindowExclude.Group        => text("GROUP")
        case WindowExclude.Ties         => text("TIES")
        case WindowExclude.NoOthers     => text("NO OTHERS")
    }

    def ppQualifyClause(x: QualifyClause): Doc = match x {
        case QualifyClause.QualifyClause(expr) => text("QUALIFY") <<>> ppExpression(expr)
    }

    def ppOrderByClause(x: OrderByClause): Doc = match x {
        case OrderByClause.OrderByExprs(xs)      => { 
            let makeExprOrdering = match (expr, orderDes) -> ppExpression(expr) <^> ppOrderDesignation(orderDes);
            commaSpaceSep(List.map(makeExprOrdering, xs))
        }
        case OrderByClause.OrderByAll(orderDes)  => text("ALL") <^> ppOrderDesignation(orderDes)
    }

    def ppOrderDesignation(x: OrderDesignation): Doc = match x {
        case OrderDesignation.OrderDesignation(optSortOrder, optNullsPosition) => {
            let order = someOrEmpty(optSortOrder, ppSortOrder);
            let nullsPos = someOrEmpty(optNullsPosition, ppNullsPosition);
            order <^> nullsPos
        }

    }

    def ppSortOrder(x: SortOrder): Doc = match x {
        case SortOrder.Asc  => text("ASC")
        case SortOrder.Desc => text("DESC")
    }

    def ppNullsPosition(x: NullsPosition): Doc = match x {
        case NullsPosition.First    => text("FIRST")
        case NullsPosition.Last     => text("LAST")
    }

    def ppLimitClause(x: LimitClause): Doc = match x {
        case LimitClause.LimitClause(expr, optOffset) => {
            let offset = someOrEmpty(optOffset, expr1 -> text("OFFSET") <<>> ppExpression(expr1));
            text("LIMIT") <<>> ppExpression(expr) <^> offset
        }
    }

    pub def ppUseStmt(x: UseStmt): Doc= match x {
        case UseStmt.UseStmt(qname) => 
            text("USE") <<>> ppQualifiedName(qname)
    }

    // NOTE - we might need various printers for types...
    pub def ppDataType(x: DataType): Doc = match x {
        case DataType.Array(ty, sz)     => ppDataType(ty) ++ brackets(int32(sz))
        case DataType.Bit               => text("BIT")
        case DataType.Blob              => text("BLOB")
        case DataType.Boolean           => text("BOOLEAN")
        case DataType.Date              => text("DATE")
        case DataType.FixedDecimal(w,sc)    => text("DECIMAL") ++ tupled(List#{int32(w), int32(sc)})
        case DataType.FloatingPointDecimal(tyFP)    => ppFloatingPointType(tyFP)
        case DataType.Integer(tyInt)        => ppIntegerType(tyInt)
        case DataType.Interval          => text("INTERVAL")
        case DataType.List(ty)          => ppDataType(ty) ++ text("[]")
        case DataType.Map(kty, vty)     => text("MAP") ++ tupled(List#{ppDataType(kty), ppDataType(vty)})
        case DataType.Time              => text("TIME")
        case DataType.TimeTz            => text("TIMETZ")
        case DataType.Timestamp(prec)   => timestampPrecAsDataType(prec)
        case DataType.UUID              => text("UUID")
        case DataType.Varchar           => text("VARCHAR")
    }

    def ppIntegerType(x: IntegerType): Doc = match x {
        case IntegerType.TinyInt    => text("TINYINT")
        case IntegerType.SmallInt   => text("SMALLINT")
        case IntegerType.Integer    => text("INTEGER")
        case IntegerType.BigInt     => text("BIGINT")
        case IntegerType.HugeInt    => text("HUGEINT")
        case IntegerType.UTinyInt   => text("UTINYINT")
        case IntegerType.USmallInt  => text("USMALLINT")
        case IntegerType.UInteger   => text("UINTEGER")
        case IntegerType.UBigInt    => text("UBIGINT")
        case IntegerType.UHugeInt   => text("UHUGEINT")
    }


    def ppFloatingPointType(x: FloatingPointType): Doc = match x {
        case FloatingPointType.Real     => text("REAL")
        case FloatingPointType.Double   => text("DOUBLE")
    }

    def timestampPrecAsDataType(x: TimestampPrecision): Doc = match x {
        case TimestampPrecision.Nanosecond  => text("TIMESTAMP_NS")
        case TimestampPrecision.Microsecond => text("TIMESTAMP")
        case TimestampPrecision.Millisecond => text("TIMESTAMP_MS")
        case TimestampPrecision.Second      => text("TIMESTAMP_S")
        case TimestampPrecision.Timezone    => text("TIMESTAMPTZ")
    }
}
