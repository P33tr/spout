#![allow(non_snake_case)]
use std::env;
use std::fs::File;
use notify_rust::{Notification, Hint};
use std::time::{SystemTime, UNIX_EPOCH};
use dioxus::prelude::*;


fn main() -> Result<(), Box<dyn std::error::Error>> {

    // get the command line arguments
    let args: Vec<String> = env::args().collect();
    
    //let args_as_string = String::from(args[1]);
    let args_as_string = "xxx";
    // let mut file = File::open(path).await.unwrap();
    // let mut interval = time::interval(Duration::from_millis(1000));
    // let mut contents = vec![];
    // let mut position = 0;

    Notification::new()
    .summary("Args notification")
    .body(&args_as_string)
    .icon("thunderbird")
    .appname("thunderbird")
    .timeout(0)
    .show()?;

    Notification::new()
    .summary("minimal notification")
    .body("This has nothing to do with emails.\nIt should not go away until you acknowledge it.")
    .icon("thunderbird")
    .appname("thunderbird")
    .timeout(0)
    .show()?;



    dioxus_desktop::launch(App);


    
    // loop {
    //     contents.truncate(0);
    //     file.seek(SeekFrom::Start(position as u64)).await;
    //     position += file.read_to_end(&mut contents).await.unwrap();
        
    //     /// do_process(contents)
        
    //     interval.tick().await;
    // }

    Ok(())
}

// define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {

    let mut count_a = use_state(cx, || 0);

    cx.render(rsx! {
        div {
            "Hello, peter"
        }
        h1 { "Counter_a: {count_a}" }
        div {
            button { onclick: move |_| count_a += 1, "a++" }
        }
    })
}
//https://stackoverflow.com/questions/71632833/how-to-continuously-watch-and-read-file-asynchronously-in-rust-using-tokio
//https://dioxuslabs.com/learn/0.4/getting_started/wasm
