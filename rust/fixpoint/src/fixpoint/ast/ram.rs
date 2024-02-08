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


use std::fmt;
use std::cmp::Ordering;
use crate::fixpoint::ast::shared::{Denotation, PredSym};

// RamStmt
pub enum RamStmt<V> {
    Insert(RelOp<V>),
    Merge(Box<RamSym<V>>, Box<RamSym<V>>),
    Assign(Box<RamSym<V>>, Box<RamSym<V>>),
    Purge(Box<RamSym<V>>),
    Seq(Vec<RamStmt<V>>),
    Until(Vec<BoolExp<V>>, Box<RamStmt<V>>),
    Comment(String),
}

impl<V: fmt::Display> fmt::Display for RamStmt<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RamStmt::Insert(op) => write!(f, "{}", op),
            RamStmt::Merge(src, dst) => write!(f, "merge {} into {}", src, dst),
            RamStmt::Assign(lhs, rhs) => write!(f, "{} := ${}", lhs, rhs),
            RamStmt::Purge(ram_sym) => write!(f, "purge {}", ram_sym),
            RamStmt::Seq(xs) => {
                let body = xs
                    .into_iter()
                    .map(|r| r.to_string())
                    .collect::<Vec<String>>()
                    .join(";\n");
                write!(f, "{}", body)
            },
            RamStmt::Until(test, body) => {
                let test1 = test
                    .into_iter()
                    .map(|r| r.to_string())
                    .collect::<Vec<String>>()
                    .join(" Λ ");
                write!(f, "until({}) do\n{}end", test1, *body)
            }
            RamStmt::Comment(comment) => write!(f, "// {}", comment),
        }
    }
}

// RelOp
pub enum RelOp<V> {
    Search(RowVar, RamSym<V>, Box<RelOp<V>>),
    Query(RowVar, RamSym<V>, Vec<(i32, RamTerm<V>)>, Box<RelOp<V>>),
    Functional(RowVar, fn(Vec<V>) -> Vec<Vec<V>>, Vec<RamTerm<V>>, Box<RelOp<V>>),
    Project(Vec<RamTerm<V>>, RamSym<V>),
    If(Vec<BoolExp<V>>, Box<RelOp<V>>),
}

impl<V: fmt::Display> fmt::Display for RelOp<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RelOp::Search(var, ram_sym, body) => 
                write!(f, "search {} ∈ {} do\n{}end", var, ram_sym, body),
            RelOp::Query(_, _, _, _) => todo!(),
            // RelOp::Query(var, ram_sym, prefix_query, body) => {
            //     let query = prefix_query
            //         .into_iter()
            //         .map(|&(i, term)| BoolExp::Eq(RamTerm::RowLoad(var, i), term).to_string())
            //         .collect::<Vec<String>>()
            //         .join(" ∧ ");
            //     write!(f, "query {{{} ∈ {} | {}}} do\n{}end", var, ram_sym, query, body)
            // },
            RelOp::Functional(row_var, _, terms, body) => {
                let terms1 = terms
                    .into_iter()
                    .map(|r| r.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "loop({} <- f({})) do\n{}end", row_var, terms1, body)
            },
            RelOp::Project(terms, ram_sym) => {
                let terms1 = terms
                    .into_iter()
                    .map(|r| r.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "project ({}) into {}", terms1, ram_sym)
            },
            RelOp::If(test, then) => {
                let test1 = test
                    .into_iter()
                    .map(|r| r.to_string())
                    .collect::<Vec<String>>()
                    .join(" ∧ ");
                write!(f, "if({}) then\n{}end", test1, then)
            },
        }
    }
}

// BoolExp
pub enum BoolExp<V> {
    Empty(RamSym<V>),
    NotMemberOf(Vec<RamTerm<V>>, RamSym<V>),
    Eq(RamTerm<V>, RamTerm<V>),
    Leq(fn(V, V) -> bool, RamTerm<V>, RamTerm<V>),
    Guard0(fn() -> bool),
    Guard1(fn(V) -> bool, RamTerm<V>),
    Guard2(fn(V, V) -> bool, RamTerm<V>, RamTerm<V>),
    Guard3(fn(V, V, V) -> bool, RamTerm<V>, RamTerm<V>, RamTerm<V>),
    Guard4(fn(V, V, V, V) -> bool, RamTerm<V>, RamTerm<V>, RamTerm<V>, RamTerm<V>),
    Guard5(fn(V, V, V, V, V) -> bool, RamTerm<V>, RamTerm<V>, RamTerm<V>, RamTerm<V>, RamTerm<V>),
}

impl<V: fmt::Display> fmt::Display for BoolExp<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BoolExp::Empty(ram_sym) => write!(f, "{} == ∅", ram_sym),
            BoolExp::NotMemberOf(terms, ram_sym) => {
                let terms1 = terms  
                    .into_iter()
                    .map(|r| r.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "({}) ∉ {}", terms1, ram_sym)
            },
            BoolExp::Eq(lhs, rhs) => write!(f, "{} == {}", lhs, rhs),
            BoolExp::Leq(_, lhs, rhs) => write!(f, "{} ≤ {}", lhs, rhs),
            BoolExp::Guard0(_) => write!(f, "<clo>()"),
            BoolExp::Guard1(_, v) => write!(f, "<clo>({})", v),
            BoolExp::Guard2(_, v1, v2) => write!(f, "<clo>({}, {})", v1, v2),
            BoolExp::Guard3(_, v1, v2, v3) => write!(f, "<clo>({}, {}, {})", v1, v2, v3),
            BoolExp::Guard4(_, v1, v2, v3, v4) => write!(f, "<clo>({}, {}, {}, {})", v1, v2, v3, v4),
            BoolExp::Guard5(_, v1, v2, v3, v4, v5) => write!(f, "<clo>({}, {}, {}, {}, {})", v1, v2, v3, v4, v5),
        }
    }
}

// RamTerm
pub enum RamTerm<V> {
    Lit(V),
    RowLoad(RowVar, i32),
    LoadLatVar(RowVar),
    Meet(fn(V, V) -> V, Box<RamTerm<V>>, Box<RamTerm<V>>),
    App0(fn() -> V),
    App1(fn(V) -> V, Box<RamTerm<V>>),
    App2(fn(V, V) -> V, Box<RamTerm<V>>, Box<RamTerm<V>>),
    App3(fn(V, V, V) -> V, Box<RamTerm<V>>, Box<RamTerm<V>>, Box<RamTerm<V>>),
    App4(fn(V, V, V, V) -> V, Box<RamTerm<V>>, Box<RamTerm<V>>, Box<RamTerm<V>>, Box<RamTerm<V>>),
    App5(fn(V, V, V, V, V) -> V, Box<RamTerm<V>>, Box<RamTerm<V>>, Box<RamTerm<V>>, Box<RamTerm<V>>, Box<RamTerm<V>>),
}

impl<V: fmt::Display> fmt::Display for RamTerm<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RamTerm::Lit(v) => write!(f, "{}", v),
            RamTerm::RowLoad(var, index) => write!(f, "{}[{}]", var, index),
            RamTerm::LoadLatVar(var) => write!(f, "{}[-1]", var),
            RamTerm::Meet(_, lhs, rhs) => write!(f, "({} ⊓ {})", lhs, rhs),
            RamTerm::App0(_) => write!(f, "<clo>()"),
            RamTerm::App1(_, v) => write!(f, "<clo>({})", v),
            RamTerm::App2(_, v1, v2) => write!(f, "<clo>({}, {})", v1, v2),
            RamTerm::App3(_, v1, v2, v3) => write!(f, "<clo>({}, {}, {})", v1, v2, v3),
            RamTerm::App4(_, v1, v2, v3, v4) => write!(f, "<clo>({}, {}, {}, {})", v1, v2, v3, v4),
            RamTerm::App5(_, v1, v2, v3, v4, v5) => write!(f, "<clo>({}, {}, {}, {}, {})", v1, v2, v3, v4, v5),
        }
    }
}

// RamSym
pub enum RamSym<V> {
    Full(PredSym, i32, Denotation<V>),
    Delta(PredSym, i32, Denotation<V>),
    New(PredSym, i32, Denotation<V>),
}


impl<V: fmt::Display> fmt::Display for RamSym<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RamSym::Full(sym, _, _)    => write!(f, "{}", sym),
            RamSym::Delta(sym, _, _)   => write!(f, "Δ{}", sym),
            RamSym::New(sym, _, _)     => write!(f, "Δ{}'", sym),
        }
    }
}

impl<V> PartialEq for RamSym<V> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RamSym::Full(s1, _, _),  RamSym::Full(s2, _, _))  => s1 == s2,
            (RamSym::Delta(s1, _, _), RamSym::Delta(s2, _, _)) => s1 == s2,
            (RamSym::New(s1, _, _),   RamSym::New(s2, _, _))   => s1 == s2,
            (_, _) => false,
        }
    }
}

pub fn arity_of<V>(ram_sym: RamSym<V>) -> i32 {
    match ram_sym {
        RamSym::Full(_, arity, _) => arity,
        RamSym::Delta(_, arity, _) => arity,
        RamSym::New(_, arity, _) => arity,
    }
}

pub fn into_denotation<V>(ram_sym: RamSym<V>) -> Denotation<V> {
    match ram_sym {
        RamSym::Full(_, _, den) => den,
        RamSym::Delta(_, _, den) => den,
        RamSym::New(_, _, den) => den,
    }
}

// RowVar
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum RowVar {
    Named(String),
    Index(i32),
}

impl fmt::Display for RowVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        match self {
            RowVar::Named(s) => write!(f, "{}", s),
            RowVar::Index(i) => write!(f, "{}$", i),
        }
    }
}

impl PartialOrd for RowVar {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (RowVar::Named(s1), RowVar::Named(s2)) => s1.partial_cmp(s2),
            (RowVar::Index(i1), RowVar::Index(i2)) => i1.partial_cmp(i2),
            (_, _) => None,
        }
    }
}
