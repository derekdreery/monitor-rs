extern crate libc;

use libc::funcs::posix88::unistd;

fn main() {
}

#[test]
fn test_pid() {
    unsafe {
        let pid = unistd::getpid() as i32;
        println!("Process PID: {}", pid);
    }
}
