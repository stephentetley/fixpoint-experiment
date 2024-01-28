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
use crate::fixpoint::ast::ram::{RamTerm, RamSym, RowVar};

pub type Database<V> = HashMap<RamSym<V>, HashMap<Vec<V>, V>>;

pub type SearchEnv<V> = (Vec<Vec<V>>, Vec<V>);

fn eval_term<V: std::fmt::Display + std::clone::Clone>(env: &SearchEnv<V>, term: RamTerm<V>) -> V {
    match term {
        RamTerm::Lit(v) => v,
        RamTerm::RowLoad(RowVar::Index(i), index) => {
            let (tuple_env, _) = env;
            tuple_env[index as usize][i as usize].clone()
        },
        RamTerm::LoadLatVar(RowVar::Index(i)) => {
            let (_, lat_env) = env;
            lat_env[i as usize].clone()
        },
        RamTerm::Meet(cap, lhs, rhs) => {
            let v1 =eval_term(env, *lhs);
            let v2 = eval_term(env, *rhs);
            cap(v1, v2)
        }
        RamTerm::App0(f) => f(),
        RamTerm::App1(f, t) => {
            let v = eval_term(env, *t);
            f(v)
        },
        RamTerm::App2(f, t1, t2) => {
            let v1 = eval_term(env, *t1);
            let v2 = eval_term(env, *t2);
            f(v1, v2)
        },
        RamTerm::App3(f, t1, t2, t3) => {
            let v1 = eval_term(env, *t1);
            let v2 = eval_term(env, *t2);
            let v3 = eval_term(env, *t3);
            f(v1, v2, v3)
        },
        RamTerm::App4(f, t1, t2, t3, t4) => {
            let v1 = eval_term(env, *t1);
            let v2 = eval_term(env, *t2);
            let v3 = eval_term(env, *t3);
            let v4 = eval_term(env, *t4);
            f(v1, v2, v3, v4)
        },
        RamTerm::App5(f, t1, t2, t3, t4, t5) => {
            let v1 = eval_term(env, *t1);
            let v2 = eval_term(env, *t2);
            let v3 = eval_term(env, *t3);
            let v4 = eval_term(env, *t4);
            let v5 = eval_term(env, *t5);
            f(v1, v2, v3, v4, v5)
        },
        RamTerm::RowLoad(row_var, index) => panic!("Illegal RowLoad with {} {}", row_var, index),
        _ => panic!("Illegal term {}", term),
    }
}
