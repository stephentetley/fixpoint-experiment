def main(): Int32 \ IO = 
    println("Pump...");
    noPump();
    0

def noPump(): Unit \ IO = 
    let p = #{
        
        System("WDC01-WT-SYS01", "SPMS", "WCD01-WT").
        SubSystem("WDC01-WT-SYS01-KIS01", "KISK", "WDC01-WT-SYS01").
        Pump("ZAN01-WT-SYS02-PMP01", "Auto Pump-1").

        HasPump(floc) :-  
            System(floc, "SPMS", _parent),
            SubSystem(ssfloc, "PUMP", floc),
            Pump(ssfloc, _name).

        NoPump(floc) :-  
            System(floc, "SPMS", _),
            not HasPump(floc).

    };
    println("# No pump:");
    query p select (x1) from NoPump(x1) |> Vector.forEach(println);
    ()