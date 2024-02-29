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
use crate::fixpoint::ast::shared::{Value, Denotation, PredSym};

// RamStmt
pub enum RamStmt {
    Insert(RelOp),
    Merge(Box<RamSym>, Box<RamSym>),
    Assign(Box<RamSym>, Box<RamSym>),
    Purge(Box<RamSym>),
    Seq(Vec<RamStmt>),
    Until(Vec<BoolExp>, Box<RamStmt>),
    Comment(String),
}

impl fmt::Display for RamStmt {
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
pub enum RelOp {
    Search(RowVar, RamSym, Box<RelOp>),
    Query(RowVar, RamSym, Vec<(i32, RamTerm)>, Box<RelOp>),
    Functional(RowVar, fn(Vec<Value>) -> Vec<Vec<Value>>, Vec<RamTerm>, Box<RelOp>),
    Project(Vec<RamTerm>, RamSym),
    If(Vec<BoolExp>, Box<RelOp>),
}

impl fmt::Display for RelOp {
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
pub enum BoolExp {
    Empty(RamSym),
    NotMemberOf(Vec<RamTerm>, RamSym),
    Eq(RamTerm, RamTerm),
    Leq(fn(Value, Value) -> bool, RamTerm, RamTerm),
    Guard0(fn() -> bool),
    Guard1(fn(Value) -> bool, RamTerm),
    Guard2(fn(Value, Value) -> bool, RamTerm, RamTerm),
    Guard3(fn(Value, Value, Value) -> bool, RamTerm, RamTerm, RamTerm),
    Guard4(fn(Value, Value, Value, Value) -> bool, RamTerm, RamTerm, RamTerm, RamTerm),
    Guard5(fn(Value, Value, Value, Value, Value) -> bool, RamTerm, RamTerm, RamTerm, RamTerm, RamTerm),
}

impl fmt::Display for BoolExp {
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
pub enum RamTerm {
    Lit(Value),
    RowLoad(RowVar, i32),
    LoadLatVar(RowVar),
    Meet(fn(Value, Value) -> Value, Box<RamTerm>, Box<RamTerm>),
    App0(fn() -> Value),
    App1(fn(Value) -> Value, Box<RamTerm>),
    App2(fn(Value, Value) -> Value, Box<RamTerm>, Box<RamTerm>),
    App3(fn(Value, Value, Value) -> Value, Box<RamTerm>, Box<RamTerm>, Box<RamTerm>),
    App4(fn(Value, Value, Value, Value) -> Value, Box<RamTerm>, Box<RamTerm>, Box<RamTerm>, Box<RamTerm>),
    App5(fn(Value, Value, Value, Value, Value) -> Value, Box<RamTerm>, Box<RamTerm>, Box<RamTerm>, Box<RamTerm>, Box<RamTerm>),
}

impl fmt::Display for RamTerm {
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

// RamSym - Denotation embeds functions / function pointers, PredSym is "scalar"
#[derive(Eq, Debug, Hash, Clone)]
pub enum RamSym {
    Full(PredSym, i32, Denotation),
    Delta(PredSym, i32, Denotation),
    New(PredSym, i32, Denotation),
}


impl fmt::Display for RamSym {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RamSym::Full(sym, _, _)    => write!(f, "{}", sym),
            RamSym::Delta(sym, _, _)   => write!(f, "Δ{}", sym),
            RamSym::New(sym, _, _)     => write!(f, "Δ{}'", sym),
        }
    }
}

impl PartialEq for RamSym {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RamSym::Full(s1, _, _),  RamSym::Full(s2, _, _))  => s1 == s2,
            (RamSym::Delta(s1, _, _), RamSym::Delta(s2, _, _)) => s1 == s2,
            (RamSym::New(s1, _, _),   RamSym::New(s2, _, _))   => s1 == s2,
            (_, _) => false,
        }
    }
}

pub fn arity_of(ram_sym: RamSym) -> i32 {
    match ram_sym {
        RamSym::Full(_, arity, _) => arity,
        RamSym::Delta(_, arity, _) => arity,
        RamSym::New(_, arity, _) => arity,
    }
}

pub fn into_denotation(ram_sym: &RamSym) -> &Denotation {
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
