use std::{
    fs::remove_file,
    io::Read,
    os::unix::net::{UnixListener, UnixStream},
    sync::{Arc, Mutex},
    thread,
};

use swayipc::{
    reply::{Event, WindowChange},
    Connection, EventType, Fallible,
};

const MAGIC_STRING: &str = "please let me alt+tab";

fn handle_client(mut stream: UnixStream) -> bool {
    let mut buf = String::new();
    match stream.read_to_string(&mut buf) {
        Ok(_) => buf == MAGIC_STRING,
        Err(_) => false,
    }
}

fn main() -> Fallible<()> {
    let subs = [EventType::Window];

    let last_window_id_mutex: Arc<Mutex<(i64, i64)>> = Arc::new(Mutex::new((-999, -999)));

    let foo_clone = last_window_id_mutex.clone();

    let handle = thread::spawn(move || {
        let conn = Connection::new().unwrap();

        for event in conn.subscribe(&subs).unwrap() {
            if let Ok(event) = event {
                match event {
                    Event::Window(window_event) => {
                        let mut last_windows = foo_clone.lock().unwrap();
                        if window_event.change == WindowChange::Focus {
                            match *last_windows {
                                (-999, -999) => {
                                    last_windows.0 = window_event.container.id;
                                }
                                (n, _) => {
                                    last_windows.0 = window_event.container.id;
                                    last_windows.1 = n;
                                }
                            }
                        }
                    }
                    _ => panic!("only listening to window events"),
                }
            }
        }
    });

    let _handle2 = thread::spawn(move || {
        let socket_path = "/run/user/1000/sway_focus_back_and_forth.sock";
        // If socket file exists, remove it, otherwise we don't care
        remove_file(socket_path);
        let listener = UnixListener::bind(socket_path).unwrap();
        println!("unix socket listening");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let valid_request = thread::spawn(|| handle_client(stream)).join().unwrap();

                    if valid_request {
                        let mut conn2 = Connection::new().unwrap();
                        let (_current, last) = *last_window_id_mutex.lock().unwrap();
                        let command = format!("[con_id={}] focus", last);
                        dbg!(&command);
                        conn2.run_command(command).unwrap();
                    }
                }
                Err(_err) => {
                    /* connection failed */
                    print!("connection failed");
                    break;
                }
            }
        }
    });

    handle.join().unwrap();

    Ok(())
}
