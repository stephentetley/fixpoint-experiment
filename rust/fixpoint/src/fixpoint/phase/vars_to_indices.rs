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


use std::collections::HashMap;
use crate::fixpoint::ast::ram::{RamStmt, RamTerm, RowVar};

pub fn lower_stmt<V>(stmt: RamStmt<V>) -> RamStmt<V> {
    match stmt {
        RamStmt::Insert(_op) => todo!(), // RamStmt.Insert(lowerOp(op, Map#{}, 0))
        RamStmt::Merge(_, _) => stmt,
        RamStmt::Assign(_, _) => stmt,
        RamStmt::Purge(_) => stmt,
        RamStmt::Seq(_xs) => todo!(), // RamStmt::Seq(xs.iter().map(|x| lower_stmt(*x)).collect::<Vec<RamStmt<V>>>()),
        RamStmt::Until(test, body) => RamStmt::Until(test, Box::new(lower_stmt(*body))),
        RamStmt::Comment(_) => stmt,
    }
}

fn _lower_term<V>(_row_vars: HashMap<RowVar, RowVar>, term: RamTerm<V>) -> RamTerm<V> {
    match term {
        RamTerm::Lit(_) => term,
        RamTerm::RowLoad(_row_var, _index) => todo!(), 
            // match row_vars.get(&row_var) {
            //     Some(&row_var1) => RamTerm::RowLoad(row_var1, index),
            //     None => RamTerm::RowLoad(row_var, index), 
            // },
        RamTerm::LoadLatVar(_row_var) => todo!(),
    //      RamTerm.LoadLatVar(Map.getWithDefault(rowVar, rowVar, rowVars))
        RamTerm::Meet(_f, _v1, _v2) => todo!(), // {
        //     let t1 = lower_term(row_vars, *v1);
        //     let t2 = lower_term(row_vars, *v2);
        //     RamTerm::Meet(f, Box::new(t1), Box::new(t2))
        // },
        RamTerm::App0(_) => term,
        _ => todo!(),
        // RamTerm::App1(f, v) => todo!(),
    //     let t = lowerTerm(rowVars, v);
    //     RamTerm.App1(f, t)
        // RamTerm::App2(f, v1, v2) => todo!(),
    //     let t1 = lowerTerm(rowVars, v1);
    //     let t2 = lowerTerm(rowVars, v2);
    //     RamTerm.App2(f, t1, t2)
        // RamTerm::App3(f, v1, v2, v3) => todo!(),
    //     let t1 = lowerTerm(rowVars, v1);
    //     let t2 = lowerTerm(rowVars, v2);
    //     let t3 = lowerTerm(rowVars, v3);
    //     RamTerm.App3(f, t1, t2, t3)
        // RamTerm::App4(f, v1, v2, v3, v4) => todo!(),
    //     let t1 = lowerTerm(rowVars, v1);
    //     let t2 = lowerTerm(rowVars, v2);
    //     let t3 = lowerTerm(rowVars, v3);
    //     let t4 = lowerTerm(rowVars, v4);
    //     RamTerm.App4(f, t1, t2, t3, t4)
        // RamTerm::App5(f, v1, v2, v3, v4, v5) => todo!(),
    //     let t1 = lowerTerm(rowVars, v1);
    //     let t2 = lowerTerm(rowVars, v2);
    //     let t3 = lowerTerm(rowVars, v3);
    //     let t4 = lowerTerm(rowVars, v4);
    //     let t5 = lowerTerm(rowVars, v5);
    //     RamTerm.App5(f, t1, t2, t3, t4, t5)
    }
}
