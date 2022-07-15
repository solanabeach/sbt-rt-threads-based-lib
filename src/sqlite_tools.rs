use std::collections::HashMap;

use rusqlite::{Connection, Result};


#[derive(Debug)]
struct Cat {
    name : String,
    color: String,
}

pub fn insert_data(conn:&Connection)->Result<()>{

    let mut cat_colors = HashMap::new();
    cat_colors.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);
    cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);
    for (color, catnames) in &cat_colors {
        conn.execute(
            "INSERT INTO cat_colors (name) values (?1)",
            &[&color.to_string()],
        )?;
        let last_id: String = conn.last_insert_rowid().to_string();

        for cat in catnames {
            conn.execute(
                "INSERT INTO cats (name, color_id) values (?1, ?2)",
                &[&cat.to_string(), &last_id],
            )?;
        }
    }
    let mut stmt = conn.prepare(
        "SELECT c.name, cc.name from cats c
         INNER JOIN cat_colors cc
         ON cc.id = c.color_id;",
    )?;

    let cats = stmt.query_map([], |row| {
        Ok(Cat {
            name: row.get(0)?,
            color: row.get(1)?,
        })
    })?;

    for cat in cats {
        println!("Found cat {:?}", cat);
    }
    
    Ok(())
}

pub fn sqlite_tools() -> Result<()> {
    let conn = Connection::open("account-profiles1.db")?;

    conn.execute(
        "create table if not exists accounts (
            hash                               TEXT    PRIMARY KEY,
            num_entered_as_signed_rw           integer not null ,
            num_entered_as_signed_r            integer not null ,
            num_entered_as_unsigned_rw         integer not null ,
            num_entered_as_unsigned_r          integer not null ,
            tx_top_mentions                    integer not null ,
            ix_mentions                        integer not null ,
            is_pda                     boolean not     null ,
            is_program                 boolean not     null ,
            number_of_calls_to                 integer not null,
            total_data_length                  integer not null ,
            total_data_occurences              integer not null ,
            num_zero_len_data                  integer not null
         )",
        [])?;

    conn.execute("create table if not exists num_input_accs_ix (
            hash text primary key,
         )", [])?;

    conn.execute("create table if not exists data_first_byte (
            hash text primary key,
        )", [])?;


    insert_data(&conn)?;
    Ok(())
}



fn test_sqlite () -> Result<()> {
    let conn = Connection::open("test1.db")?;

    Ok(())
}
pub fn main(){
    // sqlite_tools().unwrap();
}


#[cfg(test)]
mod tests{

    // #[test]
    // pub fn create 

}



