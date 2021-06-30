use std::fs::File;
use std::io::prelude::*;
use bat::PrettyPrinter;
use std::env;

use crate::lib::database::STORAGE_ROOT;

pub fn print(){

    let banner_file = format!("{}/{}/{}", env::var("HOME").unwrap(), STORAGE_ROOT, "banner.ascii");
    let b = std::path::Path::new(&banner_file.clone()).exists();
    if b {
        PrettyPrinter::new()
        .input_file(&banner_file)
        .print().unwrap();
    }
    else{
        println!("🐺 No Banner Found at ~/.julie/banner.ascii");
    }
}


