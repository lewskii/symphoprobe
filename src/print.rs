use std::fmt::{Debug, Display};

/// Directly print a value and its name.
pub fn display<T: Display>(name: &str, thing: T) {
    println!("{}: {}", name, thing)
}

/// Print the name and content of an Option, or "none" if appropriate.
/// 
/// Uses the Display representation.
pub fn display_option<T: Display>(name: &str, option: Option<T>) {
    match option {
        Some(thing) => println!("{}: {}", name, thing),
        None => println!("{}: none", name)
    }
}

/// Print the name and content of an Option, or "none" if appropriate.
/// 
/// Uses the Debug representation.
pub fn debug_option<T: Debug>(name: &str, option: Option<T>) {
    match option {
        Some(thing) => println!("{}: {:?}", name, thing),
        None => println!("{}: none", name)
    }
}
