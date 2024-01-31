//
// Copyright 2022 Magnus Madsen
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
use crate::fixpoint::ast::shared::PredSym;

// A trait for types that contain predicate symbols.
pub trait PredSymsOf {
    fn pred_syms_of(&self) -> HashSet<PredSym>;
}