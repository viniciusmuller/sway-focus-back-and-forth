use std::{io::Write, os::unix::net::UnixStream};

use swayipc::Fallible;

const MAGIC_STR: &[u8] = b"please let me alt+tab";

fn main() -> Fallible<()> {
    let socket_path = "/run/user/1000/sway_focus_back_and_forth.sock";
    let mut stream = UnixStream::connect(socket_path)?;
    stream.write_all(MAGIC_STR)?;

    Ok(())
}
