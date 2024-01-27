//
// Copyright 2021 Benjamin Dahse
// Copyright 2024 Stephen Tetley
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.



use crate::fixpoint::ast::ram::{RamStmt};

pub fn lower_stmt<V>(stmt: RamStmt<V>) -> RamStmt<V> {
    match stmt {
        RamStmt::Insert(_op) => todo!(), // RamStmt.Insert(lowerOp(op, Map#{}, 0))
        RamStmt::Merge(_, _) => stmt,
        RamStmt::Assign(_, _) => stmt,
        RamStmt::Purge(_) => stmt,
        RamStmt::Seq(_xs) => todo!(), // RamStmt.Seq(Vector.map(lowerStmt, xs))
        RamStmt::Until(test, body) => RamStmt::Until(test, Box::new(lower_stmt(*body))),
        RamStmt::Comment(_) => stmt,
    }
}