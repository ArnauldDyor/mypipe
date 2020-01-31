extern crate clap;
use clap::{Arg, App};
use std::process::Command;

fn main() {
    let matches = App::new("mypipe").version("1.0").author("Arnauld BIAM")
    .arg(Arg::with_name("input")
     .long("in")
     .takes_value(true)
     .required(true))
     .arg(Arg::with_name("output")
     .long("out")
     .takes_value(true)
     .required(true))
     .get_matches();



    let argument1 = matches.value_of("input").unwrap();
    let argument2 = matches.value_of("output").unwrap();
 
    let argument1_ouput = Command::new(argument1.to_string())
                         .output()
                         .expect("keine Ausfuehrung");
 
 
    let argument2_output = Command::new(argument2.to_string())
                         .arg(String::from_utf8_lossy(&argument1_ouput.stdout).to_string())
                         .output()
                         .expect("keine Ausfuehrung");
 
 
     println!("{}", String::from_utf8_lossy(&argument2_output.stdout).to_string());
 }