use std::{
    fs::{read_dir, File},
    io::{self, BufReader, Read},
    path::PathBuf,
    sync::{Arc, RwLock},
    thread::{self, JoinHandle},
};
use std::collections::HashMap;

use serde_json::Value;

use crate::transaction_ops::{process_tx, AccountProfile};
pub mod instruction_ops;
pub mod transaction_ops;


pub fn process_block  (serde_block: &Value){

}

fn main() -> io::Result<()> {
    println!("My process id is :{}", std::process::id());


    let global_map:HashMap<String, AccountProfile> = HashMap::new();

    let datapath = "/home/rxz/dev/sb-actix-lib/sample-data";
    println!("Will read from {}", datapath);
    let mut reader = read_dir(datapath)?
        .map(|readdir| readdir.map(|p| p.path()))
        .collect::<io::Result<Vec<PathBuf>>>()?;

    let strpaths: Vec<String> = reader.clone().iter_mut().map(|pb| pb.to_str().unwrap().to_string()).collect();
    let paths                 = Arc::new(RwLock::new(strpaths));

    let mut handles: Vec<_> = vec![];

    for i in 0..2 {
        let innerpaths = Arc::clone(&paths);
        let _handle = thread::spawn(move || {
            let sr = innerpaths.read().unwrap();
            let sref: &Vec<String> = sr.as_ref();
            let firstblock = &sref[0..2][i];
            let mut reader = BufReader::new(File::open(firstblock).unwrap());
            let mut block  = String::new();
            let _ = reader.read_to_string(&mut block);
            let mut block_parsed: Value = serde_json::from_str(&block).unwrap();
            let mut block_hm            = HashMap::new();
            for tx in block_parsed["transactions"].as_array_mut().unwrap().iter() {
                let _ = process_tx(&tx["transaction"], &mut block_hm);
            }
            // println!("{:?}", block_hm);
            block_hm
        });
        handles.push(_handle);
    }

    while handles.len() > 0 {
        let cur_thread = handles.pop().unwrap(); // moves it into cur_thread
        let returned_block = cur_thread.join().unwrap();





    }
    println!("Done");
    Ok(())
}
