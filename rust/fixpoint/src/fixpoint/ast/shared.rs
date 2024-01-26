/*
 * Copyright 2021 Magnus Madsen
 * Copyright 2021 Benjamin Dahse
 * Copyright 2024 Stephen Tetley
 * 
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::fmt;

pub enum Denotation<V> {
    Relational,
    Latticenal(V, fn(V, V) -> bool, fn(V, V) -> V, fn(V, V) -> V),
}

pub fn is_relational<V>(den: Denotation<V>) -> bool {
    match den {
        Denotation::Relational => true,
        _ => false,
    }
}

#[derive(PartialEq, PartialOrd, Eq, Hash)]
pub struct PredSym {
    pub name : String,
    pub id: i64,
}

// Implement `Display` for `PredSym`.
impl fmt::Display for PredSym {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}%{})", self.name, self.id)
    }
}
