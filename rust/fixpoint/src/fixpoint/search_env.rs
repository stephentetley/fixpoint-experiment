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


use crate::fixpoint::ast::shared::{Value};

#[derive(Debug)]
pub struct SearchEnv(Vec<Vec<Value>>, Vec<Value>);

// TODO - do we want a mutable env, Flix uses arrays so it suggest we do?

impl SearchEnv {
    pub fn new(level: usize) -> Self {
        SearchEnv(vec![Vec::new(); level], vec![Default::default(); level])
    }

    
    pub fn get_tuple_var(&self, i: usize, j: usize) -> &Value {
        let SearchEnv(tuple_env, _) = self;
        &tuple_env[i][j]
    }

    pub fn get_lat_var(&self, i: usize) -> &Value {
        let SearchEnv(_, lat_env) = self;
        &lat_env[i]
    }

    pub fn update_tuple_env(&mut self, i: usize, vals: Vec<Value>) {
        let SearchEnv(tuple_env, _) = self;
        tuple_env[i] = vals;
    }

    pub fn update_lat_env(&mut self, i: usize, vals: Value) {
        let SearchEnv(_, lat_env) = self;
        lat_env[i] = vals;
    }
}
