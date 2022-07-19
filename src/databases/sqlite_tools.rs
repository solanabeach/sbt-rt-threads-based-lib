use clap::Parser;
use rusqlite::{Connection, Result, named_params, ToSql, params, OptionalExtension};
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

pub fn upsert_acc(conn:&Connection, addr:&str,acc:&AccountProfile)->Result<()>{
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
            :num_zero_len_data)
        ON CONFLICT (address) DO UPDATE SET
            num_entered_as_signed_rw  =   num_entered_as_signed_rw   + :num_entered_as_signed_rw  ,
            num_entered_as_signed_r   =   num_entered_as_signed_r    + :num_entered_as_signed_r   ,
            num_entered_as_unsigned_rw=   num_entered_as_unsigned_rw + :num_entered_as_unsigned_rw,
            num_entered_as_unsigned_r =   num_entered_as_unsigned_r  + :num_entered_as_unsigned_r ,
            tx_top_mentions           =   tx_top_mentions            + :tx_top_mentions           ,
            ix_mentions               =   ix_mentions                + :ix_mentions               ,
            is_pda                    =   is_pda                     + :is_pda                    ,
            num_call_to               =   num_call_to                + :num_call_to               ,
            is_program                =   is_program                 + :is_program                ,
            total_data_length         =   total_data_length          + :total_data_length         ,
            total_data_occurences     =   total_data_occurences      + :total_data_occurences     ,
            num_zero_len_data         =   num_zero_len_data          + :num_zero_len_data         
        ",
        named_params! {
            ":address"                    :addr,
            ":num_entered_as_signed_rw"   :acc.num_entered_as_signed_rw  ,
            ":num_entered_as_signed_r"    :acc.num_entered_as_signed_r   ,
            ":num_entered_as_unsigned_rw" :acc.num_entered_as_unsigned_rw,
            ":num_entered_as_unsigned_r"  :acc.num_entered_as_unsigned_r ,
            ":tx_top_mentions"            :acc.tx_top_mentions           ,
            ":ix_mentions"                :acc.ix_mentions               ,
            ":is_pda"                     :acc.is_pda                    ,
            ":num_call_to"                :acc.num_call_to               ,
            ":is_program"                 :acc.is_program                ,
            ":total_data_length"          :acc.arg_data.total_length     ,
            ":total_data_occurences"      :acc.arg_data.num_occurences   ,
            ":num_zero_len_data"          :acc.num_zero_len_data         ,
        },
    )?;
    Ok(())
}



pub fn add_column_if_not_exists(conn:&Connection, table:&str, colname: &str)->Result<(), rusqlite::Error>{
    let mut stmt = conn.prepare(&format!("SELECT COUNT(*) AS CNTREC FROM pragma_table_info('{}') WHERE name='{}'", table, colname))?;
    let exists   = {
        let mut rows = stmt.query([]).unwrap();
        let x = rows.next().map(|r| r.unwrap().get_ref_unwrap(0).as_i64()).unwrap().unwrap();
        x > 0
    };
    println!("Column \"{}\" {} in the table \"{}\".", colname,{if exists{"EXISTS"}else {"DNE"}}, table);
    if !exists {
        let mut prepared = conn.prepare(&format!("ALTER TABLE {} ADD column {} int DEFAULT 0;", table, colname))?;
        let _ = prepared.execute([])?;
        println!("Appended colum \"{}\" in to tablel \"{}\"", colname, table);
    }


    // let mut rows     = stmt.query(named_params! {
    //     ":tablename": "accounts",
    //     ":colname"  : "ix_mentions",
    // }).unwrap();

    // let y = rows   
    // .mapped(|r| Ok({let x:Result<_>  = r.get_ref(0);println!("{:?}", x);
    // }));


    // let y = rows   
    // .mapped(|r| Ok({let x:Result<_>  = r.get_ref(0);println!("{:?}", x);
    // }));


    // for i in y{
    //     println!("{:?}", i);
    // }




    // let names = Vec::new();
    // while let Some(row) = rows.next()?{
    //     names.push(row.get(0)?);
    // }
    // rows.next().unwrap().get_checked(0)?;
    
    Ok(())

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
            address    text    primary key
        )", [])?;
    // conn.execute("create table if not exists data_first_byte (
    //         address    text    primary key,
    //         // first_byte integer not     null,
    //         // num_calls  integer not     null
    //     )", [])?;

    // conn.execute("create table if not exists num_input_accs_ix (
    //         address text primary key, 1
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
