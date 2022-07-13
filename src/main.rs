use std::{
    fs::{read_dir, File},
    io::{self, BufReader, Read},
    path::PathBuf,
    sync::{Arc, RwLock},
    thread::{self, JoinHandle},
};

use itertools::{Itertools, EitherOrBoth};
use std::collections::HashMap;
use std::collections::BTreeMap;

use serde_json::Value;
use transaction_ops::DataFreq;

use crate::transaction_ops::{process_tx, AccountProfile};
pub mod instruction_ops;
pub mod transaction_ops;


pub fn process_block  (serde_block: &Value){

}

fn main() -> io::Result<()> {
    println!("My process id is :{}", std::process::id());


    let global_map:BTreeMap<String, AccountProfile> = BTreeMap::new();

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
            let mut block_hm            = BTreeMap::new();
            for tx in block_parsed["transactions"].as_array_mut().unwrap().iter() {
                let _ = process_tx(&tx["transaction"], &mut block_hm);
            }
            // println!("{:?}", block_hm);
            block_hm
        });
        handles.push(_handle);
    }

    let mut global_map:BTreeMap<String,AccountProfile> = BTreeMap::new();
    while handles.len() > 0 {
        let cur_thread     = handles.pop().unwrap();      // moves it into cur_thread
        let returned_block = cur_thread.join().unwrap();

        global_map = global_map.into_iter().merge_join_by(returned_block, |(key_global, _), (key_local,_)| Ord::cmp(key_global, key_local))
        .map(|kvpair|match kvpair{
            EitherOrBoth::Both(global,local ) =>(global.0,merge_account_profiles(global.1, local.1)),
            EitherOrBoth::Left(global) => global,
            EitherOrBoth::Right(local) => local
        }).collect::<BTreeMap<String,AccountProfile>>()
    }

    println!("{:?}", global_map);
    Ok(())
}


fn merge_account_profiles (mut a: AccountProfile, mut b: AccountProfile)->AccountProfile{
    if (a.is_pda != b.is_pda){println!("Got a problem. PDA mismatch");}
    if (a.is_program != b.is_program){println!("Got a problem. Program mismatch");}
    AccountProfile{
        num_entered_as_signed_rw   : a.num_entered_as_signed_rw   + b.num_entered_as_signed_rw   ,
        num_entered_as_signed_r    : a.num_entered_as_signed_r    + b.num_entered_as_signed_r    ,
        num_entered_as_unsigned_rw : a.num_entered_as_unsigned_rw + b.num_entered_as_unsigned_rw ,
        num_entered_as_unsigned_r  : a.num_entered_as_signed_r    + b.num_entered_as_unsigned_r  ,
        tx_top_mentions            : a.tx_top_mentions            + b.tx_top_mentions            ,
        ix_mentions                : a.ix_mentions                + b.ix_mentions                ,
        num_call_to                : a.num_call_to                + b.num_call_to                ,
        num_zero_len_data          : a.num_zero_len_data          + b.num_zero_len_data          ,
        arg_data         : DataFreq{
            num_occurences : a.arg_data.num_occurences + b.arg_data.num_occurences,
            total_length   : a.arg_data.total_length   + b.arg_data.total_length
        },         

        is_pda     : a.is_pda     || b.is_pda     ,
        is_program : a.is_program || b.is_program ,

        data_first_byte  : (||{ b.data_first_byte.iter()
            .for_each(|(k2,v2)|{
                 a.data_first_byte.entry(*k2).and_modify(|v1|{ *v1 += *v2}).or_insert(*v2);}); 
            a.data_first_byte})(),

        num_input_accs_ix:  (||{ a.num_input_accs_ix.append(&mut b.num_input_accs_ix); a.num_input_accs_ix })(),        
    }
}

pub fn merge_hmaps (mut m1 : HashMap<u8,u64>,m2: &mut HashMap<u8,u64>)->HashMap<u8,u64>{
    m2.iter().for_each(|(k2,val2)|{
        m1.entry(*k2).and_modify(|v1|{ *v1 += *val2}).or_insert(*val2);
    });
    m1
}


