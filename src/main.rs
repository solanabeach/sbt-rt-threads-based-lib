use std::{
    fs::{read_dir, File},
    io::{self, BufReader, Read},
    path::PathBuf,
    sync::{Arc, RwLock},
    thread::{self, JoinHandle}, error::Error,
};

use itertools::{EitherOrBoth, Itertools};
use std::collections::BTreeMap;
use std::collections::HashMap;

use serde_json::Value;
use transaction_ops::DataFreq;

use crate::transaction_ops::{process_tx, AccountProfile};
pub mod instruction_ops;
pub mod transaction_ops;


fn main() -> io::Result<()> {
    println!("My process id is :{}", std::process::id());

    let datapath = "/home/rxz/dev/sb-actix-lib/sample-data";
    println!("Will read from {}", datapath);
    let reader = read_dir(datapath)?
        .map(|readdir| readdir.map(|p| p.path()))
        .collect::<io::Result<Vec<PathBuf>>>()?;

    let strpaths: Vec<String> = reader
        .clone()
        .iter_mut()
        .map(|pb| pb.to_str().unwrap().to_string())
        .collect();
    let paths = Arc::new(RwLock::new(strpaths));

    let mut handles: Vec<_> = vec![];

    for i in 0..2 {
        let innerpaths = Arc::clone(&paths);
        let _handle = thread::spawn(move || {
            let sr                      = innerpaths.read().unwrap();
            let sref: &Vec<String>      = sr.as_ref();
            let to_injest               = &sref[0..2][i];
            println!("Opening block {}", to_injest);
            let mut reader              = BufReader::new(File::open(to_injest).unwrap());
            let mut block               = String::new();
            let _                       = reader.read_to_string(&mut block);
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

    let mut global_map: BTreeMap<String, AccountProfile> = BTreeMap::new();
    while handles.len() > 0 {
        let cur_thread     = handles.pop().unwrap();      // moves it into cur_thread
        let returned_block = cur_thread.join().unwrap();

        // global_map = global_map
        //     .into_iter()
        //     .merge_join_by(returned_block, |(key_global, _), (key_local, _)| {
        //         Ord::cmp(key_global, key_local)
        //     })
        //     .map(|kvpair| match kvpair {
        //         EitherOrBoth::Both(global, local) => {
        //             (global.0, merge_account_profiles(global.1, local.1))
        //         }
        //         EitherOrBoth::Left(global) => global,
        //         EitherOrBoth::Right(local) => local,
        //     })
        //     .collect::<BTreeMap<String, AccountProfile>>()

        global_map = merge_btree_maps(global_map, returned_block);
    }

    Ok(())
}



#[derive(Debug)]
pub struct LogicalError{}

fn merge_account_profiles(mut a: AccountProfile, mut b: AccountProfile) -> Result<AccountProfile, LogicalError> {
    if a.is_pda != b.is_pda {
        println!("Got a problem. PDA mismatch");
        return Err(LogicalError{})
    }

    // if a.is_program != b.is_program {
    //     println!("Got a problem. Program mismatch");
    //     return Err( LogicalError{} )
    // }

    Ok(AccountProfile {
        num_entered_as_signed_rw  : a.num_entered_as_signed_rw + b.num_entered_as_signed_rw,
        num_entered_as_signed_r   : a.num_entered_as_signed_r + b.num_entered_as_signed_r,
        num_entered_as_unsigned_rw: a.num_entered_as_unsigned_rw + b.num_entered_as_unsigned_rw,
        num_entered_as_unsigned_r : a.num_entered_as_signed_r + b.num_entered_as_unsigned_r,
        tx_top_mentions           : a.tx_top_mentions + b.tx_top_mentions,
        ix_mentions               : a.ix_mentions + b.ix_mentions,
        num_call_to               : a.num_call_to + b.num_call_to,
        num_zero_len_data         : a.num_zero_len_data + b.num_zero_len_data,
        arg_data                  : DataFreq {
            num_occurences: a.arg_data.num_occurences + b.arg_data.num_occurences,
            total_length: a.arg_data.total_length + b.arg_data.total_length,
        },

        is_pda    : a.is_pda || b.is_pda,
        is_program: a.is_program || b.is_program,

        data_first_byte: (|| {
            b.data_first_byte.iter().for_each(|(k2, v2)| {
                a.data_first_byte
                    .entry(*k2)
                    .and_modify(|v1| *v1 += *v2)
                    .or_insert(*v2);
            });
            a.data_first_byte
        })(),

        num_input_accs_ix: (|| {
            a.num_input_accs_ix.append(&mut b.num_input_accs_ix);
            a.num_input_accs_ix
        })(),
    })
}

pub fn merge_hmaps(mut m1: HashMap<u8, u64>, m2: &mut HashMap<u8, u64>) -> HashMap<u8, u64> {
    m2.iter().for_each(|(k2, val2)| {
        m1.entry(*k2).and_modify(|v1| *v1 += *val2).or_insert(*val2);
    });
    m1
}


pub fn merge_btree_maps(mut bm1: BTreeMap<String, AccountProfile>, bm2:  BTreeMap<String, AccountProfile>) -> BTreeMap<String, AccountProfile> {
        bm1 = bm1
            .into_iter()
            .merge_join_by(bm2, |(key_global, _), (key_local, _)| {
                Ord::cmp(key_global, key_local)
            })
            .map(|kvpair| match kvpair {
                EitherOrBoth:: Both(global, local) => {
                    let gl2 = global.0.clone();
                    (global.0, merge_account_profiles(global.1, local.1).map_or_else(|_e| {println!("miscalculated program(?): {} ", &gl2); AccountProfile {..Default::default()}}, |v|v))}

                EitherOrBoth:: Left(global) => global,
                EitherOrBoth:: Right(local) => local,
            })
            .collect::<BTreeMap<String, AccountProfile>>();
            bm1
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, BTreeMap};

    use itertools::{Itertools, EitherOrBoth};
    use solana_sdk::blake3::Hash;

    use crate::{transaction_ops::{AccountProfile, DataFreq}, merge_account_profiles};

    #[test]
    fn block_merging() {
        // "4GPN2JK3Ub3ACMPnRkiKe5HRAWPf8bXNStaRNXdUeEUC",
        // "AfEj5hyt4vAauLVQJYJiSnCmBQ2zpwcYefPnsCbqsyzV",
        // "SysvarS1otHashes111111111111111111111111111",
        // "SysvarC1ock11111111111111111111111111111111",
        // "Vote111111111111111111111111111111111111111"


        let addr1 = String::from("AfEj5hyt4vAauLVQJYJiSnCmBQ2zpwcYefPnsCbqsyzV");
        let mut method_invocations_1 =  HashMap::new();
        method_invocations_1.insert(213u8, 1);
        method_invocations_1.insert(0u8, 20);
        method_invocations_1.insert(1u8, 25);
        method_invocations_1.insert(8u8, 100);
        
        
        let acc1 = AccountProfile{
            arg_data: DataFreq{
                num_occurences: 1,
                total_length: 1,
            },
            data_first_byte           : method_invocations_1,
            is_pda                    : false,
            is_program                : false,
            ix_mentions               : 20,
            num_call_to               : 30,
            num_entered_as_signed_r   : 40,
            num_entered_as_signed_rw  : 50,
            num_entered_as_unsigned_r : 50,
            num_entered_as_unsigned_rw: 60,
            num_input_accs_ix         : vec![49, 50, 2 ,3 ,5,],
            num_zero_len_data         : 70,
            tx_top_mentions           : 80
        };

        let addr2 = String::from("AfEj5hyt4vAauLVQJYJiSnCmBQ2zpwcYefPnsCbqsyzV");
        let mut method_invocations_2 =  HashMap::new();
        method_invocations_2.insert(49u8, 111);
        method_invocations_2.insert(0u8, 20);
        method_invocations_2.insert(1u8, 25);
        method_invocations_2.insert(8u8, 100);

        let acc2 = AccountProfile{
            arg_data: DataFreq{
                num_occurences: 2,
                total_length  : 5,
            },
            data_first_byte           : method_invocations_2,
            is_pda                    : false,
            is_program                : false,
            ix_mentions               : 5,
            num_call_to               : 5,
            num_entered_as_signed_r   : 5,
            num_entered_as_signed_rw  : 5,
            num_entered_as_unsigned_r : 5,
            num_entered_as_unsigned_rw: 5,
            num_input_accs_ix         : vec![90,90],
            num_zero_len_data         : 2,
            tx_top_mentions           : 3
        };


        let addr3 = String::from("SysvarC1ock11111111111111111111111111111111");
        let mut method_invocations_3 =  HashMap::new();
        method_invocations_3.insert(7u8, 100);
        method_invocations_3.insert(14u8, 200);
        method_invocations_3.insert(21u8, 300);
        method_invocations_3.insert(28u8, 400);
        let acc3 = AccountProfile{
            arg_data: DataFreq{
                num_occurences: 2,
                total_length  : 2,
            },
            data_first_byte           : method_invocations_3,
            is_pda                    : false,
            is_program                : true,
            ix_mentions               : 111,
            num_call_to               : 111,
            num_entered_as_signed_r   : 111,
            num_entered_as_signed_rw  : 111,
            num_entered_as_unsigned_r : 111,
            num_entered_as_unsigned_rw: 111,
            num_input_accs_ix         : vec![11,11],
            num_zero_len_data         : 11,
            tx_top_mentions           : 11
        };



        let mut global_map:BTreeMap<String, AccountProfile> = BTreeMap::new();
        let mut block1    :BTreeMap<String, AccountProfile> = BTreeMap::new();
        let mut block2    :BTreeMap<String, AccountProfile> = BTreeMap::new();
        let mut block3    :BTreeMap<String, AccountProfile> = BTreeMap::new();


        block1.insert(addr1, acc1);
        block2.insert(addr2, acc2);
        block3.insert(addr3, acc3);



        // let _ = global_map.into_iter()
        //     .merge_join_by(block1, |(key_global, _), (key_local, _)| {
        //         Ord::cmp(key_global, key_local)
        //     })
        //     .map(|kvpair| match kvpair {
        //         EitherOrBoth::Both(global, local) => {
        //             (global.0, merge_account_profiles(global.1, local.1))
        //         }
        //         EitherOrBoth::Left(global) => global,
        //         EitherOrBoth::Right(local) => local,
        //     })
        //     .collect::<BTreeMap<String, AccountProfile>>();


        // assert_eq!()


        // global_map = global_map
        //     .into_iter()
        //     .merge_join_by(returned_block, |(key_global, _), (key_local, _)| {
        //         Ord::cmp(key_global, key_local)
        //     })
        //     .map(|kvpair| match kvpair {
        //         EitherOrBoth::Both(global, local) => {
        //             (global.0, merge_account_profiles(global.1, local.1))
        //         }
        //         EitherOrBoth::Left(global) => global,
        //         EitherOrBoth::Right(local) => local,
        //     })
        //     .collect::<BTreeMap<String, AccountProfile>>()


    }


    #[test]
    fn merge_hashmaps() {
        let mut hm1 = HashMap::new();
        hm1.insert(2, 3);
        hm1.insert(4, 20);
        hm1.insert(5, 100);
        let mut hm2 = HashMap::new();
        hm2.insert(10, 200);
        hm2.insert(5, 1);

        let mut hmcontrol =HashMap::new();
        hmcontrol.insert(2, 3);
        hmcontrol.insert(4, 20);
        hmcontrol.insert(5,101);
        hmcontrol.insert(10,200);
        assert_eq!(
            crate::merge_hmaps(hm1,&mut hm2),
            hmcontrol
        )
    }
}
