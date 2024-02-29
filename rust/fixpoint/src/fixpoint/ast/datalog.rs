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
use crate::fixpoint::ast::shared::{Value, Denotation, PredSym};
use crate::fixpoint::ast::ram; 
use crate::fixpoint::ast::ram::RamSym;
use crate::fixpoint::pred_syms_of::PredSymsOf;
use crate::fixpoint::substitute_pred_sym::SubstitutePredSym;

// Datalog
pub enum Datalog {
    Datalog(Vec<Constraint>, Vec<Constraint>),
    Model(HashMap<RamSym, HashMap<Vec<Value>, Value>>),
    Join(Box<Datalog>, Box<Datalog>),
}


impl fmt::Display for Datalog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Datalog::Datalog(facts, rules) => {
                let fs: Vec<_> = facts.iter().map(|v| format!("{v}")).collect();
                let rs: Vec<_> = rules.iter().map(|v| format!("{v}")).collect();
                write!(f, "{}\n{}", fs.join("\n"), rs.join("\n"))
            },
            Datalog::Model(db) => {
                let ss: Vec<String> = db.iter()
                    .map(|(ram_sym, rel)| match ram::into_denotation(ram_sym) {
                        Denotation::Relational => {
                            let ts: Vec<_> = rel.iter().map(|(tuple, _)| {
                                let tuple_string = tuple.iter().map(|v| format!("{v}")).collect::<Vec<_>>().join(", ");
                                format!("{}({})", ram_sym, tuple_string)
                                }).collect();
                            ts.join("\n")
                        },
                        Denotation::Latticenal(..) => {
                            let ts: Vec<_> = rel.iter().map(|(tuple, lat)| {
                                let tuple_string = tuple.iter().map(|v| format!("{v}")).collect::<Vec<_>>().join(", ");
                                format!("{}({}; {})", "{ram_sym}", tuple_string, lat)
                                }).collect();
                            ts.join("\n")
                        },
                    })
                    .collect();
                write!(f, "{}", ss.join("\n"))
            },
            Datalog::Join(d1, d2) => {
                let s1 = format!("{d1}");
                let s2 = format!("{d2}");
                write!(f, "{}\n{}", s1, s2)
            },
        }
    }
}

// Constraint (facts x rules)
pub enum Constraint {
    Constraint(HeadPredicate, Vec<BodyPredicate>),
}


impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Constraint::Constraint(head, body) => {
                if body.len() == 0 {
                    write!(f, "{}.", head)
                } else {
                    let bs: Vec<_> = body.iter().map(|v| format!("{v}")).collect();
                    write!(f, "{} :- {}.", head, bs.join(", "))
                }
            }
       }
    }
}

// HeadPredicate
pub enum HeadPredicate {
    HeadAtom(PredSym, Denotation, Vec<HeadTerm>),
}

impl fmt::Display for HeadPredicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HeadPredicate::HeadAtom(pred_sym, Denotation::Relational, terms) => {
                let ts: Vec<_> = terms.iter().map(|v| format!("{v}")).collect();
                write!(f, "{}({})", pred_sym, ts.join(", "))
            },
            HeadPredicate::HeadAtom(pred_sym, Denotation::Latticenal(..), terms) => {
                let n = terms.len() - 1;
                match terms.get(n) { 
                    None => write!(f, "{}()", pred_sym),
                    Some(l) => {
                        let key_terms = &terms[..n];
                        let kts: Vec<_> = terms.iter().map(|v| format!("{v}")).collect();
                        write!(f, "{}({}; {})", pred_sym, kts.join(", "), l)
                    },
                }
            },
        }
    }
}

// BodyPredicate
pub enum BodyPredicate {
    BodyAtom(PredSym, Denotation, Polarity, Fixity, Vec<BodyTerm>),
    Functional(Vec<VarSym>, fn(Vec<Value>) -> Vec<Vec<Value>>, Vec<VarSym>),
    Guard0(fn() -> bool),
    Guard1(fn(Value) -> bool, VarSym),
    Guard2(fn(Value, Value) -> bool, VarSym, VarSym),
    Guard3(fn(Value, Value, Value) -> bool, VarSym, VarSym, VarSym),
    Guard4(fn(Value, Value, Value, Value) -> bool, VarSym, VarSym, VarSym, VarSym),
    Guard5(fn(Value, Value, Value, Value, Value) -> bool, VarSym, VarSym, VarSym, VarSym, VarSym),
}

impl PredSymsOf for BodyPredicate {
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

impl SubstitutePredSym for BodyPredicate {
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

impl fmt::Display for BodyPredicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BodyPredicate::BodyAtom(pred_sym, Denotation::Relational, pty, fty, terms) => {
                let ts: Vec<_> = terms.iter().map(|v| format!("{v}")).collect();
                write!(f, "{}{}{}({})", polarity_prefix(pty), fixity_prefix(fty), pred_sym, ts.join(", "))
            },
            BodyPredicate::BodyAtom(pred_sym, Denotation::Latticenal(..), pty, fty, terms) => {
                let n = terms.len() - 1;
                match terms.get(n) { 
                    None => write!(f, "{}{}{}()", polarity_prefix(pty), fixity_prefix(fty), pred_sym),
                    Some(l) => {
                        let key_terms = &terms[..n];
                        let kts: Vec<_> = terms.iter().map(|v| format!("{v}")).collect();
                        write!(f, "{}{}{}({}; {})", polarity_prefix(pty), fixity_prefix(fty), pred_sym, kts.join(", "), l)
                    },
                }
            },
            BodyPredicate::Functional(bound_vars, _, free_vars) => { 
                let bvs: Vec<_> = bound_vars.iter().map(|v| format!("{v}")).collect();
                let fvs: Vec<_> = free_vars.iter().map(|v| format!("{v}")).collect();
                write!(f, "<loop>({}, {})", bvs.join(" "), fvs.join(" "))
            },
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

fn polarity_prefix(pl: &Polarity) -> String {
    match pl {
        Polarity::Negative => "not ".to_string(),
        Polarity::Positive => "".to_string(),
    }
}

fn fixity_prefix(fx: &Fixity) -> String {
    match fx {
        Fixity::Fixed => "fix ".to_string(),
        Fixity::Loose => "".to_string(),
    }
}

// HeadTerm
pub enum HeadTerm {
    Var(VarSym),
    Lit(Value),
    App0(fn() -> Value),
    App1(fn(Value) -> Value, VarSym),
    App2(fn(Value, Value) -> Value, VarSym, VarSym),
    App3(fn(Value, Value, Value) -> Value, VarSym, VarSym, VarSym),
    App4(fn(Value, Value, Value, Value) -> Value, VarSym, VarSym, VarSym, VarSym),
    App5(fn(Value, Value, Value, Value, Value) -> Value, VarSym, VarSym, VarSym, VarSym, VarSym),
}

impl fmt::Display for HeadTerm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HeadTerm::Var(var_sym) => write!(f, "{}", var_sym),
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
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum BodyTerm {
    Wild,
    Var(VarSym),
    Lit(Value),
}

impl fmt::Display for BodyTerm {
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
