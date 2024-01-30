//
// Copyright 2021 Benjamin Dahse
// Copyright 2022 Jonathan Lindegaard Starup
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
use std::collections::HashMap;
use crate::fixpoint::ast::shared::{Denotation, PredSym};
use crate::fixpoint::ast::ram::{RamSym};

// Datalog
pub enum Datalog<V> {
    Datalog(Vec<Constraint<V>>, Vec<Constraint<V>>),
    Model(HashMap<RamSym<V>, HashMap<Vec<V>, V>>),
    Join(Box<Datalog<V>>, Box<Datalog<V>>),
}

// Constraint
pub enum Constraint<V> {
    Constraint(HeadPredicate<V>, Vec<BodyPredicate<V>>),
}

// HeadPredicate
pub enum HeadPredicate<V> {
    HeadAtom(PredSym, Denotation<V>, Vec<HeadTerm<V>>),
}

// BodyPredicate
pub enum BodyPredicate<V> {
    BodyAtom(PredSym, Denotation<V>, Polarity, Fixity, Vec<BodyTerm<V>>),
    Functional(Vec<VarSym>, fn(Vec<V>) -> Vec<Vec<V>>, Vec<VarSym>),
    Guard0(fn() -> bool),
    Guard1(fn(V) -> bool, VarSym),
    Guard2(fn(V, V) -> bool, VarSym, VarSym),
    Guard3(fn(V, V, V) -> bool, VarSym, VarSym, VarSym),
    Guard4(fn(V, V, V, V) -> bool, VarSym, VarSym, VarSym, VarSym),
    Guard5(fn(V, V, V, V, V) -> bool, VarSym, VarSym, VarSym, VarSym, VarSym),
}


// HeadTerm
pub enum HeadTerm<V> {
    Var(VarSym),
    Lit(V),
    App0(fn() -> V),
    App1(fn(V) -> V, VarSym),
    App2(fn(V, V) -> V, VarSym, VarSym),
    App3(fn(V, V, V) -> V, VarSym, VarSym, VarSym),
    App4(fn(V, V, V, V) -> V, VarSym, VarSym, VarSym, VarSym),
    App5(fn(V, V, V, V, V) -> V, VarSym, VarSym, VarSym, VarSym, VarSym),
}

impl<V: std::fmt::Display> fmt::Display for HeadTerm<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HeadTerm::Var(var_sym) => write!(f, "${}", var_sym),
            HeadTerm::Lit(v) => write!(f, "%{}", v),
            HeadTerm::App0(_) => write!(f, "<clo>()"),
            HeadTerm::App1(_, v) => write!(f, "<clo>({})", v),
            HeadTerm::App2(_, v1, v2) => write!(f, "<clo>({}, {})", v1, v2),
            HeadTerm::App3(_, v1, v2, v3) => write!(f, "<clo>({}, {}, {})", v1, v2, v3),
            HeadTerm::App4(_, v1, v2, v3, v4) => write!(f, "<clo>({}, {}, {}, {})", v1, v2, v3, v4),
            HeadTerm::App5(_, v1, v2, v3, v4, v5) => write!(f, "<clo>({}, {}, {}, {}, {})", v1, v2, v3, v4, v5),
        }
    }
}

// BodyTerm
pub enum BodyTerm<V> {
    Wild,
    Var(VarSym),
    Lit(V),
}

impl<V: std::fmt::Display> fmt::Display for BodyTerm<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BodyTerm::Wild => write!(f, "_"),
            BodyTerm::Var(var_sym) => write!(f, "{}", var_sym),
            BodyTerm::Lit(v) => write!(f, "{}", v),
        }
    }
}

// VarSym
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum VarSym {
    VarSym(String),
}


impl fmt::Display for VarSym {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VarSym::VarSym(s) => write!(f, "{}", s),
        }
    }
}

// Fixity
pub enum Fixity {
    Loose,
    Fixed,
}

pub enum Polarity {
    Positive,
    Negative,
}