fn main() {
    // We can reference a part of a collection using 'slices'

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);

    // This is helpful because it allows us to ensure at compile time,
    // that the data is not mutated after we have done some computation 
    // that we will later use.

    let length = s.len();
    let word_1 = first_word(&s[0..length]);
    let word_2 = first_word(&s[..]);// if starting at index 0, we can ommit the 0. Same for last index.
    let word_3 = first_word(&s); 

    // The compiler will not allow this call to s.clear, since that would be mutating 
    // the data that is already borrowed by the immutable references/slices.
    //s.clear();

    println!("{} {} {}", word_1, word_2, word_3);

    // We can take slices of other collections
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}