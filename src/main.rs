mod utility;
mod file_manipulator;
use colored::*;
#[path = "person_data.rs"]
mod person_data;

use person_data::Person;
use protobuf::Message;

use std::{
    fs::{File,OpenOptions},
    io::{prelude::*, BufReader,Write},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("employee data file not found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


fn main() {
    let cmd_arguments = utility::command_line::argument::get_arguments();
    
    let error_strings = file_manipulator::guard::get_error_strings(&cmd_arguments);

    if error_strings.is_empty(){

        
        let input_data = lines_from_file(&cmd_arguments.input_data_file_path);
        
        
        for i in 0..input_data.len() {
            let person = Person {
                last_name: "i[0]".into(),
                first_name: "i[1]".into(),
                date_of_birth: "i[2]".into(),
                unknown_fields: Default::default(),
                cached_size: Default::default(),
            };
            let mut buf = vec![];
            person.encode(&mut buf).unwrap();
            let _protobuff: Person = Message::parse_from_bytes(&buf).unwrap();
            println!("{} {:#?} ",buf.len(), _protobuff);
        }

    let data = file::read_file(args[1].as_str()).unwrap();
    let _protobuffer: Person = parse_from_bytes::<Person>(&data).unwrap();

    println!("Last_name: {}", _protobuffer.last_name);
    println!("First_Name: {}", _protobuffer.first_name);
    println!("Date of Birth: {}", _protobuffer.date_of_birth);
    
        
        
    }
    else{
        for error in error_strings{
            eprintln!("{}", error.red());
        }
    }
} 