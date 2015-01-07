extern crate libc;
extern crate getopts;

use libc::funcs::posix88::unistd;

fn get_settings() {

}

fn main() {
    println!("process PID: {}", get_pid());
}

fn get_pid() -> i32 {
    unsafe {
        return unistd::getpid() as i32;
    }
}

#[test]
fn test_pid() {
    unsafe {
        let pid = unistd::getpid() as i32;
        println!("Process PID: {}", pid);
    }
}
