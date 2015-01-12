
use std::default::Default;
use std::os;
use std::io;
use getopts;
use toml;

pub struct Settings {
    pid_file_path: Option<String>,
    config_file_path: Option<String>
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            pid_file_path: None,
            config_file_path: None
        }
    }
}

pub fn get_settings() -> Option<Settings> {
    let mut settings: Settings = Default::default();
    let args: Vec<String> = os::args();
    let program = args[0].clone();
    let opts = &[
        getopts::optopt("p", "pid_file", "set location for pid file", "PID"),
        getopts::optopt("c", "config_file", "set location of config file", "CFG"),
        getopts::optflag("h", "help", "print this help message and exit")
    ];
    let cmd_opts = match getopts::getopts(args.tail(), opts) {
        Ok(m) => { m },
        Err(f) => { panic!(f.to_string()) }
    };
    // bail out if help flag present
    if cmd_opts.opt_present("h") {
        print_usage(program.as_slice(), opts);
        return None;
    }
    match cmd_opts.opt_str("c") {
        Some(s) => {
            settings.config_file_path = Some(s);
        }
        None => {}
    }
    // First load config
    if settings.config_file_path.is_some() {
        load_config(&mut settings);
    }
    // Then load cmd line options
    match cmd_opts.opt_str("p") {
        Some(s) => {
            settings.pid_file_path = Some(s);
        },
        _ => {}
    }
    Some(settings)
}

impl Settings {
    pub fn pid_file_path<'a>(&'a self) -> Option<&'a str> {
        match self.pid_file_path {
            Some(ref s) => { Some(s.as_slice()) },
            None => { None }
        }
    }
}

fn print_usage(program: &str, opts: &[getopts::OptGroup]) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", getopts::usage(brief.as_slice(), opts));
}

fn load_config(settings: &mut Settings) {
    let mut conf_file = match io::File::open(&Path::new(settings.config_file_path.clone().unwrap())) {
        Ok(file) => { file },
        Err(e) => {
            println!("Warning: failed to open config file, using defaults - {}", e);
            return;
        }
    };
    let conf_str = match conf_file.read_to_string() {
        Ok(s) => { s },
        Err(e) => {
            println!("Error reading in config file, {}", e);
            panic!("Exiting on fatal error");
        }
    };
    let mut parser = toml::Parser::new(conf_str.as_slice());
    let conf_table = match parser.parse() {
        Some(s) => { s },
        None => {
            println!("There were errors parsing toml config file:");
            for err in parser.errors.iter() {
                let (lo_line, lo_col) = parser.to_linecol(err.lo);
                let (hi_line, hi_col) = parser.to_linecol(err.hi);
                println!("  Row {} col {} to row {} col {}: {}",
                         lo_line, lo_col, hi_line, hi_col, err.desc);
            }
            panic!("Exiting on fatal error");
        }
    };

    // Now we have the hashmap we can get our options
    if conf_table.contains_key("pid_file") {
        match conf_table.get("pid_file").unwrap().as_str() {
            Some(s) => { settings.pid_file_path = Some(s.to_string()); },
            None => { panic!("Error: In config file pid_file should be a string") }
        }
    }
}
