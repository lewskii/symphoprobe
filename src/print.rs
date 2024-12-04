use std::fmt::{Debug, Display};
use symphonia::core::units::Time;

/// Directly print a value and its name.
/// 
/// Uses the Display representation.
pub fn display<T: Display>(name: &str, thing: &T) {
    println!("{}: {}", name, thing)
}

/// Directly print a value and its name.
/// 
/// Uses the Debug representation.
pub fn debug<T: Debug>(name: &str, thing: &T) {
    println!("{}: {:?}", name, thing)
}

/// Print the name and content of an Option, or "none" if appropriate.
/// 
/// Uses the Display representation.
pub fn display_option<T: Display>(name: &str, option: &Option<T>) {
    match option {
        Some(thing) => println!("{}: {}", name, thing),
        None => println!("{}: none", name)
    }
}

/// Print the name and content of an Option, or "none" if appropriate.
/// 
/// Uses the Debug representation.
pub fn debug_option<T: Debug>(name: &str, option: &Option<T>) {
    match option {
        Some(thing) => println!("{}: {:?}", name, thing),
        None => println!("{}: none", name)
    }
}

/// Print a human-friendly representation of an Option<Time>.
pub fn print_time_option(name: &str, time_opt: &Option<Time>) {
    match time_opt {
        Some(time) => {
            let seconds = time.seconds.to_string();
            // strip the leading 0
            // this way we can just append the decimal part to the int part
            let frac = time.frac.to_string().split_off(1);
            println!("{}: {}{}", name, seconds, frac);
        }
        None => {
            println!("{}: none", name);
        }
    }
}
