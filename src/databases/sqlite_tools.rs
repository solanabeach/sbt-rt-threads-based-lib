use clap::Parser;
use rusqlite::{Connection, Result, named_params, ToSql};
use crate::block_ops::{*};

pub fn insert_account(conn:&Connection, address_hash:&str, accprofile: &AccountProfile)->Result<()>{
    conn.execute(
        "INSERT INTO accounts (
            address                   ,
            num_entered_as_signed_rw  ,
            num_entered_as_signed_r   ,
            num_entered_as_unsigned_rw,
            num_entered_as_unsigned_r ,
            tx_top_mentions           ,
            ix_mentions               ,
            is_pda                    ,
            num_call_to               ,
            is_program                ,
            total_data_length         ,
            total_data_occurences     ,
            num_zero_len_data        
        ) values (
            :address                   ,
            :num_entered_as_signed_rw  ,
            :num_entered_as_signed_r   ,
            :num_entered_as_unsigned_rw,
            :num_entered_as_unsigned_r ,
            :tx_top_mentions           ,
            :ix_mentions               ,
            :is_pda                    ,
            :num_call_to               ,
            :is_program                ,
            :total_data_length         ,
            :total_data_occurences     ,
            :num_zero_len_data        
        )",
        named_params! {
            ":address"                    :address_hash                            ,
            ":num_entered_as_signed_rw"   :accprofile.num_entered_as_signed_rw  ,
            ":num_entered_as_signed_r"    :accprofile.num_entered_as_signed_r   ,
            ":num_entered_as_unsigned_rw" :accprofile.num_entered_as_unsigned_rw,
            ":num_entered_as_unsigned_r"  :accprofile.num_entered_as_unsigned_r ,
            ":tx_top_mentions"            :accprofile.tx_top_mentions           ,
            ":ix_mentions"                :accprofile.ix_mentions               ,
            ":is_pda"                     :accprofile.is_pda                    ,
            ":num_call_to"                :accprofile.num_call_to               ,
            ":is_program"                 :accprofile.is_program                ,
            ":total_data_length"          :accprofile.arg_data.total_length     ,
            ":total_data_occurences"      :accprofile.arg_data.num_occurences   ,
            ":num_zero_len_data"          :accprofile.num_zero_len_data         ,
        },
    )?;
    println!("Inserted account {}", address_hash);

    Ok(())
}

pub fn enter_first_byte_data(conn:&Connection, address_hash: &str,accprofile:&AccountProfile)->Result<()>{
        for (key, value) in accprofile.data_first_byte.iter(){
        println!("adding {} : {}", key, value);
        // TODO : Add the logic for adding a new column in the case that the byte has not figured yet.

        conn.execute(
            "INSERT INTO data_first_byte (
                address,
                first_byte,
                num_calls
            ) values (
                :address,
                :first_byte,
                :num_calls
            )",
            named_params! {
                ":address"   : address_hash,
                ":first_byte": key,
                ":num_calls" : value,
            },
        )?;

        println!("Inserted data_first_byte {}", key);
    }

    Ok(())

}




pub fn add_column_if_not_exists(conn:&Connection,colname: u8, table:&str){

}

pub fn create_tables() -> Result<Connection> {
    let conn = Connection::open("test1.db")?;
    conn.execute(
        "create table if not exists accounts (
            address    TEXT    PRIMARY KEY,
            num_entered_as_signed_rw   integer not null ,
            num_entered_as_signed_r    integer not null ,
            num_entered_as_unsigned_rw integer not null ,
            num_entered_as_unsigned_r  integer not null ,
            tx_top_mentions            integer not null ,
            ix_mentions                integer not null ,
            is_pda                     boolean not null ,
            num_call_to                integer not null,
            is_program                 boolean not null ,
            total_data_length          integer not null ,
            total_data_occurences      integer not null ,
            num_zero_len_data          integer not null
         )",
        [])?;

    conn.execute("create table if not exists data_first_byte (
            address    text    primary key,
            first_byte integer not     null,
            num_calls  integer not     null
        )", [])?;

    // conn.execute("create table if not exists num_input_accs_ix (
    //         address text primary key,
    //         FOREIGN KEY(address) REFERENCES accounts(address)
    //      )", [])?;



    // insert_data(&conn)?;
    Ok(conn)
}






#[derive(Parser)]
struct Cli {
    #[clap(long, value_parser)]
    two: String,
    #[clap(long, value_parser)]
    one: String,
    #[clap(value_parser)]
    name: Option<String>,
}
