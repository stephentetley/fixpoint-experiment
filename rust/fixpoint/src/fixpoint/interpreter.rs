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
use std::cmp::Ordering;
use rpds::List;
use crate::fixpoint::ast::ram::{RamTerm, BoolExp, RamSym, RowVar};

pub type Database<V> = HashMap<RamSym<V>, HashMap<Vec<V>, V>>;

pub type SearchEnv<V> = (Vec<Vec<V>>, Vec<V>);

fn eval_query<V: Ord + std::clone::Clone + std::fmt::Display>(env: &SearchEnv<V>, query: List<&(i32, RamTerm<V>)>, tuple: Vec<V>) -> Ordering {
    match query.first() {
        Option::None => Ordering::Equal,
        Option::Some(x1) => {
            let (j, t) = x1;
            let vv = tuple[*j as usize].clone();
            let v1 = eval_term(env, t);
            match vv.cmp(&v1) {
                Ordering::Equal => match query.drop_first() {
                    Option::None => eval_query(env, List::new(), tuple),
                    Option::Some(rs) => eval_query(env, rs, tuple),
                },
                cmp => cmp
            }
        },
    }
}

fn eval_bool_exp<V: Ord + std::clone::Clone + std::fmt::Display>(db: &Database<V>, env: &SearchEnv<V>, exprs: &Vec<BoolExp<V>>) -> bool {
    exprs
        .into_iter()
        .all(|exp| match exp {
            // BoolExp::Empty(ramSym) =>
            //     MutMap.isEmpty(MutMap.getWithDefault(ramSym, MutMap.new(rc1), db))
            // BoolExp::NotMemberOf(terms, ramSym) =>
            //     let rel = MutMap.getWithDefault(ramSym, MutMap.new(rc1), db);
            //     match toDenotation(ramSym) {
            //         case Denotation.Relational =>
            //             let tuple = Vector.map(eval_term(env), terms);
            //             not MutMap.memberOf(tuple, rel)
            //         case Denotation.Latticenal(bot, leq, _, _) =>
            //             let len = Vector.length(terms);
            //             let eval_terms = Vector.map(eval_term(env), terms);
            //             let key = Vector.take(len - 1, eval_terms);
            //             let latTerms = Vector.drop(len - 1, eval_terms);
            //             let latTerm = match Vector.head(latTerms) {
            //                 case None => bug!("Found predicate without terms")
            //                 case Some(hd) => hd
            //             };
            //             not (latTerm `leq` MutMap.getWithDefault(key, bot, rel))
            //     }
            BoolExp::Eq(lhs, rhs) => {
                let lhs1 = eval_term(env, lhs);
                let rhs1 = eval_term(env, rhs);
                lhs1 == rhs1
            },
            BoolExp::Leq(leq, lhs, rhs) => {
                let lhs1 = eval_term(env, lhs);
                let rhs1 = eval_term(env, rhs);
                leq(lhs1, rhs1)
            },
            BoolExp::Guard0(f) => f(),
            BoolExp::Guard1(f, t) => {
                let v = eval_term(env, t);
                f(v)
            },
            BoolExp::Guard2(f, t1, t2) => {
                let v1 = eval_term(env, t1);
                let v2 = eval_term(env, t2);
                f(v1, v2)
            },
            BoolExp::Guard3(f, t1, t2, t3) => {
                let v1 = eval_term(env, t1);
                let v2 = eval_term(env, t2);
                let v3 = eval_term(env, t3);
                f(v1, v2, v3)
            },
            BoolExp::Guard4(f, t1, t2, t3, t4) => {
                let v1 = eval_term(env, t1);
                let v2 = eval_term(env, t2);
                let v3 = eval_term(env, t3);
                let v4 = eval_term(env, t4);
                f(v1, v2, v3, v4)
            },
            BoolExp::Guard5(f, t1, t2, t3, t4, t5) => {
                let v1 = eval_term(env, t1);
                let v2 = eval_term(env, t2);
                let v3 = eval_term(env, t3);
                let v4 = eval_term(env, t4);
                let v5 = eval_term(env, t5);
                f(v1, v2, v3, v4, v5)
            },
            _ => todo!()
        })
    }

fn eval_term<V: std::fmt::Display + std::clone::Clone>(env: &SearchEnv<V>, term: &RamTerm<V>) -> V {
    match term {
        RamTerm::Lit(v) => v.clone(),
        RamTerm::RowLoad(RowVar::Index(i), index) => {
            let (tuple_env, _) = env;
            // tuple_env[index as usize][i as usize].clone()
            todo!()
        },
        RamTerm::LoadLatVar(RowVar::Index(i)) => {
            let (_, lat_env) = env;
            lat_env[*i as usize].clone()
        },
        RamTerm::Meet(cap, lhs, rhs) => {
            let v1 = eval_term(env, lhs);
            let v2 = eval_term(env, rhs);
            cap(v1, v2)
        }
        RamTerm::App0(f) => f(),
        RamTerm::App1(f, t) => {
            let v = eval_term(env, t);
            f(v)
        },
        RamTerm::App2(f, t1, t2) => {
            let v1 = eval_term(env, t1);
            let v2 = eval_term(env, t2);
            f(v1, v2)
        },
        RamTerm::App3(f, t1, t2, t3) => {
            let v1 = eval_term(env, t1);
            let v2 = eval_term(env, t2);
            let v3 = eval_term(env, t3);
            f(v1, v2, v3)
        },
        RamTerm::App4(f, t1, t2, t3, t4) => {
            let v1 = eval_term(env, t1);
            let v2 = eval_term(env, t2);
            let v3 = eval_term(env, t3);
            let v4 = eval_term(env, t4);
            f(v1, v2, v3, v4)
        },
        RamTerm::App5(f, t1, t2, t3, t4, t5) => {
            let v1 = eval_term(env, t1);
            let v2 = eval_term(env, t2);
            let v3 = eval_term(env, t3);
            let v4 = eval_term(env, t4);
            let v5 = eval_term(env, t5);
            f(v1, v2, v3, v4, v5)
        },
        RamTerm::RowLoad(row_var, index) => panic!("Illegal RowLoad with {} {}", row_var, index),
        _ => panic!("Illegal term {}", term),
    }
}
