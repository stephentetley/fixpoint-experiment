//
// Copyright 2021 Magnus Madsen
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


use std::fmt;

// Box functional types?
#[derive(Eq, Debug, PartialEq, Hash, Clone)]
pub enum Denotation<V> {
    Relational,
    Latticenal(V, Box<fn(V, V) -> bool>, Box<fn(V, V) -> V>, Box<fn(V, V) -> V>),
}

impl<V> Denotation<V> {
    pub fn is_relational(&self) -> bool {
        match self {
            Denotation::Relational => true,
            _ => false,
        }
    }
}


#[derive(PartialEq, PartialOrd, Eq, Debug, Hash, Clone)]
pub struct PredSym {
    pub name : String,
    pub id: i64,
}

impl fmt::Display for PredSym {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}%{})", self.name, self.id)
    }
}
