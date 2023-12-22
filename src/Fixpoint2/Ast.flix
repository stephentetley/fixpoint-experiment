/*
 * Copyright 2021 Benjamin Dahse, 2023 Stephen Tetley
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

mod Fixpoint2.Ast {

    use Text.Pretty
    use Text.Pretty.{pPrintPrec}
    use Text.PrettyPrint.{Doc, optionalParens, <<>>, text}
    use Text.{appPrec, PrettyLevel}

    use Fixpoint2.Ram.Ast.{RamSym}
    use Fixpoint2.Ast.VarSym.{VarSym}
    use Fixpoint2.Ast.Fixity.{Loose, Fixed}
    use Fixpoint2.Shared.{Denotation, PredSym}


    @Internal
    pub enum PrecedenceGraph(Set[PrecedenceEdge])


    instance Monoid[PrecedenceGraph] {
        pub def empty(): PrecedenceGraph = PrecedenceGraph.PrecedenceGraph(Set.empty())
    }

    instance SemiGroup[PrecedenceGraph] {
        pub def combine(x: PrecedenceGraph, y: PrecedenceGraph): PrecedenceGraph = match (x, y) {
            case (PrecedenceGraph.PrecedenceGraph(xs), PrecedenceGraph.PrecedenceGraph(ys)) => PrecedenceGraph.PrecedenceGraph(Set.union(xs, ys))
        }
    }


    @Internal
    pub enum PrecedenceEdge with Eq, Order {
        case StrongEdge(PredSym, PredSym)
        case WeakEdge(PredSym, PredSym)
    }

    pub enum Datalog[v] {
        case Datalog(Vector[Constraint[v]], Vector[Constraint[v]])
        case Model(Map[RamSym[v], Map[Vector[v], v]])
        case Join(Datalog[v], Datalog[v])
    }

    pub enum Constraint[v] {
        case Constraint(HeadPredicate[v], Vector[BodyPredicate[v]])
    }

    pub enum HeadPredicate[v] {
        case HeadAtom(PredSym, Denotation[v], Vector[HeadTerm[v]])
    }

    pub enum BodyPredicate[v] {
        case BodyAtom(PredSym, Denotation[v], Polarity, Fixity, Vector[BodyTerm[v]])
        case Functional(Vector[VarSym], Vector[v] -> Vector[Vector[v]], Vector[VarSym])
        case Guard0(Unit -> Bool)
        case Guard1(v -> Bool, VarSym)
        case Guard2(v -> v -> Bool, VarSym, VarSym)
        case Guard3(v -> v -> v -> Bool, VarSym, VarSym, VarSym)
        case Guard4(v -> v -> v -> v -> Bool, VarSym, VarSym, VarSym, VarSym)
        case Guard5(v -> v -> v -> v -> v -> Bool, VarSym, VarSym, VarSym, VarSym, VarSym)
    }

    pub enum HeadTerm[v] {
        case Var(VarSym)
        case Lit(v)
        case App0(Unit -> v)
        case App1(v -> v, VarSym)
        case App2(v -> v -> v, VarSym, VarSym)
        case App3(v -> v -> v -> v, VarSym, VarSym, VarSym)
        case App4(v -> v -> v -> v -> v, VarSym, VarSym, VarSym, VarSym)
        case App5(v -> v -> v -> v -> v -> v, VarSym, VarSym, VarSym, VarSym, VarSym)
    }

    pub enum BodyTerm[v] {
        case Wild
        case Var(VarSym)
        case Lit(v)
    }


    pub enum VarSym with Eq, Order, ToString {
        case VarSym(String)
    }

    pub enum Fixity with Eq, Order, ToString {
        case Loose
        case Fixed
    }

    pub enum Polarity {
        case Positive,
        case Negative
    }

    instance Pretty[VarSym] {
        pub override def pPrintPrec(l: PrettyLevel, p: Int32, x: VarSym): Doc = match x {
            case VarSym(x1) => optionalParens(p > appPrec(), text("VarSym") <<>> Pretty.pPrintPrec(l, appPrec() + 1, x1))
        }
    }

    instance Pretty[Fixity] {
        pub override def pPrint(x: Fixity): Doc = match x {
            case Loose => text("Loose")
            case Fixed => text("Fixed")
        }
    }


    instance ToString[Constraint[v]] {
        pub def toString(c: Constraint[v]): String =
            match c {
                case Constraint.Constraint(head, body) =>
                    if (Vector.length(body) == 0)
                        "${head}."
                    else
                        "${head} :- ${body |> Vector.join(", ")}."
            }
    }

    instance ToString[HeadPredicate[v]] {
        pub def toString(head: HeadPredicate[v]): String =
            match head {
                case HeadPredicate.HeadAtom(predSym, Denotation.Relational, terms) => "${predSym}(${terms |> Vector.join(", ")})"
                case HeadPredicate.HeadAtom(predSym, Denotation.Latticenal(_), terms) =>
                    let (keyTerms, latticeTerms) = Vector.splitAt(Vector.length(terms)-1, terms);
                    match Vector.head(latticeTerms) {
                        case None    => "${predSym}(${keyTerms |> Vector.join(", ")})"
                        case Some(l) => "${predSym}(${keyTerms |> Vector.join(", ")}; ${l})"
                    }
            }
    }

    instance ToString[BodyPredicate[v]] {
        pub def toString(body: BodyPredicate[v]): String =
            def polarityPrefix(p) = match p {
                case Polarity.Negative => "not "
                case Polarity.Positive => ""
            };
            def fixityPrefix(f) = match f {
                case Fixity.Fixed => "fix "
                case Fixity.Loose => ""
            };
            match body {
                case BodyPredicate.BodyAtom(predSym, Denotation.Relational, p, f, terms) =>
                    "${polarityPrefix(p)}${fixityPrefix(f)}${predSym}(${terms |> Vector.join(", ")})"
                case BodyPredicate.BodyAtom(predSym, Denotation.Latticenal(_), p, f, terms) =>
                    let n = Vector.length(terms)-1;
                    let (keyTerms, latticeTerms) = (Vector.take(n, terms), Vector.drop(n, terms));
                    match Vector.head(latticeTerms) {
                        case None    => "${polarityPrefix(p)}${fixityPrefix(f)}${predSym}()"
                        case Some(l) => "${polarityPrefix(p)}${fixityPrefix(f)}${predSym}(${keyTerms |> Vector.join(", ")}; ${l})"
                    }
                case BodyPredicate.Functional(boundVars, _, freeVars) => "<loop>(${boundVars}, ${freeVars})"
                case BodyPredicate.Guard0(_) => "<clo>()"
                case BodyPredicate.Guard1(_, v) => "<clo>(${v})"
                case BodyPredicate.Guard2(_, v1, v2) => "<clo>(${v1}, ${v2})"
                case BodyPredicate.Guard3(_, v1, v2, v3) => "<clo>(${v1}, ${v2}, ${v3})"
                case BodyPredicate.Guard4(_, v1, v2, v3, v4) => "<clo>(${v1}, ${v2}, ${v3}, ${v4})"
                case BodyPredicate.Guard5(_, v1, v2, v3, v4, v5) => "<clo>(${v1}, ${v2}, ${v3}, ${v4}, ${v5})"
            }
    }

    instance ToString[HeadTerm[v]] {
        pub def toString(term: HeadTerm[v]): String = match term {
            case HeadTerm.Var(varSym) => "${varSym}"
            case HeadTerm.Lit(v) => "%{v}"
            case HeadTerm.App0(_) => "<clo>()"
            case HeadTerm.App1(_, v) => "<clo>(${v})"
            case HeadTerm.App2(_, v1, v2) => "<clo>(${v1}, ${v2})"
            case HeadTerm.App3(_, v1, v2, v3) => "<clo>(${v1}, ${v2}, ${v3})"
            case HeadTerm.App4(_, v1, v2, v3, v4) => "<clo>(${v1}, ${v2}, ${v3}, ${v4})"
            case HeadTerm.App5(_, v1, v2, v3, v4, v5) => "<clo>(${v1}, ${v2}, ${v3}, ${v4}, ${v5})"
        }
    }

    instance ToString[BodyTerm[v]] {
        pub def toString(term: BodyTerm[v]): String = match term {
            case BodyTerm.Wild => "_"
            case BodyTerm.Var(varSym) => ToString.toString(varSym)
            case BodyTerm.Lit(v) => Debug.stringify(v)
        }
    }

}