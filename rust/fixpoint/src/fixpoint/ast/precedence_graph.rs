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

pub struct PrecedenceGraph(pub HashSet<PrecedenceEdge>);

impl PrecedenceGraph {
    pub fn new() -> Self {
        PrecedenceGraph(HashSet::new())
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
