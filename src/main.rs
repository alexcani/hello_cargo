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
}
