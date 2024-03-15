// SCC
// from Souffle paper - On Fast Large-Scale Program Analysis in Datalog

def main(): Int32 \ IO = 
    println("SCC...");
    scc();
    0

def scc(): Unit \ IO = 
    let p = #{
        
        Edge(1, 2).
        Edge(2, 3).
        Edge(3, 4).
        
        Path(x, y) :-  Edge(x, y).

        Path(x, z) :- 
            Path(x, y), 
            Edge(y, z).

    };
    query p select (x1, x2) from Path(x1, x2) |> Vector.forEach(println);
    ()

