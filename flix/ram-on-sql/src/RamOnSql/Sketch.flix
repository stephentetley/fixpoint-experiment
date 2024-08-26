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

mod RamOnSql.Sketch {

    use Text.PrettyPrint.Doc
    use Text.PrettyPrint.{<<>>, text, parens, dquotes, intersperse, comma, semi}
    use RamOnSql.Datatypes.{Table}

    pub def purge(table: Table): Doc = 
        let tName =  RamOnSql.Datatypes.getTableName(table) |> SqlSyntax.Datatypes.QualifiedName.UnqualifiedName;
        let sqlStmt = SqlSyntax.Datatypes.Statement.Delete(SqlSyntax.Datatypes.DeleteStmt.DeleteStmt(tName, None));
        text("con.execute") ++ parens(dquotes(SqlSyntax.Pretty.ppStatement(sqlStmt) ++ semi()))

    // Using datatype constructors is too difficult, maybe smart constructors?

    // pub def merge(table: Table): Doc =
    //     let tName = RamOnSql.Datatypes.getTableName(table) |> SqlSyntax.Datatypes.QualifiedName.UnqualifiedName;
    //     let cols = RamOnSql.Datatypes.getColumnNames(table);
    //     let selectStmt = SqlSyntax.Datatypes.SelectStmt.SelectStmt();
    //     let selectNode = SqlSyntax.Datatypes.InsertValuesClause.Select(selectStmt);
    //     let sqlStmt = SqlSyntax.Datatypes.Statement.Insert(SqlSyntax.Datatypes.InsertStmt.InsertStmt(None, tName, cols, None, selectNode, None));
    //     text("con.execute") ++ parens(dquotes(SqlSyntax.Pretty.ppStatement(sqlStmt) ++ semi()))


}
