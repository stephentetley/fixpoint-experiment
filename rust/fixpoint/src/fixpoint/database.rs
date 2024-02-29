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
use crate::fixpoint::ast::ram::{RamSym};
use crate::fixpoint::ast::shared::{Value};

// RamSym => (Map: values => value)
#[derive(Debug)]
pub struct Database(pub HashMap<RamSym, HashMap<Vec<Value>, Value>>);


impl Database {
    pub fn new() -> Self {
        Database(HashMap::new())
    }
    
    pub fn insert(&mut self, k: RamSym, v: HashMap<Vec<Value>, Value>) -> bool {
        self.0.insert(k, v).is_some()
    }

    pub fn remove(&mut self, k: &RamSym) -> bool {
        self.0.remove(k).is_some()
    }

    pub fn eval_inplace<A>(&mut self, k: RamSym, f: fn(&HashMap<Vec<Value>, Value>) -> A) -> A {
        let v = self.0.entry(k).or_insert(HashMap::new());
        f(v)
    }

    pub fn eval_inplace_or_insert_mut<A>(&mut self, k: RamSym, f: fn(&mut HashMap<Vec<Value>, Value>) -> A) -> A {
        let v = self.0.entry(k).or_insert(HashMap::new());
        f(v)
    }

}