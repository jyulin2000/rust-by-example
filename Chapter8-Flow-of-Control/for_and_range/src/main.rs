use std::fmt::{Display, Formatter, Result};

// TODO: Figure out what this 'a and '_ business is.
// It's something to do with the lifetime of the elements of the vector and guaranteeing that
// they'll still be alive while the vector containing them is alive
struct NameVec<'a>(Vec<&'a str>);

impl Display for NameVec<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut str_out = String::from("[ ");
                
        for name in self.0.iter() {
            str_out.push_str(name);
            str_out.push_str(", ");
        }
        
        write!(f, "{}]", str_out)
    }
}

// Using .iter_mut
// Mutably borrows each element, so the elements can be modified in place
fn main() {
    let mut names = vec!["Alice".to_string(), "Bob".to_string(), "Carl".to_string(), "Dave".to_string()];
    let suffix = "'s bastard child";

    println!("{:?}", names);    
    for name in names.iter_mut() {
        //let new_name = format!("{}{}", name, suffix);
 
        (*name).push_str(suffix);       
        /*
        match name {
            "Carl" => println!("Fuck you, Carl."),
            _ => println!("Hi {} :)", name)
        }*/
    }
    
    println!("names: {:?}", names);
}

// Using .into_iter
// This consumes the collection, providing the exact data on each iteration
// Once the collection is consumed it can't be reused and has been consumed by the loop
/*
fn main() {
    let names = vec!["Alice", "Bob", "Carl", "Dave"];
    
    for name in names.into_iter() {
        match name {
            "Carl" => println!("Fuck you, Carl."),
            _ => println!("Hi {} :)", name)
        }
    }
    
    // These lines won't work because names has been consumed already
    println!("names: {:?}", names);
    println!("names: {}", NameVec(names));
}
*/

// Using .iter
// This borrows each element of the collection through the iteration
// The collection is untouched and can be reused after the loop
// Basically we're looping through references to each element, not the elements themselves
/*
fn main() {
    let names = vec!["Alice", "Bob", "Carl", "Dave"];
    
    for name in names.iter() {
        match name {
            &"Carl" => println!("Fuck you, Carl."),
            _ => println!("Hi {} :)", name)
        }
    }
    
    println!("names: {:?}", names);
    println!("names: {}", NameVec(names));
}
*/
