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


}
