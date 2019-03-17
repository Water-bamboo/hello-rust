
use std::sync::{Arc, RwLock, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::borrow::Cow::{self, Borrowed, Owned};
use std::sync::mpsc;
use std::sync::mpsc::*;
use std::sync::*;
//use std::string::String;

//#![feature(const_fn)]
static mut DATA: Option<Arc<Mutex<String>>> = None;//Arc::new(Mutex::new(String::from("")));
//static mut data: Arc<Mutex<String>> = initData();
//static mut (tx: Sender!<i32>, rx: Receiver!<i32>) = channel!();
static mut CHANNEL_AB: (Option<Sender<String>>, Option<Receiver<String>>) = (None, None);//initChannel();
//static mut commandArray: Vec<String> = vec![String::new()];
static mut INITIALIZED: bool = false;


//#![feature(const_fn)]
fn initChannel() {
    let (tx, rx) = channel();
    unsafe {
        CHANNEL_AB.0 = Some(tx);
//        CHANNEL_AB.0 = Some(Mutex::new(tx));
//        CHANNEL_AB.1 = Some(Mutex::new(rx));
        CHANNEL_AB.1 = Some(rx);
    }
}

fn initData() {
    let dt = Arc::new(Mutex::new(String::from("")));
    unsafe {
        DATA = Some(dt);
    }
}

//static mut t: ;

//pub mod libApi {
pub extern fn do_command(
    command: &str,
    out_is_safe: &mut bool
) {
    println!("main>>>>!");
    initChannel();
    initData();

    unsafe {
        println!("thread threadInitialized={}", INITIALIZED);
        if INITIALIZED == true {
            println!("thread threadInitialized=true 1111");
            let tx = CHANNEL_AB.0.as_ref().unwrap();//.clone();
            println!("thread threadInitialized=true 2222:{}", command.to_string());
            let dataR = DATA.unwrap();
            let mut dataUn = dataR.lock().unwrap();//.clone();
            *dataUn = command.to_string();

            println!("thread threadInitialized=true 4444:{}", dataUn);
            tx.send(command.to_string());
            println!("thread threadInitialized=true 33333");
            return;
        }
    }

    println!("thread is empty>>>");

    let handle = thread::spawn(move || {
        println!("thread:spawning");
        unsafe {
            INITIALIZED = true;
        }

//        let rx: &Receiver<String> = CHANNEL_AB.1.as_ref().unwrap();//.clone();

        loop {
            println!("thread: get rx");

            println!("thread: rx receive...");
            let mut commandLine: String;
            unsafe {
//                let command2 =
                    CHANNEL_AB.1.as_ref().unwrap().recv();//.unwrap();
                commandLine = DATA.unwrap().lock().unwrap().to_string();
            }

            if (commandLine.len() == 0) {
                println!("thread. read dataUn={}", commandLine);
                continue;
            }

            println!("read a command line={}", commandLine);

            if commandLine == "exit".to_string() {
                break;
            }

            println!("do command work");
            //do command work >>
        }
        println!("--->>exit<<-----");
    });

//    handle.join().unwrap();
    //    thread.start();
}

//}
