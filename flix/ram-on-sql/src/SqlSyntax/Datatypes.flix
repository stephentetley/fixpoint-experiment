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

mod SqlSyntax.Datatypes {


    pub enum ShowParens with Eq, ToString {
        case ShowParens
        case NoParens
    }

    pub enum ShowAsKeyword with Eq, ToString {
        case ShowAs
        case NoAs
    }

    pub enum ShowColumnKeyword with Eq, ToString {
        case ShowColumn
        case NoColumn
    }

    pub enum ShowDatabaseKeyword with Eq, ToString {
        case ShowDatabase
        case NoDatabase
    }

    pub enum QualifiedName with Eq, ToString {
        case QualifiedName(String, String)
        case UnqualifiedName(String)
    }


    pub enum Expression with Eq, ToString {
        case Identifier(String)
        case Literal(Literal)
        case Binary(Expression, BinOp, Expression)
        case Case(CaseStmt)
        case Cast(Expression, DataType)
        case TryCast(Expression, DataType)
        case Collation(Expression, Collation)
    }



    pub enum Literal with Eq, ToString {
        case Null
        case Integer(Int32) // TODO
        case Varchar(String)
    }


    pub enum BinOp with Eq, ToString {
        case LessThan
        case GreaterThan
        case LessThanEquals
        case GreaterThanEquals
        case Equals
        case NotEquals
    }

    pub enum CaseStmt with Eq, ToString {
        case CaseStmt(Option[Expression], List[(Expression, Expression)], Option[Expression])
    }


    pub enum Collation with Eq, ToString {
        case NoCase
        case NoAccent
        case NFC
        case Extension(String)
    }

    // Columns and primary keys
    pub enum Statement with Eq, ToString {
        case AlterTable(AlterTableStmt)
        case AlterView(AlterViewStmt)
        case Attach(AttachStmt)
        case CreateSchema(CreateSchemaStmt)
        case CreateTable(CreateTableStmt)
        case Delete(DeleteStmt)
        case Detach(DetachStmt)
        case Drop(DropStmt)
        case Insert(InsertStmt)
        case Select(SelectStmt)
        case Use(UseStmt)
    }

    pub enum AlterTableStmt with Eq, ToString {
        case AlterTableStmt(QualifiedName, AlterTableClause)
    }

    pub enum AlterTableClause with Eq, ToString {
        case Add(ShowColumnKeyword, Option[IfNotExists], String, DataType, List[ColumnConstraint])
        case Drop(ShowColumnKeyword, Option[IfExists], String)
        case Alter(ShowColumnKeyword, String, AlterColumnOptions)
        case Rename(ShowColumnKeyword, String, String)
        case RenameTo(String)
    }

    pub enum AlterColumnOptions with Eq, ToString {
        case SetData(Option[SetData], DataType, Option[String], Option[Expression])
        case SetDefault(Expression)
        case DropDefault
    }


    pub enum SetData with Eq, ToString {
        case SetData
    }


    pub enum AlterViewStmt with Eq, ToString {
        case AlterViewStmt(QualifiedName, AlterViewClause)
    }

    pub enum AlterViewClause with Eq, ToString {
        case RenameTo(String)
    }

    pub enum AttachStmt with Eq, ToString {
        case AttachStmt(ShowDatabaseKeyword, Option[IfNotExists], DatabasePath, Option[DatabaseAlias], Option[List[AttachOption]])
    }

    pub type alias DatabasePath = String
    pub type alias DatabaseAlias = String

    pub enum AttachOption with Eq, ToString {
        case ReadOnly(Option[Bool])
        case AttachType(AttachType)
    }

    pub enum AttachType with Eq, ToString {
        case Sqlite
    }

    pub enum CreateSchemaStmt with Eq, ToString {
        case CreateSchemaStmt(Option[IfNotExists], String)
    }

    pub enum CreateTableStmt with Eq, ToString {
        case CreateTableStmt(Option[OrReplace], Option[Temporary], Option[IfNotExists], QualifiedName, List[(String, ColumnSpec)], List[TableConstraint])
    }

    pub enum OrReplace with Eq, ToString  {
        case OrReplace
    }

    pub enum Temporary with Eq, ToString  {
        case Temporary
        case Temp
    }

    pub enum IfNotExists with Eq, ToString  {
        case IfNotExists
    }

    pub enum IfExists with Eq, ToString  {
        case IfExists
    }


    pub enum ColumnSpec with Eq, ToString {
        case Column(DataType, List[ColumnConstraint])
        case GeneratedColumn(GeneratedColumn)
    }

    pub enum GeneratedColumn with Eq, ToString {
        case GeneratedColumn(Option[DataType], Option[GeneratedAlways], Expression, Option[VirtualOrStored])
    }

    pub enum GeneratedAlways with Eq, ToString {
        case GeneratedAlways
    }

    pub enum VirtualOrStored with Eq, ToString {
        case Virtual
        case Stored
    }

    pub enum ColumnConstraint with Eq, ToString {
        case PrimaryKey
        case AllowNull(Bool)
        case Unique
        case Check(Expression)
        case Default(Either[Literal, Expression])      // TODO left should be literal
        case References(String, List[String])
        case Collate(String)
    }

    pub enum TableConstraint with Eq, ToString {
        case PrimaryKey(List[String])
        case Unique(List[String])
        case Check(Expression)
        case ForeignKey(List[String], String, List[String])
    }

    // Missing USING ...
    pub enum DeleteStmt with Eq, ToString {
        case DeleteStmt(QualifiedName, Option[Expression])
    }

    pub enum DetachStmt with Eq, ToString {
        case DetachStmt(Option[DetachQualifier], String)
    }

    pub enum DetachQualifier with Eq, ToString {
        case DetachDatabase(Option[IfExists])
    }

    pub enum DropStmt with Eq, ToString {
        case DropStmt(DropQualifier, Option[IfExists], QualifiedName, Option[CascadeOrRestrict])
    }

    pub enum DropQualifier with Eq, ToString {
        case Function
        case Index
        case Macro
        case MacroTable
        case Schema
        case Sequence
        case Table
        case View
        case Type
    }

    pub enum CascadeOrRestrict with Eq, ToString {
        case Cascade
        case Restrict
    }


    pub enum InsertStmt with Eq, ToString {
        case InsertStmt(Option[InsertQualifier], QualifiedName, Option[QualifiedName], List[ColumnName], Option[InsertColumnQualifier], InsertValuesClause, Option[OnConflictClause], Option[ReturningClause])
    }

    pub enum InsertQualifier with Eq, ToString {
        case OrReplace
        case OrIgnore
    }

    pub type alias ColumnName = String

    pub enum InsertColumnQualifier with Eq, ToString {
        case ByPosition
        case ByName
    }


    pub enum InsertValuesClause with Eq, ToString {
        case Values(ValuesClause)
        case Select(SelectStmt)
        case DefaultValues
    }

    pub enum ReturningClause with Eq, ToString {
        case ReturningClause(List[ReturnExpression])
    }

    pub enum ReturnExpression with Eq, ToString {
        case ReturnStar
        case ReturnExpression(Expression, Option[(ShowAsKeyword, String)])
    }

    pub enum OnConflictClause with Eq, ToString {
        case OnConflictClause(List[String], Option[Expression], DoUpdateOrDoNothing)
    }

    pub enum DoUpdateOrDoNothing with Eq, ToString {
        case DoUpdate(List[(String, Expression)], Option[Expression])
        case DoNothing
    }

    pub enum ValuesClause with Eq, ToString {
        case ValuesClause(List[List[Expression]])
    }
    pub enum SelectStmt with Eq, ToString {
        case SelectStmt(SelectClause, FromClause, Option[WhereClause], Option[GroupByClause], Option[HavingClause], Option[WindowClause], Option[QualifyClause], Option[OrderByClause], Option[LimitClause])
    }

    pub enum SelectClause with Eq, ToString {
        case SelectClause(Option[DistinctClause], List[SelectExpr])
    }

    pub enum DistinctClause with Eq, ToString {
        case Distinct(Option[List[Expression]])
        case All
    }

    pub enum SelectExpr with Eq, ToString {
        case SelectExpr(Expression, Option[(ShowAsKeyword, String)])
        case SelectStar(Option[String], StarOptions)
        case SelectColumns(ColumnExpression)
    }

    pub enum StarOptions with Eq, ToString {
        case StarOptions(Option[List[String]], Option[List[(Expression, String)]])
    }

    pub enum ColumnExpression with Eq, ToString {
        case ColumnStar(Option[String], StarOptions)
        case ColumnRegex(String)
        case ColumnLambda(String, LambdaFilter)
    }

    pub enum LambdaFilter with Eq, ToString {
        case LambdaFilter(Expression)
    }

    pub enum FromClause with Eq, ToString {
        case FromClause(List[FromExpressionOrJoin])
    }

    pub enum FromExpressionOrJoin with Eq, ToString {
        case FromExpression(FromExpression, Option[TableAlias], Option[TableSample])
        case Join(ShowParens, JoinClause, Option[String])
    }


    pub enum FromExpression with Eq, ToString {
        case TableReference(QualifiedName)
        case TableFunction(QualifiedName, List[Expression])
        case Subquery(SelectStmt)
        case Values(ValuesClause)
    }

    pub enum JoinClause with Eq, ToString {
        case JoinClause(TableOrSubquery, Option[JoinDesignation], TableOrSubquery, Option[JoinCondition])
    }

    pub enum TableOrSubquery with Eq, ToString {
        case Table(String)
        case Subquery(SelectStmt)
    }


    pub enum JoinDesignation with Eq, ToString {
        case Join(Option[NaturalOrAsOf], JoinType)
        case Positional
        case Cross
    }

    pub enum JoinCondition with Eq, ToString {
        case On(Expression)
        case Using(List[String])
    }

    pub enum NaturalOrAsOf with Eq, ToString {
        case Natural
        case AsOf
    }

    pub enum JoinType with Eq, ToString {
        case Inner
        case Left(Option[OuterJoin])
        case Right(Option[OuterJoin])
        case Full(Option[OuterJoin])
    }

    pub enum OuterJoin with Eq, ToString {
        case Outer
    }

    pub enum TableAlias with Eq, ToString {
        case TableAlias(ShowAsKeyword, String, Option[List[String]])
    }

    pub enum TableSample with Eq, ToString {
        case SizeFirst(Int32, Option[PercentOrRow], Option[(SamplingMethod, Option[Int32])])
        case FunctionStyle(Option[SamplingMethod], Int32, Option[PercentOrRow], Option[Int32])
    }

    pub enum PercentOrRow with Eq, ToString {
        case Percent
        case PercentSymbol
        case Row
    }

    pub enum SamplingMethod with Eq, ToString {
        case Reservoir
        case Bernoulli
        case System
    }

    pub enum WhereClause with Eq, ToString {
        case WhereClause(Expression)
    }

    pub enum GroupByClause with Eq, ToString {
        case GroupBy(List[Grouping])
        case GroupByAll
    }

    pub enum Grouping with Eq, ToString {
        case GroupingExpr(Expression)
        case GroupingSets(List[List[Expression]])
        case GroupingSimple(CubeOrRollup, List[Expression])
    }

    pub enum CubeOrRollup with Eq, ToString {
        case Cube
        case Rollup
    }

    pub enum HavingClause with Eq, ToString {
        case HavingClause(Expression)
    }

    pub enum WindowClause with Eq, ToString {
        case WindowClause(List[(String, WindowDefinition)])
    }

    pub enum WindowDefinition with Eq, ToString {
        case WindowDefinition(Option[String], Option[PartitionByClause], Option[OrderByClause], FrameSpec)
    }

    pub enum PartitionByClause with Eq, ToString {
        case PartitionByClause(List[Expression])
    }

    pub enum FrameSpec with Eq, ToString {
        case FrameSpec(RangeOrRows, FrameBody, Option[WindowExclude])
    }

    pub enum RangeOrRows with Eq, ToString {
        case Range
        case Rows
    }

    pub enum FrameBody with Eq, ToString {
        case Between(FrameStart, FrameStop)
        case UnboundedPreceding
        case Preceding(Expression)
        case CurrentRow
    }

    pub enum FrameStart with Eq, ToString {
        case UnboundedPreceding
        case Preceding(Expression)
        case CurrentRow
        case Following(Expression)
    }

    pub enum FrameStop with Eq, ToString {
        case Preceding(Expression)
        case CurrentRow
        case Following(Expression)
        case UnboundedFollowing
    }

    pub enum WindowExclude with Eq, ToString {
        case CurrentRow
        case Group
        case Ties
        case NoOthers
    }

    pub enum QualifyClause with Eq, ToString {
        case QualifyClause(Expression)
    }


    pub enum OrderByClause with Eq, ToString {
        case OrderByExprs(List[(Expression, OrderDesignation)])
        case OrderByAll(OrderDesignation)
    }

    pub enum OrderDesignation with Eq, ToString {
        case OrderDesignation(Option[SortOrder], Option[NullsPosition])
    }

    pub enum SortOrder with Eq, ToString, Order {
        case Asc
        case Desc
    }

    pub enum NullsPosition with Eq, ToString, Order {
        case First
        case Last
    }

    pub enum LimitClause with Eq, ToString {
        case LimitClause(Expression, Option[Expression])
    }

    pub enum UseStmt with Eq,ToString {
        case UseStmt(QualifiedName)
    }

    pub enum DataType with Eq, ToString {
        case Array(DataType, Int32)
        case Bit
        case Blob
        case Boolean
        case Date
        case FixedDecimal(Int32, Int32)
        case FloatingPointDecimal(FloatingPointType)
        case Integer(IntegerType)
        case Interval
        case List(DataType)
        case Map(DataType, DataType)
        case Time
        case TimeTz
        case Timestamp(TimestampPrecision)
        case UUID
        case Varchar
    }

    pub enum IntegerType with Eq, ToString {
        case TinyInt
        case SmallInt
        case Integer
        case BigInt
        case HugeInt
        case UTinyInt
        case USmallInt
        case UInteger
        case UBigInt
        case UHugeInt
    }


    pub enum FloatingPointType with Eq, ToString {
        case Real
        case Double
    }

    pub enum TimestampPrecision with Eq, ToString {
        case Nanosecond
        case Microsecond
        case Millisecond
        case Second
        case Timezone
    }


}
