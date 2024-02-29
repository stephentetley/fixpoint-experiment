//
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

use fixpoint::fixpoint::ast::shared::*;
use fixpoint::fixpoint::ast::datalog::*;



fn main() {
    println!("Running...\n");
    let ps1 = PredSym {
        name: String::from("parent"),
        id: 10001,
    };
    println!("{}", ps1);

    let dlog1: Datalog = program();
    println!("{}", dlog1);
}

fn program() -> Datalog {
    Datalog::Datalog(vec![], vec![])
}