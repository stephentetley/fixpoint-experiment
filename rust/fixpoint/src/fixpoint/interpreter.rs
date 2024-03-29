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


use std::cmp::Ordering;
use rpds::List;
use crate::fixpoint::ast::ram::{RamStmt, RamTerm, RelOp, BoolExp, RowVar};
use crate::fixpoint::ast::shared::{Value};
use crate::fixpoint::search_env::{SearchEnv};
use crate::fixpoint::database::{Database};


// TODO eliminate over use of `.clone()`


pub fn interpret(stmt: RamStmt) -> Database {
    let mut db = Database::new();
    interpret_with_database(&mut db, stmt);
    db
}

pub fn interpret_with_database(db: &mut Database, stmt: RamStmt) {
    eval_stmt(db, &stmt);
}

fn eval_stmt(db: &mut Database, stmt: &RamStmt) {
    match stmt {
        RamStmt::Insert(rel_op) => { 
            let search_env = alloc_env(0, rel_op);
            eval_op(db, &search_env, rel_op)
        },
        RamStmt::Merge(src_sym, dst_sym) => {
            todo!()
            // let mut m1 = HashMap::new();
            // let mut dst: &mut HashMap<Vec<Value>, Value> = match db.get(dst_sym) {
            //     Some(dst1) => todo!(), // dst1,
            //     None => {db.insert(dst_sym.clone(), m1.clone()); &mut m1},
            // };
            // match ram::into_denotation(&src_sym) {
            //     Denotation::Relational => {
            //         let mut m2 = HashMap::new();
            //         let src_in_db: HashMap<Vec<Value>, Value> = db.get(src_sym).unwrap_or(&m2).clone();
            //         dst.extend(src_in_db.into_iter());
            //     },
            //     Denotation::Latticenal(_, _, lub, _) => {
            //         // MutMap.mergeWith!(lub, MutMap.getWithDefault(srcSym, MutMap.new(rc), db), dst)
            //         todo!()
            //     }
            // }
        },
        RamStmt::Assign(lhs, rhs) => {
            todo!()
            // let mut m1 = HashMap::new();
            // match db.get(lhs) {
            //     None => db.insert(*lhs.clone(), m1),
            //     Some(m2) => db.insert(*lhs.clone(), m2.clone()),
            // };
        },
        RamStmt::Purge(ram_sym) => { db.remove(ram_sym); },
        RamStmt::Seq(stmts) => stmts.iter().for_each(|st| eval_stmt(db, &st)),
        RamStmt::Until(test, body) => {
            let search_env = SearchEnv::new(0);
            if eval_bool_exp(db, &search_env, test) {
                ()
            } else {
                eval_stmt(db, body);
                eval_stmt(db, stmt)
            }
        },
        RamStmt::Comment(_) => (),
        _ => todo!(),
    }
}


fn alloc_env(depth: i32, rel_op: &RelOp) -> SearchEnv {
    match rel_op {
        RelOp::Search(_, _, body)           => alloc_env(depth + 1, &*body),
        RelOp::Query(_, _, _, body)         => alloc_env(depth + 1, &*body),
        RelOp::Functional(_, _, _, body)    => alloc_env(depth + 1, &*body),
        RelOp::Project(..)                  => SearchEnv::new(depth as usize),
        RelOp::If(_, then)                  => alloc_env(depth, &*then),
    }
}

fn eval_op(db: &mut Database, env: &SearchEnv, op: &RelOp) {
    match op {
        RelOp::Search(RowVar::Index(i), ram_sym, body) => {
            // let (tuple_env, lat_env) = env;
            // MutMap.forEach(t -> l -> {
            //     Array.put(t, i, tupleEnv);
            //     Array.put(l, i, latEnv);
            //     eval_op(rc1, db, env, body)
            // }, MutMap.getWithDefault(ramSym, MutMap.new(rc1), db))
            todo!();
        },
        // RelOp::Query(RowVar.Index(i), ramSym, query, body) =>
        //     let (tupleEnv, latEnv) = env;
        //     MutMap.queryWith(evalQuery(env, Vector.toList(query)), t -> l -> {
        //         Array.put(t, i, tupleEnv);
        //         Array.put(l, i, latEnv);
        //         eval_op(rc1, db, env, body)
        //     }, MutMap.getWithDefault(ramSym, MutMap.new(rc1), db))
        // RelOp::Functional(RowVar.Index(i), f, terms, body) =>
        //     let args = terms |> Vector.map(evalTerm(env));
        //     let result = f(args): Vector[Vector[v]];

        //     let (tupleEnv, _latEnv) = env; // TODO: Do we ever need to use latEnv?
        //     foreach (t <- result) {
        //         Array.put(t, i, tupleEnv);
        //         eval_op(rc1, db, env, body)
        //     }
        // RelOp::Project(terms, ramSym) =>
        //     let rel = MutMap.getOrElsePut!(ramSym, MutMap.new(rc1), db);
        //     match toDenotation(ramSym) {
        //         case Denotation.Relational =>
        //             let tuple = Vector.map(evalTerm(env), terms);
        //             MutMap.put!(tuple, Reflect.default(), rel)
        //         case Denotation.Latticenal(bot, leq, lub, _) =>
        //             // assume that length(terms) > 0
        //             let len = Vector.length(terms);
        //             let keyList = terms |> Vector.map(evalTerm(env));
        //             let (key, latValList) = Vector.splitAt(len - 1, keyList);
        //             let latVal = match Vector.head(latValList) {
        //                 case None => bug!("Found predicate without terms")
        //                 case Some(k) => k
        //             };
        //             if (latVal `leq` bot) ()
        //             else MutMap.putWith!(lub, key, latVal, rel)
        //     }
        RelOp::If(test, then) =>
            if eval_bool_exp(db, env, &test) {
                eval_op(db, env, &*then)
            } else {
                ()
            },
        _ => (),
    }
}

fn eval_query(env: &SearchEnv, query: List<&(i32, RamTerm)>, tuple: Vec<Value>) -> Ordering {
    match query.first() {
        None => Ordering::Equal,
        Some(x1) => {
            let (j, t) = x1;
            let vv = tuple[*j as usize].clone();
            let v1 = eval_term(env, t);
            match vv.cmp(&v1) {
                Ordering::Equal => match query.drop_first() {
                    None => eval_query(env, List::new(), tuple),
                    Some(rs) => eval_query(env, rs, tuple),
                },
                cmp => cmp
            }
        },
    }
}

fn eval_bool_exp(db: &mut Database, env: &SearchEnv, exprs: &Vec<BoolExp>) -> bool {
    exprs
        .into_iter()
        .all(|exp| match exp {
            BoolExp::Empty(ram_sym) => {
                db.eval_inplace(ram_sym.clone(), |hm| hm.is_empty())
            },
            BoolExp::NotMemberOf(terms, ram_sym) => {
                todo!()
                // let blank_map = HashMap::new();
                // let rel = db.get(ram_sym).unwrap_or(&blank_map);
                // match ram::into_denotation(ram_sym) {
                //     Denotation::Relational => {
                //         let tuple: Vec<_> = terms.iter().map(|t| eval_term(env, t)).collect();
                //         !rel.contains_key(&tuple)
                //     },
                //     Denotation::Latticenal(bot, leq, _, _) => {
                //         let len = terms.len();
                //         let eval_terms: Vec<_> = terms.iter().map(|t| eval_term(env, t)).collect();
                //         let key = &eval_terms[..len - 1];
                //         let lat_term = match eval_terms.get(len) {
                //             None => panic!("Found predicate without terms"),
                //             Some(l) => l,
                //         };
                //         let rel1 = rel.get(key).unwrap_or(bot);
                //         !(leq)(lat_term.clone(), rel1.clone())
                //     },
                // }
            },
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

// Should this return &Value ?
fn eval_term(env: &SearchEnv, term: &RamTerm) -> Value {
    match term {
        RamTerm::Lit(v) => v.clone(),
        RamTerm::RowLoad(RowVar::Index(i), index) => {
            env.get_tuple_var(*index as usize, *i as usize).clone()
        },
        RamTerm::LoadLatVar(RowVar::Index(i)) => {
            env.get_lat_var(*i as usize).clone()
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
