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

pub struct Database<V>(pub HashMap<RamSym<V>, HashMap<Vec<V>, V>>);


impl<V: Eq + std::hash::Hash> Database<V> {
    pub fn new() -> Self {
        Database(HashMap::new())
    }
    
    pub fn insert(&mut self, k: RamSym<V>, v: HashMap<Vec<V>, V>) -> bool {
        self.0.insert(k, v).is_some()
    }

    pub fn remove(&mut self, k: &RamSym<V>) -> bool {
        self.0.remove(k).is_some()
    }
}