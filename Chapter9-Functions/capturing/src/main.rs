fn main() {
    // `Vec` has non-copy semantics.
    let mut haystack = vec![1, 2, 3];

    let mut contains = |needle| { 
        let ret = haystack.contains(needle);
        haystack.retain(|x: &i32| x != needle);
        ret
    };
    
    println!("{}", contains(&1));
    println!("{}", contains(&3));
    println!("{}", contains(&2));

    println!("There're {} elements in vec: {:?}", haystack.len(), haystack);
    // ^ Uncommenting above line will result in compile-time error
    // because borrow checker doesn't allow re-using variable after it
    // has been moved.
    
    // Removing `move` from closure's signature will cause closure
    // to borrow _haystack_ variable immutably, hence _haystack_ is still
    // available and uncommenting above line will not cause an error.
}

