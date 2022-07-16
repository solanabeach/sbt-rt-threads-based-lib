use clap::Parser;
use rusqlite::{Connection, Result};
use crate::block_ops::{*};

pub fn insert_account(conn:&Connection, address_hash:&str, accprofile: &AccountProfile)->Result<()>{

    conn.execute(
        "INSERT INTO accounts (address,num_call_to) values (?1,?2)",
        &[address_hash, &accprofile.num_call_to.to_string() ],
    )?;

    
    Ok(())
}

pub fn add_column_if_not_exists(colname: &str, table:&str){

}

pub fn create_tables() -> Result<()> {
    let conn = Connection::open("test1.db")?;
    conn.execute(
        "create table if not exists accounts (
            address     TEXT    PRIMARY KEY,
            num_call_to integer not     null
         )",
        // "create table if not exists accounts (
        //     address                               TEXT    PRIMARY KEY,
        //     num_entered_as_signed_rw           integer not null ,
        //     num_entered_as_signed_r            integer not null ,
        //     num_entered_as_unsigned_rw         integer not null ,
        //     num_entered_as_unsigned_r          integer not null ,
        //     tx_top_mentions                    integer not null ,
        //     ix_mentions                        integer not null ,
        //     is_pda                     boolean not     null ,
        //     is_program                 boolean not     null ,
        //     number_of_calls_to                 integer not null,
        //     total_data_length                  integer not null ,
        //     total_data_occurences              integer not null ,
        //     num_zero_len_data                  integer not null
        //  )",
        [])?;

    conn.execute("create table if not exists num_input_accs_ix (
            address text primary key,
            FOREIGN KEY(address) REFERENCES accounts(address)
         )", [])?;

    conn.execute("create table if not exists data_first_byte (
            address text primary key,
            FOREIGN KEY(address) REFERENCES accounts(address)
        )", [])?;


    // insert_data(&conn)?;
    Ok(())
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
