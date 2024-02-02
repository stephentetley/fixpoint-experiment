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


use crate::fixpoint::ast::shared::PredSym;
use std::collections::HashSet;
use std::collections::hash_set::Union;
use std::collections::hash_map::RandomState;

pub struct PrecedenceGraph(pub HashSet<PrecedenceEdge>);

// pub type PrecedenceGraphUnion<'a> = Union<'a, &PrecedenceEdge, RandomState>

impl PrecedenceGraph {
    pub fn new() -> Self {
        PrecedenceGraph(HashSet::new())
    }

    pub fn insert(&mut self, value: PrecedenceEdge) -> bool {
        self.0.insert(value)
    }

    pub fn union<'a>(&'a self, other: &'a PrecedenceGraph) -> Union<'a, PrecedenceEdge, RandomState> {
        self.0.union(&other.0)
    }

}


#[derive(PartialEq, PartialOrd, Eq, Hash)]
pub enum PrecedenceEdge {
    StrongEdge(PredSym, PredSym),
    WeakEdge(PredSym, PredSym),
}



// pub fn combine(x: mut PrecedenceGraph, y: PrecedenceGraph) -> PrecedenceGraph {
//     let hs1 = x.0;
//     let hs2 = y.0;
//     PrecedenceGraph(hs1.extend(&hs2))
// }
