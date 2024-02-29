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
pub enum Denotation {
    Relational,
    Latticenal(Value, Box<fn(Value, Value) -> bool>, Box<fn(Value, Value) -> Value>, Box<fn(Value, Value) -> Value>),
}

impl Denotation {
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

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Clone)]
pub enum Value {
    Bool(bool),
    Int32(i32),
    Str(String),
}


impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{}", b),
            Value::Int32(i) => write!(f, "{}", i),
            Value::Str(s) => write!(f, "{}", s),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Bool(false)
    }
}