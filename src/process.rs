use libc::funcs::posix88::unistd;
use std::io;

/// getpid is safe to wrap function
fn get_pid() -> i32 {
    unsafe {
        return unistd::getpid() as i32;
    }
}

/// create a file with 'path' and insert the pid into it
pub fn set_pid_file(path: Option<&str>) {
    if path.is_some() {
        let pid = get_pid();
        let mut pid_file = io::File::create(&Path::new(path.unwrap()));
        match pid_file.write(format!("{}", pid).as_bytes()) {
            Err(e) => {
                println!("Warning: could not write pid file: {}", e);
            },
            _ => {}
        }
    }
}

/// remove pid file
pub fn remove_pid_file(path: Option<&str>) {
    if path.is_some() {
        match io::fs::unlink(&Path::new(path.unwrap())) {
            Err(e) => {
                println!("Warning: could not delete pid file: {}", e);
            },
            _ => {}
        }
    }
}
