use std::fmt::{Debug, Display};
use symphonia::core::units::Time;

/// Directly print a value with a label.
/// 
/// Uses the Display representation.
pub fn display<T: Display>(label: &str, thing: &T) {
    println!("{}: {}", label, thing)
}

/// Directly print a value with a label.
/// 
/// Uses the Debug representation.
pub fn debug<T: Debug>(label: &str, thing: &T) {
    println!("{}: {:?}", label, thing)
}

/// Print the content of an Option with a label, or "none" if appropriate.
/// 
/// Uses the Display representation.
pub fn display_option<T: Display>(label: &str, option: &Option<T>) {
    match option {
        Some(thing) => println!("{}: {}", label, thing),
        None => println!("{}: none", label)
    }
}

/// Print the content of an Option with a label, or "none" if appropriate.
/// 
/// Uses the Debug representation.
pub fn debug_option<T: Debug>(label: &str, option: &Option<T>) {
    match option {
        Some(thing) => println!("{}: {:?}", label, thing),
        None => println!("{}: none", label)
    }
}

/// Print a human-friendly representation of an Option<Time> with a label.
pub fn print_time_option(label: &str, time_opt: &Option<Time>) {
    match time_opt {
        Some(time) => {
            let seconds = time.seconds.to_string();
            // strip the leading 0
            // this way we can just append the decimal part to the int part
            let frac = time.frac.to_string().split_off(1);
            println!("{}: {}{}", label, seconds, frac);
        }
        None => {
            println!("{}: none", label);
        }
    }
}
