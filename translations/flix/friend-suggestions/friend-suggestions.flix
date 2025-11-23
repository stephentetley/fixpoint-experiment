// From flix source tree `examples/larger-examples/datalog`
// This Datalog program has negation and an if clause (closure).

def main(): Int32 \ IO = 
    println("friend-suggestions...");
    demoFriendSuggestions01();
    0

def demoFriendSuggestions01(): Unit \ IO = 
    let friends = List#{ ("George", "Antonio"), ("George", "Sarah"), ("George", "Roberto"), ("Sarah", "Hisham"), ("Antonio", "Hisham"), ("Roberto", "Hisham")};
    friendSuggestions(friends) |> Vector.forEach(println)


// must have three friends that have your not-yet friend as a friend
pub def friendSuggestions(friends: List[(person, person)]): Vector[(person, person)] with Order[person] =
    let f = inject friends into Friend/2;
    let lp = #{
        Suggestion(me, nf) :-
            Friend(me, f1), Friend(me, f2), Friend(me, f3),
            Friend(f1, nf), Friend(f2, nf), Friend(f3, nf),
            not Friend(me, nf),
            if (f1 != f2 and f2 != f3 and f1 != f3).
    };
    query f, lp select (x, y) from Suggestion(x, y)


