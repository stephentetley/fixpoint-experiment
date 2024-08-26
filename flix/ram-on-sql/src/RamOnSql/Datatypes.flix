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

mod RamOnSql.Datatypes {

    use Text.Pretty
    use Text.Pretty.{pPrint}
    use Text.PrettyPrint.Doc
    use Text.PrettyPrint.{<<>>, text, parens, intersperse, comma}

    use RamOnSql.Datatypes.SqlType.{Varchar, Integer}

    // Columns and primary keys
    pub enum Table with Eq, ToString {
        case Table(String, List[(String, SqlType)], List[String])
    }

    pub enum SqlType with Eq, ToString {
        case Varchar
        case Integer
        case Date
    }


    pub def getTableName(t: Table): String = match t {
        case Table.Table(name, _, _) => name
    }
    
    pub def getColumnNames(t: Table): List[String] = match t {
        case Table.Table(_, cols, _) => List.map(fst, cols)
    }


    pub def ppCreateTable(x: Table): Doc = match x {
        case Table.Table(name, cols, pks) => 
            let colsD = intersperse(text(", "), List.map(match (n,ty) -> text(n) <<>> pPrint(ty), cols));
            let pkD = text("PRIMARY KEY") + parens(intersperse(text(", "), List.map(text, pks)));
            text("CREATE OR REPLACE TABLE") <<>> text(name) <<>> parens(colsD + comma() <<>> pkD) 
    }

    instance Pretty[SqlType] {
        pub override def pPrint(x: SqlType): Doc = match x {
            case SqlType.Date => text("Date")
            case SqlType.Integer => text("INTEGER")
            case SqlType.Varchar => text("VARCHAR")
        }
    }

}
