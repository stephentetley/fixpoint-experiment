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
use std::collections::{HashMap, HashSet};
use crate::fixpoint::ast::shared::{Denotation, PredSym};
use crate::fixpoint::ast::ram::RamSym;
use crate::fixpoint::pred_syms_of::PredSymsOf;
use crate::fixpoint::substitute_pred_sym::SubstitutePredSym;

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

impl<V> PredSymsOf for BodyPredicate<V> {
    fn pred_syms_of(&self) -> HashSet<PredSym> {
        match self {
            BodyPredicate::BodyAtom(pred_sym, _, _, _, _) => { 
                let mut syms = HashSet::<PredSym>::new();
                let sym1 = pred_sym.clone();
                syms.insert(sym1);
                syms
            }
            _ => HashSet::new(),
        }
    }
}

impl<V> SubstitutePredSym for BodyPredicate<V> {
    fn substitute_pred_sym(&self, s: HashMap<PredSym, PredSym>) -> &Self {
        match self {
            // BodyPredicate::BodyAtom(pred_sym, den, polarity, fixity, terms) => {
            //     let new_sym = match s.get(&pred_sym) {
            //         Some(sym1) => sym1.clone(),
            //         None => pred_sym.clone(), 
            //     };
            //     &BodyPredicate::BodyAtom(new_sym, den, polarity.clone(), fixity.clone(), terms)
            // },
            _ => self.clone(),
        }
    }
}

fn polarity_prefix(pl: Polarity, f: &mut fmt::Formatter) -> fmt::Result {
    match pl {
        Polarity::Negative => write!(f, "not "),
        Polarity::Positive => write!(f, ""),
    }
}

fn fixity_prefix(fx: Fixity, f: &mut fmt::Formatter) -> fmt::Result {
    match fx {
        Fixity::Fixed => write!(f, "fix "),
        Fixity::Loose => write!(f, ""),
    }
}

impl<V: std::fmt::Display> fmt::Display for BodyPredicate<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // BodyPredicate::BodyAtom(predSym, Denotation.Relational, p, f, terms) =>
            //     "${polarityPrefix(p)}${fixityPrefix(f)}${predSym}(${terms |> Vector.join(", ")})"
            // BodyPredicate::BodyAtom(predSym, Denotation.Latticenal(_), p, f, terms) =>
            //     let n = Vector.length(terms)-1;
            //     let (keyTerms, latticeTerms) = (Vector.take(n, terms), Vector.drop(n, terms));
            //     match Vector.head(latticeTerms) {
            //         case None    => "${polarityPrefix(p)}${fixityPrefix(f)}${predSym}()"
            //         case Some(l) => "${polarityPrefix(p)}${fixityPrefix(f)}${predSym}(${keyTerms |> Vector.join(", ")}; ${l})"
            //     }
            // BodyPredicate::Functional(bound_vars, _, free_vars) => write!(f, "<loop>({}, {})", bound_vars, free_vars),
            BodyPredicate::Guard0(_) => write!(f, "<clo>()"),
            BodyPredicate::Guard1(_, v) => write!(f, "<clo>({})", v),
            BodyPredicate::Guard2(_, v1, v2) => write!(f, "<clo>({}, {})", v1, v2),
            BodyPredicate::Guard3(_, v1, v2, v3) => write!(f, "<clo>({}, {}, {})", v1, v2, v3),
            BodyPredicate::Guard4(_, v1, v2, v3, v4) => write!(f, "<clo>({}, {}, {}, {})", v1, v2, v3, v4),
            BodyPredicate::Guard5(_, v1, v2, v3, v4, v5) => write!(f, "<clo>({}, {}, {}, {}, {})", v1, v2, v3, v4, v5), 
            _ => todo!(),
        }
    }
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
#[derive(PartialEq, PartialOrd, Eq, Hash, Clone, Debug)]
pub enum Fixity {
    Loose,
    Fixed,
}

// Polarity
#[derive(PartialEq, PartialOrd, Eq, Hash, Clone, Debug)]
pub enum Polarity {
    Positive,
    Negative,
}