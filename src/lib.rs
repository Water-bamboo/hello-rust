
use std::sync::{Arc, RwLock, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::borrow::Cow::{self, Borrowed, Owned};
use std::sync::mpsc;
use std::sync::mpsc::*;
use std::sync::*;
//use std::string::String;


//static mut data: Arc<Mutex<String>> = Arc::new(Mutex::new(String::from("")));
//static mut data: Arc<Mutex<String>> = initData();
//static mut (tx: Sender!<i32>, rx: Receiver!<i32>) = channel!();
static mut channelAB: (Option<Sender<String>>, Option<Receiver<String>>) = (None, None);//initChannel();
//static mut commandArray: Vec<String> = vec![String::new()];
static mut threadInitialized: bool = false;
//fn init() {
//    static mut RESULT_SENDER: Option<Sender> = None;
//    static mut RESULT_RECEIVER: Option<Receiver> = None;

//    unsafe {
//        let (tx, rx) = channel();
//        RESULT_SENDER = tx;//Some(Mutex::new(tx));
//        let tx = RESULT_SENDER.as_ref().unwrap().lock().unwrap().clone();

//        RESULT_RECEIVER = rx;//Some(Mutex::new(rx));
//        let rx = RESULT_RECEIVER.as_ref().unwrap().lock().unwrap().clone();
//    }
//}
//let (dataThread, tx) = (data.clone(), tx.clone());
//let mainThreadClone = data.clone();

//const fn initData() -> Arc<Mutex<String>> {
//    return Arc::new(Mutex::new(String::from("")));
//}

//#![feature(const_fn)]
fn initChannel() {
    let (tx, rx) = channel();
    unsafe {
        channelAB.0 = Some(tx);
        channelAB.1 = Some(rx);
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

    unsafe {
        println!("thread threadInitialized={}", threadInitialized);
        if threadInitialized == true {
            let tx = channelAB.0.as_ref().unwrap().clone();
            tx.send(command.to_string());

            return;
        }
    }

    println!("thread is empty>>>");

    let handle = thread::spawn(move || {
        println!("thread:spawning");
        unsafe {
            threadInitialized = true;
        }

        loop {
            println!("thread: get rx");
            let mut rx: &Receiver<String>;
            unsafe {
                rx = channelAB.1.as_ref().unwrap();
            }
            println!("thread: rx receive...");
            let commandWrap = rx.recv();

            println!("thread: rx received: {}", commandWrap);
            let command = commandWrap.unwrap();

            println!("thread. read dataUn={}", command);

            if (command.len() == 0) {
                continue;
            }

            println!("read a command line={}", command);

            if command == "exit".to_string() {
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
