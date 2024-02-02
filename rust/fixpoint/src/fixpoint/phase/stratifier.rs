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
pub fn stratify<V>(d: Datalog<V>) -> HashMap<PredSym, i32> {
    match d {
        // Datalog::Datalog(_, rules) =>
        //     Vector.foldRight(match Constraint(HeadAtom(p, _, _), _) -> Map.insert(p, 0), Map#{}, rules) |>
        //     stratifyHelper(mkDepGraph(d))
        Datalog::Model(_) => HashMap::new(), // Models contain only facts. 
        Datalog::Join(d1, d2) => { // Map.unionWith(Int32.max, ...) 
            let mut m1 = stratify(*d1);
            let m2 = stratify(*d2);
            m2.iter()
                .for_each(|(k, &v)| match m1.get(&k) {
                    Some(&v1) => { m1.insert(k.clone(), std::cmp::max(v, v1)); () }
                    None => { m1.insert(k.clone(), v); () }
                });
            m1
        },
        _ => todo!(),
    }
}

fn precedence_helper<V>(cnst: Constraint<V>) -> PrecedenceGraph {
    match cnst {
        Constraint::Constraint(head, body) => {
            let mut pg = PrecedenceGraph::new();
            // body
            //     .iter()
            //     .for_each(|bodyp| { let hs = pg.union(&mk_dep_edge(head, bodyp)).collect::<HashSet<PrecedenceEdge>>(); pg = PrecedenceGraph(hs); ()});
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
fn mk_dep_edge<V>(dst: HeadPredicate<V>, src: BodyPredicate<V>) -> PrecedenceGraph {
    match (dst, src) {
        (HeadPredicate::HeadAtom(head_sym, _, _), BodyPredicate::BodyAtom(body_sym, _, Polarity::Positive, Fixity::Loose, _)) => {
            let mut pg = PrecedenceGraph::new();
            pg.insert(PrecedenceEdge::WeakEdge(body_sym, head_sym));
            pg
        },
        //    PrecedenceGraph(Set#{WeakEdge(bodySym, headSym)})
        (HeadPredicate::HeadAtom(head_sym, _, _), BodyPredicate::BodyAtom(body_sym, _, _, _, _)) => {
            let mut pg = PrecedenceGraph::new();
            pg.insert(PrecedenceEdge::StrongEdge(body_sym, head_sym));
            pg
        },
        _ => PrecedenceGraph::new(),
    }
}
