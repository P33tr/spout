use std::env;
use std::fs::File;
use notify_rust::{Notification, Hint};
use std::time::{SystemTime, UNIX_EPOCH};


fn main() -> Result<(), Box<dyn std::error::Error>> {

    // get the command line arguments
    let args: Vec<String> = env::args().collect();
    dbg!(args);

    
    // let mut file = File::open(path).await.unwrap();
    // let mut interval = time::interval(Duration::from_millis(1000));
    // let mut contents = vec![];
    // let mut position = 0;

    Notification::new()
    .summary("minimal notification")
    .body("This has nothing to do with emails.\nIt should not go away until you acknowledge it.")
    .icon("thunderbird")
    .appname("thunderbird")
    .timeout(0)
    .show()?;






    
    // loop {
    //     contents.truncate(0);
    //     file.seek(SeekFrom::Start(position as u64)).await;
    //     position += file.read_to_end(&mut contents).await.unwrap();
        
    //     /// do_process(contents)
        
    //     interval.tick().await;
    // }

    Ok(())
}

//https://stackoverflow.com/questions/71632833/how-to-continuously-watch-and-read-file-asynchronously-in-rust-using-tokio
