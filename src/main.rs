fn main() {
    let v1 = vec![0, 2, 4, 6];
    let v1_iter = v1.iter();
    for val in v1_iter {  // consumes the iterator
        println!("Value: {}", val);
    }

    // let total: i32 = v1_iter.sum();  // iterator already consumed, need a new one
    let total: i32 = v1.iter().sum();  // sum also consumes the iterator
    println!("Total: {}", total);

    // Methods that consume iterators = "consuming adaptors"

    // v1.into_iter() -> iterator into owned values from v1
    // v1.iter_mut()  -> iterator to mutable references

    // Iterator adaptors
    v1.iter().map(|x| x+1);  // does nothing, but creates a new iterator pointing to result of adding 1 to all elements
    // iterators are lazy and do nothing unless consumed

    let result: Vec<_> = v1.iter().map(|x| x+1).collect();  // collect consumes the iterator into a new collection, need to annotate type
    // can use _ because rust can deduct the type, we're only specifying we want a Vec
    assert_eq!(result, vec![1, 3, 5, 7]);
}
