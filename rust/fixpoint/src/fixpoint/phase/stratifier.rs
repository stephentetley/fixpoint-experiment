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
use crate::fixpoint::ast::datalog::{Datalog, Constraint, HeadPredicate, BodyPredicate, Fixity, Polarity};
use crate::fixpoint::ast::precedence_graph::{PrecedenceGraph, PrecedenceEdge};
use crate::fixpoint::ast::shared::PredSym;

//
// Compute a stratification with Ullman's algorithm.
// The Flix compiler is supposed to guarantee the existence of a stratification at this point.
// Initially, all IDB predicates are assigned to stratum 0.
// I.e. facts are ignored in the stratification.
//
pub fn stratify<V>(d: &Datalog<V>) -> HashMap<PredSym, i32> {
    match d {
        Datalog::Datalog(_, rules) => {
            let mut m1 = HashMap::new();
            rules.iter()
                .rev()
                .for_each(|Constraint::Constraint(HeadPredicate::HeadAtom(p, _, _), _)| { m1.insert(p.clone(), 0); ()});
            let pg = mk_dep_graph(d);
            stratify_helper(&pg, &mut m1);
            m1
        },
        Datalog::Model(_) => HashMap::new(), // Models contain only facts. 
        Datalog::Join(d1, d2) => { 
            let mut m1 = stratify(&d1);
            let mut m2 = stratify(&d2);
            m2.iter()
                .for_each(|(k, &v)| match m1.get(&k) {
                    Some(&v1) => { m1.insert(k.clone(), std::cmp::max(v, v1)); () }
                    None => { m1.insert(k.clone(), v); () }
                });
            m1
        },
    }
}


fn stratify_helper(g: &PrecedenceGraph, stf: &mut HashMap<PredSym, i32>) {
    let PrecedenceGraph(xs) = g;
    // The number of strata is bounded by the number of predicates
    // which is bounded by the number of edges in the precedence graph.
    let max_stratum = xs.len() as i32;
    // Visit every edge and ensure the following:
    // 1. If (body, head) is a weak edge, then body belongs to a lower stratum
    // or the same stratum as head.
    // 2. If (body, head) is a strong edge, then body belongs to a strictly lower stratum
    // than head.
    let changed = xs.iter().fold(false, |acc, edge| match edge {
        PrecedenceEdge::WeakEdge(body_sym, head_sym) => {
            let body_stratum = stf.get(body_sym).unwrap_or(&0);
            let head_stratum = stf.get(head_sym).unwrap_or(&0);
            if body_stratum > head_stratum {
                stf.insert(head_sym.clone(), *body_stratum);
                true
            } else {
                acc
            }
        },
        PrecedenceEdge::StrongEdge(body_sym, head_sym) => {
            let body_stratum = stf.get(body_sym).unwrap_or(&0);
            let head_stratum = stf.get(head_sym).unwrap_or(&0);
            if body_stratum > head_stratum {
                let new_head_stratum = body_stratum + 1;
                // If there are more strata than edges,
                // the precedence graph must contain a strong cycle!
                if new_head_stratum > max_stratum {
                    panic!("Stratification error (strong cycle)")
                } else {
                   stf.insert(head_sym.clone(), new_head_stratum);
                    true
                }
            } else {
                acc
            }
        },
    });
    // Check if property 1 and 2 now holds for all edges.
    if changed {
        stratify_helper(g, stf)
    } else {
        ()
    }
}

fn mk_dep_graph<V>(d: &Datalog<V>) -> PrecedenceGraph {
    match d {
        Datalog::Datalog(_, rules) => todo!(), // Vector.fold(Vector.map(precedenceHelper, rules))
        Datalog::Model(_) => PrecedenceGraph::new(),
        Datalog::Join(d1, d2) => {
            let mut pg1 = mk_dep_graph(d1);
            let pg2 = mk_dep_graph(d2);
            pg1.extend(pg2);
            pg1
        },
    }
}


fn precedence_helper<V>(cnst: Constraint<V>) -> PrecedenceGraph {
    match cnst {
        Constraint::Constraint(head, body) => {
            let pg = PrecedenceGraph::new();
            // body
            //     .iter()
            //     .for_each(|bodyp| {pg.extend(&mk_dep_edge(&head, bodyp)); ()});
            pg
        }
    }
}


//
// Creates a singleton set with a strong or weak edge. Negatively bound atoms and
// fixed atoms create strong edges because the body predicate has to be in a strictly
// lower strata than the head. Positive, loose atoms create weak edges where the body
// has to be in the same strata as the head or lower.
//
fn mk_dep_edge<V>(dst: &HeadPredicate<V>, src: &BodyPredicate<V>) -> PrecedenceGraph {
    match (dst, src) {
        // (HeadPredicate::HeadAtom(head_sym, _, _), BodyPredicate::BodyAtom(body_sym, _, Polarity::Positive, Fixity::Loose, _)) => {
        //     let mut pg = PrecedenceGraph::new();
        //     pg.insert(PrecedenceEdge::WeakEdge(body_sym, head_sym));
        //     pg
        // },
        // //    PrecedenceGraph(Set#{WeakEdge(bodySym, headSym)})
        // (HeadPredicate::HeadAtom(head_sym, _, _), BodyPredicate::BodyAtom(body_sym, _, _, _, _)) => {
        //     let mut pg = PrecedenceGraph::new();
        //     pg.insert(PrecedenceEdge::StrongEdge(body_sym, head_sym));
        //     pg
        // },
        _ => PrecedenceGraph::new(),
    }
}
