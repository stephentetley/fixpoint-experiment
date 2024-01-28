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
use crate::fixpoint::ast::ram::{RelOp, BoolExp, RamStmt, RamTerm, RowVar};

pub fn lower_stmt<V>(stmt: RamStmt<V>) -> RamStmt<V> {
    match stmt {
        RamStmt::Insert(op) => {
            let op1 = lower_op(&HashMap::new(), 0, op);
            RamStmt::Insert(op1)
        },
        RamStmt::Merge(_, _) => stmt,
        RamStmt::Assign(_, _) => stmt,
        RamStmt::Purge(_) => stmt,
        RamStmt::Seq(xs) => {
            let xs1 = xs
                .into_iter()
                .map(lower_stmt)
                .collect::<Vec<RamStmt<V>>>();
            RamStmt::Seq(xs1)
        },
        RamStmt::Until(test, body) => {
            let body1 = lower_stmt(*body);
            RamStmt::Until(test, Box::new(body1))
        },
        RamStmt::Comment(_) => stmt,
    }
}

// TODO / WARNING this looks like it needs a persistent Map rather than HashMap...
fn lower_op<V>(row_vars: &HashMap<RowVar, RowVar>, depth: i32, op: RelOp<V>) -> RelOp<V> {
    match op {
        RelOp::Search(_var, ram_sym, body) => {
            let new_var = RowVar::Index(depth);
            let new_vars = row_vars; // Map.insert(var, new_var, row_vars);
            let op1 = lower_op(new_vars, depth + 1, *body);
            RelOp::Search(new_var, ram_sym, Box::new(op1))
        },
        RelOp::Query(_var, ram_sym, prefix_query, body) => {
            let new_var = RowVar::Index(depth);
            let new_vars = row_vars; // Map.insert(var, new_var, row_vars);
            let new_query = prefix_query
                .into_iter()
                .map(|(j, t)| (j, lower_term(row_vars, t)))
                .collect::<Vec<(i32, RamTerm<V>)>>();
            let body1 = lower_op(new_vars, depth + 1, *body);
            RelOp::Query(new_var, ram_sym, new_query, Box::new(body1))
        },
        RelOp::Functional(_row_var, f, terms, body) => {
            let new_var = RowVar::Index(depth);
            let new_vars = row_vars; // Map.insert(rowVar, newVar, row_vars);
            let new_terms = terms
                .into_iter()
                .map(|t| lower_term(row_vars, t))
                .collect::<Vec<RamTerm<V>>>();
            let new_body = lower_op(new_vars, depth + 1, *body);
            RelOp::Functional(new_var, f, new_terms, Box::new(new_body))
        },
        RelOp::Project(terms, ram_sym) => {
            let terms1 = terms
                .into_iter()
                .map(|t| lower_term(row_vars, t))
                .collect::<Vec<RamTerm<V>>>();
            RelOp::Project(terms1, ram_sym)
        },
        RelOp::If(test, then) => {
            let test1 = test
                .into_iter()
                .map(|e| lower_exp(row_vars, e))
                .collect::<Vec<BoolExp<V>>>();
            let then1 = lower_op(row_vars, depth, *then);
            RelOp::If(test1, Box::new(then1))
        },
    }
}

fn lower_exp<V>(row_vars: &HashMap<RowVar, RowVar>, exp: BoolExp<V>) -> BoolExp<V> {
    match exp {
        BoolExp::Empty(_) => exp,
        BoolExp::NotMemberOf(terms, ram_sym) => {
            let terms1 = terms
                .into_iter()
                .map(|t| lower_term(row_vars, t))
                .collect::<Vec<RamTerm<V>>>();
            BoolExp::NotMemberOf(terms1, ram_sym)
        },
        BoolExp::Eq(lhs, rhs) => {
            let lhs1 = lower_term(row_vars, lhs);
            let rhs1 = lower_term(row_vars, rhs);
            BoolExp::Eq(lhs1, rhs1)
        },
        BoolExp::Leq(f, lhs, rhs) => {
            let lhs1 = lower_term(row_vars, lhs);
            let rhs1 = lower_term(row_vars, rhs);
            BoolExp::Leq(f, lhs1, rhs1)
        },
        BoolExp::Guard0(_) => exp,
        BoolExp::Guard1(f, v) => {
            let t = lower_term(row_vars, v);
            BoolExp::Guard1(f, t)
        },
        BoolExp::Guard2(f, v1, v2) => {
            let t1 = lower_term(row_vars, v1);
            let t2 = lower_term(row_vars, v2);
            BoolExp::Guard2(f, t1, t2)
        },
        BoolExp::Guard3(f, v1, v2, v3) => {
            let t1 = lower_term(row_vars, v1);
            let t2 = lower_term(row_vars, v2);
            let t3 = lower_term(row_vars, v3);
            BoolExp::Guard3(f, t1, t2, t3)
        },
        BoolExp::Guard4(f, v1, v2, v3, v4) => {
            let t1 = lower_term(row_vars, v1);
            let t2 = lower_term(row_vars, v2);
            let t3 = lower_term(row_vars, v3);
            let t4 = lower_term(row_vars, v4);
            BoolExp::Guard4(f, t1, t2, t3, t4)
        },
        BoolExp::Guard5(f, v1, v2, v3, v4, v5) => {
            let t1 = lower_term(row_vars, v1);
            let t2 = lower_term(row_vars, v2);
            let t3 = lower_term(row_vars, v3);
            let t4 = lower_term(row_vars, v4);
            let t5 = lower_term(row_vars, v5);
            BoolExp::Guard5(f, t1, t2, t3, t4, t5)
        },
    }
}

fn lower_term<V>(row_vars: &HashMap<RowVar, RowVar>, term: RamTerm<V>) -> RamTerm<V> {
    match term {
        RamTerm::Lit(_) => term,
        RamTerm::RowLoad(row_var, index) => 
            match row_vars.get(&row_var) {
                Some(row_var1) => RamTerm::RowLoad(row_var1.clone(), index),
                None => RamTerm::RowLoad(row_var, index), 
            },
        RamTerm::LoadLatVar(row_var) => 
            match row_vars.get(&row_var) {
                Some(row_var1) => RamTerm::LoadLatVar(row_var1.clone()),
                None => RamTerm::LoadLatVar(row_var), 
            },
        RamTerm::Meet(f, v1, v2) => {
            let t1 = lower_term(row_vars, *v1);
            let t2 = lower_term(row_vars, *v2);
            RamTerm::Meet(f, Box::new(t1), Box::new(t2))
        },
        RamTerm::App0(_) => term,
        RamTerm::App1(f, v) => {
            let t = lower_term(row_vars, *v);
            RamTerm::App1(f, Box::new(t))
        },
        RamTerm::App2(f, v1, v2) => {
            let t1 = lower_term(row_vars, *v1);
            let t2 = lower_term(row_vars, *v2);
            RamTerm::App2(f, Box::new(t1), Box::new(t2))
        },
        RamTerm::App3(f, v1, v2, v3) => {
            let t1 = lower_term(row_vars, *v1);
            let t2 = lower_term(row_vars, *v2);
            let t3 = lower_term(row_vars, *v3);
            RamTerm::App3(f, Box::new(t1), Box::new(t2), Box::new(t3))
        },
        RamTerm::App4(f, v1, v2, v3, v4) => {
            let t1 = lower_term(row_vars, *v1);
            let t2 = lower_term(row_vars, *v2);
            let t3 = lower_term(row_vars, *v3);
            let t4 = lower_term(row_vars, *v4);
            RamTerm::App4(f, Box::new(t1), Box::new(t2), Box::new(t3), Box::new(t4))
        },
        RamTerm::App5(f, v1, v2, v3, v4, v5) => {
            let t1 = lower_term(row_vars, *v1);
            let t2 = lower_term(row_vars, *v2);
            let t3 = lower_term(row_vars, *v3);
            let t4 = lower_term(row_vars, *v4);
            let t5 = lower_term(row_vars, *v5);
            RamTerm::App5(f, Box::new(t1), Box::new(t2), Box::new(t3), Box::new(t4), Box::new(t5))
        },
    }
}
