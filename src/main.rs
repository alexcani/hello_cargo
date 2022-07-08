fn main() {
    let mut v: Vec<i32> = Vec::new();  // empty, type annotated
    v.push(1);
    v.push(2);
    v.push(3);  // add elements to back

    // Iterate
    for i in &v {  // immutable
        println!("{}", i)
    }

    // mutable iteration
    for i in &mut v {
        *i = *i+1;
        println!("{}", i);
    }

    println!("last is {}", v.pop().unwrap());  // removes and returns the last element (transfers ownership (returns T))

    {
    let mut v = vec![1, 2, 3, 4, 5];  // containing values 1, 2, 3, 4, 5 type inferred as i32
    let third: &i32 = &v[2];  // reference to 3rd element, [] crashes when out of bounds
    println!("Third element is {}", third);

    match v.get(2) {  // using get, no crash, forces you to treat the None case
        Some(x) => println!("Third element is {}", x),
        None => println!("There is no third element")
    }

    // Borrow checking rules still valid
    // Third is an immutable borrow from v, so we cannot do mutable borrow then use 'third'
    v.push(6);
    // println!("Third element is {}", third);  won't compile. v.push works because of non-lexical lifetime
    }  // v goes out of scope and is freed



    // Strings
    let mut s = String::new();  // new string
    s.push('c'); // for chars
    s.push_str("for slices");

    let r = "string_literal";
    let t = r.to_string();  // from literal to String
    let v = "string_literal".to_string();  // oneliner
    let n = String::from("yet another way");

    println!("{} {} {} {}", s, t, v, n);

    // Strings (and str) are UTF-8
    let x = String::from("😋");
    println!("{}", x);

    println!("concatenate: {}", n + &x);  // + works on string slices (&str) not String
                                          // also 'n' has been moved
    // println!("{}", n); won't work

    let tic = "tic".to_string();
    let tac = "tac".to_string();
    let toe = "toe".to_string();
    let s = tic + "-" + &tac + "-" + &toe;  // only tic is moved
    println!("{}", s);
    // println!("{}", tic);  won't work
    // Using format
    let tic = "tic".to_string();
    let s = format!("{}-{}-{}", tic, tac, toe);  // no one is moved
    println!("{} {}", s, tic);  // works

    // Indexing strings
    // Can't index properly like in most languages because UTF-8 encoding
    // has variable length. Even string slices are dangerous, because the boundaries
    // must match the characters boundaries
    for c in "नमस्ते".chars() {  // .chars iterates over each valid UTF-8 character
        // which is not necessarely a grepheme cluster (a "full" character)
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {  // .bytes() returns each byte, 18 in this case
        println!("{}", b);
    }

    // HashMaps
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 30);

    {
        // Also by using 'collect' on a vector of tuples
        // Vectors for team names and scores
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        // creates vector of tuples by zipping the iterators, then collects
        // Needs to specify HashMap because can collect to other data structures as well
        // types in HashMap stay inferred by using _
        let mut _scores: HashMap<_, _> =
            teams.into_iter().zip(initial_scores.into_iter()).collect();
    }

    let key_name = String::from("Yellow");
    let key_value = 30;
    scores.insert(key_name, key_value);
    // key_name is moved into the map, key_name is now invalid
    //println!("key name: {}", key_name);  won't work

    // Accessing
    let blue_value = scores.get("Blue");
    // blue_value is an Option with a reference to the value inside
    match blue_value {
        Some(x) => println!("Blue's score is {}", x),
        None => ()
    }

    // or with if let
    if let Some(x) = scores.get("Blue") {
        println!("Blue's score is {}", x);
    }

    // Iterate
    for (key, value) in scores.iter() {
        println!("{}: {}", key, value);
    }
    // or
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Overwriting value
    scores.insert(String::from("Red"), 50);
    scores.insert(String::from("Red"), 60);
    println!("{:?}", scores);

    // Only insert if key doesn't exist: entry
    scores.entry(String::from("Purple")).or_insert(140);
    scores.entry(String::from("Purple")).or_insert(160);  // 140 remains
    println!("{:?}", scores);
    // or_insert returns a mutable reference to the value, either the new one or the previous one

    // and_modify provides inplace modification if value exists
    scores.entry(String::from("Purple")).and_modify(|e| {*e += 1});  // uses lambda to modify element inplace

    // nice in constructs such as:
    scores.entry(String::from("Brown")).and_modify(|e| {*e += 1}).or_insert(50);
    println!("{:?}", scores);
    // if exists, increment by 1, otherwise insert with 50
    scores.entry(String::from("Brown")).and_modify(|e| {*e += 1}).or_insert(50);
    println!("{:?}", scores);

}
