use adder::Guess;

#[test]
fn test_struct() {
    Guess::new(40);
}

#[test]
#[should_panic]
fn will_panic() {
    Guess::new(200);
}
