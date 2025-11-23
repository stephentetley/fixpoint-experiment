// From flix source tree `examples/larger-examples/datalog`
// This Datalog program has custom conditions in the body predicates
// They get translated to `<clo>` in the RAM program.

def main(): Int32 \ IO = 
    println("drivable-speed...");
    demoDrivable01();
    0



pub def drivable(source: String, destination: String, minimumSpeed: Int32): Bool =
    let lp = #{
        Road("Rome", 80, "Turin"). 
        Road("Turin", 70, "Naples"). 
        Road("Naples", 50, "Florence").

        Path(x, y) :- Road(x, maximumSpeed, y), if (maximumSpeed > minimumSpeed).
        Path(x, z) :- Path(x, y), Road(y, maximumSpeed, z), if (maximumSpeed > minimumSpeed).
    };
    not Vector.isEmpty(query lp select () from Path(source, destination))

pub def demoDrivable01() : Unit \ IO = 
    let ans = drivable("Rome", "Florence", 30);
    if (ans) println("Drivable - Rome to Florence") else println("Not drivable")
