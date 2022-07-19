use std::collections::HashMap;

#[path ="./block_ops/mod.rs"]
mod block_ops;
#[path ="./databases/mod.rs"]
mod databases;


pub fn main(){
    use databases::sqlite_tools::*;
    use block_ops::{*};
    let conn = create_tables().unwrap();

    let addr2                    = String::from("dafEj5hyt4vAauLVQJYJiSnCmBQ2zpwcYefPnsCbqsyzV");
    let method_invocations_2 =[(49u8, 111), (0u8, 20), (1u8, 25),(8u8, 100)].into_iter().collect::<HashMap<u8,u64>>(); 
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
        num_input_accs_ix         : vec![(90,2)].into_iter().collect:: <HashMap<u8, u64>>(),
        num_zero_len_data         : 2,
        tx_top_mentions           : 3
    };


    // insert_account(&conn, &addr2, &acc2).unwrap();
    // enter_first_byte_data(&conn, &addr2, &acc2).unwrap();
    // add_column_if_not_exists(&conn, "accounts", "is_pda");


    println!("Ran main");
    

}

#[cfg(test)]
mod tests{

}




