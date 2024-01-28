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
use crate::fixpoint::ast::ram::{RamStmt, RelOp, BoolExp, RamTerm, RowVar};

///
/// Hoist and query optimize `stmt`.
///
/// This is an optimization phase and can be omitted with no semantic effect.
///
/// The purpose of this phase is to:
/// 1) Hoist if-statements from inner loops to outer loops.
/// An if-statement can be hoisted from its enclosing loop if it only reads variables
/// that are bound in an outer loop.
/// 2) Rewrite searches on relations to queries on indices, when possible.
/// It is possible to rewrite a search when it searches on attributes that form a prefix
/// of the attribute sequence used to lexicographically define the index.
/// Consider the following example:
/// search x ∈ B do
///     search (y, z, w) ∈ R do
///         search u ∈ C do
///             if (x = y ∧ x = w ∧ z = u ∧ x ∉ A) then
///                 project x into A
/// After step 1 (hoisting):
/// search x ∈ B do
///     if (x ∉ A) then
///         search (y, z, w) ∈ R do
///             if (x = y ∧ x = w) then
///                 search u ∈ C do
///                     if (z = u) then
///                         project x into A
/// After step 2 (query rewriting):
/// search x ∈ B do
///     if (x ∉ A) then
///         query {(y, z, w) ∈ R | x = y} do
///             if (x = w) then
///                 query {u ∈ C | z = u} do
///                     project x into A
///
/// Note how the search `x = y` is optimized, but `x = w` is not.
/// The index for R is defined by the attribute sequence Y < Z < W.
/// Therefore the search `x = y` is a prefix search.
/// But `x = w` is not part of any prefix and hence cannot be optimized.
///
/// Step 1 and Step 2 are implemented as one pass.
///
pub fn query_stmt<V>(stmt: RamStmt<V>) -> RamStmt<V> {
    match stmt {
        // RamStmt::Insert(op) => {
        //     let (inner_op, ground) = query_op(&HashSet::new(), op);
        //     if ground.is_empty() {
        //         RamStmt::Insert(inner_op)
        //     } else {
        //         RamStmt::Insert(RelOp::If(ground, Box::new(inner_op)))
        //     }
        // },
        RamStmt::Merge(_, _) => stmt,
        RamStmt::Assign(_, _) => stmt,
        RamStmt::Purge(_) => stmt,
        // RamStmt::Seq(xs) => RamStmt::Seq(Vector.map(queryStmt, xs)),
        // RamStmt::Until(test, body) => RamStmt::Until(test, queryStmt(body)),
        RamStmt::Comment(_) => stmt,
        _ => todo!(),
    }
}

/// Hoist and query optimize `op`.
///
/// `freeVars` is the set of variables bound by an outer loop.
/// Returns the optimized op and the conditions that occur in `op` that have to be hoisted.
///
fn query_op<V>(op: RelOp<V>, _free_vars: HashSet<RowVar>) -> (RelOp<V>, Vec<BoolExp<V>>) { 
    match op {
        // RelOp::Search(var, ramSym, body) =>
        //     use Fixpoint.Ast.Ram.BoolExp.Eq;
        //     use Fixpoint.Ast.Ram.RamTerm.{RowLoad, Lit};
        //     let (innerOp, innerGround) = queryOp(body, Set.insert(var, freeVars));
        //     let (ground, notGround) = Vector.partition(isExpGround(freeVars), innerGround);
        //     let (varQuery, rest1) =
        //         // Make sure `var` is on the lhs of all equalities.
        //         Vector.map(exp -> match exp {
        //             case Eq(RowLoad(row1, i), RowLoad(row2, j)) =>
        //                 if (row2 == var)
        //                     Eq(RowLoad(row2, j), RowLoad(row1, i))
        //                 else
        //                     exp
        //             case Eq(Lit(v), RowLoad(row, i)) => Eq(RowLoad(row, i), Lit(v))
        //             case _ => exp
        //         }, notGround) |>
        //         // Partition into those equalities that have `var` on the lhs and those that don't.
        //         Vector.partition(exp -> match exp {
        //             case Eq(RowLoad(row1, _), RowLoad(row2, _)) => row1 != row2 and row1 == var
        //             case Eq(RowLoad(row, _), Lit(_)) => row == var
        //             case _ => false
        //         });
        //     let (prefixQuery, rest2) = longestPrefixQuery(varQuery);
        //     let test = Vector.append(rest1, rest2);
        //     if (Vector.isEmpty(prefixQuery))
        //         if (Vector.isEmpty(test))
        //             let search = RelOp.Search(var, ramSym, innerOp);
        //             (search, ground)
        //         else
        //             let search = RelOp.Search(var, ramSym, RelOp.If(test, innerOp));
        //             (search, ground)
        //     else
        //         let query =
        //             Vector.map(x -> match x {
        //                 case Eq(RamTerm.RowLoad(_, j), rhs) => (j, rhs)
        //                 case _ => ???
        //             }, prefixQuery);
        //         if (Vector.isEmpty(test))
        //             let search = RelOp.Query(var, ramSym, query, innerOp);
        //             (search, ground)
        //         else
        //             let search = RelOp.Query(var, ramSym, query, RelOp.If(test, innerOp));
        //             (search, ground)
        RelOp::Query(_, _, _, _) => (op, Vec::new()),
        RelOp::Functional(_, _, _, _) => (op, Vec::new()),
        RelOp::Project(_, _) => (op, Vec::new()),
        // RelOp::If(test, then) =>
        //     let (innerOp, innerGround) = queryOp(then, freeVars);
        //     (innerOp, Vector.append(test, innerGround))
        _ => todo!(),
    }
}


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
