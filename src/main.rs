use std::{thread::{self, JoinHandle}, fs::read_dir, io, path::PathBuf, sync::{RwLock, Arc}};

fn main() ->io::Result<()>{


    let datapath = "/home/rxz/dev/sb-actix-lib/sample-data";

    println!("Will read from {}", datapath);

    let mut reader = read_dir(datapath)?
    .map(|readdir| readdir.map(|p| p.path())).collect::<io::Result<Vec<PathBuf>>>()?;

    let strpaths: Vec<String> = reader.clone().iter_mut().map(|pb| pb.to_str().unwrap().to_string()).collect();

    pub struct DataPaths{
        paths: Arc<RwLock<Vec<String>>>,
    }

    let dps =DataPaths{paths: Arc::new(RwLock::new(strpaths))};

    let mut handles:Vec<JoinHandle<()>> =vec![];
    for i in 0..200 {
        let innerpaths = dps.paths.clone();
        let _handle = thread::spawn(move||{
            let sr                = innerpaths.read().unwrap();
            let sref:&Vec<String> = sr.as_ref();
            // let filehandles       = &sref[i*100..(i*100)+100];

            // println!("Thread {} got #{} handles.", i, filehandles.len());
            let mut vx = 0;
            for  i in 0..u64::MAX{
               vx+=i
            };
            // thread::sleep(std::time::Duration::from_secs(5));
        });
        handles.push(_handle);
    }

    while handles.len() > 0 {
        let cur_thread = handles.remove(0); // moves it into cur_thread
        cur_thread.join().unwrap();
}







    

    println!("Hello, world!");
    Ok(())
}
