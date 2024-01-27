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
use crate::fixpoint::ast::ram::{RamTerm, RowVar};

///
/// Returns the set of variables that occur in `term`.
///
fn ram_term_vars<V>(term: RamTerm<V>) -> HashSet<RowVar> {
    match term {
        RamTerm::Lit(_) => HashSet::new(),
        RamTerm::RowLoad(var, _) => { 
            let mut s = HashSet::new();
            s.insert(var);
            s
        },
        RamTerm::LoadLatVar(var) => { 
            let mut s = HashSet::new();
            s.insert(var);
            s
        },
        RamTerm::Meet(_, lhs, rhs) => {
            let mut s1 = ram_term_vars(*lhs);
            let s2 = ram_term_vars(*rhs);
            s1.extend(s2);
            s1
        },
        RamTerm::App0(_) => HashSet::new(),
        RamTerm::App1(_, t) => ram_term_vars(*t),
        RamTerm::App2(_, t1, t2) => {
            let mut s1 = ram_term_vars(*t1);
            let s2 = ram_term_vars(*t2);
            s1.extend(s2);
            s1
        },
        RamTerm::App3(_, t1, t2, t3) => {
            let mut s1 = ram_term_vars(*t1);
            let s2 = ram_term_vars(*t2);
            let s3 = ram_term_vars(*t3);
            s1.extend(s2);
            s1.extend(s3);
            s1
        },
        RamTerm::App4(_, t1, t2, t3, t4) => {
            let mut s1 = ram_term_vars(*t1);
            let s2 = ram_term_vars(*t2);
            let s3 = ram_term_vars(*t3);
            let s4 = ram_term_vars(*t4);
            s1.extend(s2);
            s1.extend(s3);
            s1.extend(s4);
            s1
        },
        RamTerm::App5(_, t1, t2, t3, t4, t5) => {
            let mut s1 = ram_term_vars(*t1);
            let s2 = ram_term_vars(*t2);
            let s3 = ram_term_vars(*t3);
            let s4 = ram_term_vars(*t4);
            let s5 = ram_term_vars(*t5);
            s1.extend(s2);
            s1.extend(s3);
            s1.extend(s4);
            s1.extend(s5);
            s1
        },
    }
}