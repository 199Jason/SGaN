extern crate nalgebra as na;
extern crate dialoguer;

use dialoguer::{Select};

pub mod constants;
pub use constants::*;

pub mod functions;
pub use functions::*;

fn main() {
    
    let choices = ["Lagrange points", "Periodic orbit", "Exit"];
    let selection = Select::new()
        .with_prompt("Choose an option")
        .items(&choices)
        .default(0) 
        .interact()
        .unwrap();

    match selection {
        0 => {
            // First part: Lagrange points
            let data = Data {mu: constants::MU};
            let lp = lagrange_points(&data);
            println!("L1: {:?}", lp.l1);
            println!("L2: {:?}", lp.l2);
            println!("L3: {:?}", lp.l3);
            println!("L4: {:?}", lp.l4);
            println!("L5: {:?}", lp.l5);
            let _plots = plot_lagrange_points(&lp, constants::MU);
        },
        1 => {
            println!("WIP...");
            return;
        },
        2 => {
            // Exit
            println!("WIP...");
            return;
        },
        _ => unreachable!(),
    }
}
