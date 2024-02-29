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
use crate::fixpoint::ast::ram::{RamStmt, RamTerm, RamSym, RowVar};

///
/// Optimize and simplify `stmt` by deleting redundant code and reordering code.
/// Examples of redundancy include `x[i] == x[i]` or `x ⊓ y ≤ x`.
/// Reordering means `(0, 1) ∉ Path ∧ x[1] = y[0]` would be swapped `x[1] = y[0] ∧ (0, 1) ∉ Path`.
/// A simple static analysis also reveals that the following join-loop is redundant
/// in stratum `j` if `C` is computed in stratum `i` and `i < j`:
///     search B$1 ∈ B do
///         search C$2 ∈ ΔC do
///             if (B$1[0] == C$2[0] ∧ (B$1[0]) ∉ A ∧ (B$1[0]) ∉ R) then
///                 project (B$1[0]) into ΔR'
///             end
///         end
///     end
///
pub fn simplify_stmt(stmt: RamStmt) -> RamStmt {
    Option::unwrap_or(simplify_helper(&HashSet::new(), stmt), RamStmt::Seq(Vec::new()))
}

fn simplify_helper(_stratum: &HashSet<RamSym>, stmt: RamStmt) -> Option<RamStmt> {
    match stmt {
        // RamStmt::Insert(op) =>
        //     forM (
        //         newOp <- simplifyOp(stratum, op)
        //     ) yield {
        //         match newOp {
        //             //
        //             // DANGER! WILL ROBINSON! The following code is incorrect when a relation is being merged into a lattice.
        //             //
        //             // See #4719.
        //             //
        //             // // Rewrite join loops that copy one relation into another into a `merge` statement.
        //             // // search b ∈ B do
        //             // //   project (b[0], b[1]) into A
        //             // // end
        //             // // ==>
        //             // // merge B into A
        //             // // If A and B have the same arity.
        //             // case RelOp.Search(varB, relB, RelOp.Project(tuple, relA)) =>
        //             //     use Fixpoint.Ram.arityOf;
        //             //     let isCopy =
        //             //         Vector.map(t -> match t {
        //             //             case RamTerm.RowLoad(var, i) => if (varB == var) i else -1
        //             //             case _ => -1
        //             //         }, tuple) == Vector.range(0, arityOf(relB));
        //             //     if (isCopy)
        //             //         RamStmt.Merge(relB, relA)
        //             //     else
        //             //         RamStmt.Insert(newOp)
        //             case _ => RamStmt.Insert(newOp)
        //         }
        //     }
        RamStmt::Merge(_, _) => Some(stmt),
        RamStmt::Assign(_, _) => Some(stmt),
        RamStmt::Purge(_) => Some(stmt),
        // RamStmt::Seq(xs) =>
        //     Some(RamStmt.Seq(Vector.filterMap(simplifyHelper(stratum), xs)))
        // RamStmt::Until(test, body) =>
        //     let newStratum = Vector.foldLeft(acc -> e -> match e {
        //         case BoolExp.Empty(ramSym) => Set.insert(ramSym, acc)
        //         case _ => acc
        //     }, Set#{}, test);
        //     simplifyHelper(newStratum, body) |>
        //     Option.map(newBody -> RamStmt.Until(test, newBody))
        RamStmt::Comment(_) => Some(stmt),
        _ => todo!(),
    }
}


///
/// Returns the set of variables that occur in `term`.
///
fn ram_term_vars(term: RamTerm) -> HashSet<RowVar> {
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
