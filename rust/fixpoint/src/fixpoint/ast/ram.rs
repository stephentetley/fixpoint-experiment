/*
 * Copyright 2021 Benjamin Dahse
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

use std::fmt;
use std::cmp::Ordering;
use crate::fixpoint::ast::shared::{Denotation, PredSym};

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

impl<V: std::fmt::Display> fmt::Display for RamTerm<V> {
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


pub enum RamSym<V> {
    Full(PredSym, i32, Denotation<V>),
    Delta(PredSym, i32, Denotation<V>),
    New(PredSym, i32, Denotation<V>),
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

#[derive(PartialEq, Eq)]
pub enum RowVar {
    Named(String),
    Index(i32),
}

// Implement `Display` for `RowVar`.
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
