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

use std::collections::HashSet;
use crate::fixpoint::ast::ram::{BoolExp, RamTerm, RowVar};

///
/// An expression is ground if all its terms are ground.
///
fn is_exp_ground<V>(free_vars: &HashSet<RowVar>, exp: &BoolExp<V>) -> bool {
    match exp {
        BoolExp::Empty(_) => true,
        BoolExp::NotMemberOf(terms, _) => terms.iter().all(|t| is_term_ground(free_vars, t)),
        BoolExp::Eq(lhs, rhs) => is_term_ground(free_vars, lhs) && is_term_ground(free_vars, rhs),
        BoolExp::Leq(_, lhs, rhs) => is_term_ground(free_vars, lhs) && is_term_ground(free_vars, rhs),
        BoolExp::Guard0(_) => true,
        BoolExp::Guard1(_, t) => is_term_ground(free_vars, t),
        BoolExp::Guard2(_, t1, t2) =>
            is_term_ground(free_vars, t1) &&
            is_term_ground(free_vars, t2),
        BoolExp::Guard3(_, t1, t2, t3) =>
            is_term_ground(free_vars, t1) &&
            is_term_ground(free_vars, t2) &&
            is_term_ground(free_vars, t3),
        BoolExp::Guard4(_, t1, t2, t3, t4) =>
            is_term_ground(free_vars, t1) &&
            is_term_ground(free_vars, t2) &&
            is_term_ground(free_vars, t3) &&
            is_term_ground(free_vars, t4),
        BoolExp::Guard5(_, t1, t2, t3, t4, t5) =>
            is_term_ground(free_vars, t1) &&
            is_term_ground(free_vars, t2) &&
            is_term_ground(free_vars, t3) &&
            is_term_ground(free_vars, t4) &&
            is_term_ground(free_vars, t5),
    }
}

///
/// A term is ground if it is a literal or a free variable.
///
fn is_term_ground<V>(free_vars: &HashSet<RowVar>, term: &RamTerm<V>) -> bool {
    match term {
        RamTerm::Lit(_) => true,
        RamTerm::RowLoad(var, _) => free_vars.contains(&var),
        RamTerm::LoadLatVar(var) => free_vars.contains(&var),
        RamTerm::Meet(_, t1, t2) =>
            is_term_ground(free_vars, t1) &&
            is_term_ground(free_vars, t2),
        RamTerm::App0(_) => true,
        RamTerm::App1(_, t) => is_term_ground(free_vars, t),
        RamTerm::App2(_, t1, t2) =>
            is_term_ground(free_vars, t1) &&
            is_term_ground(free_vars, t2),
        RamTerm::App3(_, t1, t2, t3) =>
            is_term_ground(free_vars, t1) &&
            is_term_ground(free_vars, t2) &&
            is_term_ground(free_vars, t3),
        RamTerm::App4(_, t1, t2, t3, t4) =>
            is_term_ground(free_vars, t1) &&
            is_term_ground(free_vars, t2) &&
            is_term_ground(free_vars, t3) &&
            is_term_ground(free_vars, t4),
        RamTerm::App5(_, t1, t2, t3, t4, t5) =>
            is_term_ground(free_vars, t1) &&
            is_term_ground(free_vars, t2) &&
            is_term_ground(free_vars, t3) &&
            is_term_ground(free_vars, t4) &&
            is_term_ground(free_vars, t5),
    }
}
