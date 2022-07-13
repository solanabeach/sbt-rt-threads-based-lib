use serde_json::Value;
use std::collections::{HashMap, hash_map::Entry, BTreeMap};
use crate::transaction_ops::{AccountProfile, DeserializationError};

pub fn process_instruction<'a>(
    tx_accs: &[&'a str],
    tx_hm: &mut BTreeMap<String, AccountProfile>,
    ix: &'a Value,
) -> Result<(), DeserializationError> {

    let pid_ind = ix["programIdIndex"].as_u64().ok_or(DeserializationError {
        msg: "couldn't get prog index ".to_string(),
        ..Default::default()
    })?;

    let program = tx_accs[pid_ind as usize];

    let data = ix["data"].as_str().ok_or(DeserializationError {
        msg: "couldn't get ix data ".to_string(),
        ..Default::default()
    })?;

    let databytes = bs58::decode(data).into_vec().unwrap();

    let acc_inds = ix["accounts"]
        .as_array()
        .ok_or(DeserializationError {
            msg: "couldn't get accoint_ids".to_string(),
            ..Default::default()
        })?
        .iter()
        .map(|id| id.as_u64().unwrap())
        .collect::<Vec<u64>>();

    if let Some(prog_profile) = tx_hm.get_mut(program) {
        prog_profile.ix_mentions += 1;
        prog_profile.is_program   = true;
        prog_profile.num_call_to += 1;
        if databytes.len() > 0 {
            prog_profile.arg_data.num_occurences += 1;
            prog_profile.arg_data.total_length   += databytes.len() as u64;
            // if this (mehtod?)byte has appeared already, increment; otherwise add with count=1;
            match prog_profile.data_first_byte.entry(databytes[0]){
                Entry::Occupied(v) => *v.into_mut() +=1,
                Entry::Vacant(ev) =>{ ev.insert(1);}
            }

        }else{
            prog_profile.arg_data.num_occurences += 1;
            prog_profile.num_zero_len_data +=1
        }
        prog_profile.num_input_accs_ix.push(acc_inds.len() as u8);

    } else {
        panic!("Program account not found in tx_hm. Logic error.");
    }

    for acc_index in acc_inds {
        if let Some(acc) = tx_hm.get_mut(tx_accs[acc_index as usize]) {
            acc.ix_mentions += 1;
        } else {
            panic!("Account not found in tx_hm. Logic error.");
        }
    }

    Ok(())
}