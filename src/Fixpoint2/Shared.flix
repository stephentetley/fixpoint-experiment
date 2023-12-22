/*
 * Copyright 2021 Benjamin Dahse
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
mod Fixpoint2.Shared {

    @Internal
    pub enum PredSym with Eq, Order {
        case PredSym(String, Int64)
    }

    instance ToString[PredSym] {
        pub def toString(predSym: PredSym): String = match predSym {
            case PredSym.PredSym(name, id) => if (id == 0i64) name else "${name}%${id}"
        }
    }

    pub enum Denotation[v] {
        case Relational
        case Latticenal(v, v -> v -> Bool, v -> v -> v, v -> v -> v)
    }

    ///
    /// Returns `true` if the given denotation is relational.
    ///
    pub def isRelational(den: Denotation[v]): Bool = match den {
        case Denotation.Relational => true
        case _          => false
    }

    ///
    /// Returns a latticenal denotation associated with the type `t`.
    ///
    @Internal
    pub def lattice(): Denotation[t] with LowerBound[t], JoinLattice[t], MeetLattice[t] =
        // TODO: Why are these type annotations necessary?
        Denotation.Latticenal(
            LowerBound.minValue() : t,
            (x: t, y: t) -> PartialOrder.lessEqual(x: t, y: t),
            (x: t, y: t) -> JoinLattice.leastUpperBound(x: t, y: t),
            (x: t, y: t) -> MeetLattice.greatestLowerBound(x: t, y:t)
        )

}
