use clap::{Arg , App};
use std::process;
mod api;

fn main() {
    let matches = App::new("RustClip")
        .about("RustClip is a simple CLI tool to copy or cut files in Linux")
        .arg(Arg::new("cut")
            .short('x')
            .help("Copies from source to destination and deletes source")
            .takes_value(true)
            .min_values(2))
        .arg(Arg::new("copy")
            .short('c')
            .help("Copies specified file from source to destination")
            .takes_value(true)
            .min_values(2))
        .arg(Arg::new("file")
            .short('f')
            .help("Copies a directory from source to destination")
            .takes_value(true)
            .min_values(2))
        .get_matches();
    
    if let Some(args) = matches.values_of("cut"){
        let args_values: Vec<&str> = args.collect();
        let source = args_values[0];
        let destination = args_values[1];
        if let Err(e) = api::cliprust::cut(&source, &destination){
           println!("Error cutting file: {}", e);
           process::exit(0);
        }
    }

    if let Some(copy_args) = matches.values_of("copy"){
        let args_values: Vec<&str> = copy_args.collect();
        let source = args_values[0];
        let destination = args_values[1];
       if let Err(e) = api::cliprust::copy(&source, &destination){
           println!("Error copying file: {}", e);
           process::exit(0);
        }   
    }

    
    if let Some(directory) = matches.values_of("file"){
        let directory_values: Vec<&str> = directory.collect();
        let source = directory_values[0];
        let destination = directory_values[1];
       if let Err(e) = api::cliprust::copy_dir_all(&source, &destination){
           println!("Error copying file: {}", e);
           process::exit(0);
        }
        
    }

 
}

