use time;
use std::time::duration;
use std::io;
use std::str::FromStr;
use std::ops;

#[derive(Show)]
pub struct StatSnapshot {
    time: time::Timespec,
    cpu_no: Option<u32>,
    user: u32,
    nice: u32,
    system: u32,
    idle: u32,
    iowait: u32,
    irq: u32,
    softirq: u32
}

#[derive(Show)]
pub struct StatDelta {
    time: duration::Duration,
    cpu_no: Option<u32>,
    user: u32,
    nice: u32,
    system: u32,
    idle: u32,
    iowait: u32,
    irq: u32,
    softirq: u32
}

#[derive(Show)]
pub struct StatReport {
    duration: duration::Duration,
    cpu_no: Option<u32>,
    user: f32,
    nice: f32,
    system: f32,
    idle: f32,
    iowait: f32,
    irq: f32,
    softirq: f32,
    total_used: f32
}

/// Function to check cpus match, and give helpful error message on failure
fn check_cpus_match(cpu0: Option<u32>, cpu1: Option<u32>) {
    match (cpu0, cpu1) {
        (Some(c), None) | (None, Some(c)) => {
            panic!("Cannot add cpu aggregate and cpu {}", c);
        },
        (Some(c1), Some(c2)) => {
            if c1 != c2 {
                panic!("Cannot add different cpus '{}' and {}", c1, c2);
            }
        },
        _ => {}
    }
}

impl ops::Add<StatDelta> for StatSnapshot {
    type Output = StatSnapshot;
    fn add(self, other: StatDelta) -> StatSnapshot {
        check_cpus_match(self.cpu_no, other.cpu_no);
        StatSnapshot {
            time: self.time + other.time,
            cpu_no: self.cpu_no,
            user: self.user + other.user,
            nice: self.nice + other.nice,
            system: self.system + other.system,
            idle: self.idle + other.idle,
            iowait: self.iowait + other.iowait,
            irq: self.irq + other.irq,
            softirq: self.softirq + other.softirq
        }
    }
}

impl ops::Sub<StatDelta> for StatSnapshot {
    type Output = StatSnapshot;
    fn sub(self, other: StatDelta) -> StatSnapshot {
        check_cpus_match(self.cpu_no, other.cpu_no);
        StatSnapshot {
            time: self.time - other.time,
            cpu_no: self.cpu_no,
            user: self.user - other.user,
            nice: self.nice - other.nice,
            system: self.system - other.system,
            idle: self.idle - other.idle,
            iowait: self.iowait - other.iowait,
            irq: self.irq - other.irq,
            softirq: self.softirq - other.softirq
        }
    }
}

impl ops::Sub for StatSnapshot {
    type Output = StatDelta;

    fn sub(self, other: StatSnapshot) -> StatDelta {
        check_cpus_match(self.cpu_no, other.cpu_no);
        StatDelta {
            time: self.time - other.time,
            cpu_no: self.cpu_no,
            user: self.user - other.user,
            nice: self.nice - other.nice,
            system: self.system - other.system,
            idle: self.idle - other.idle,
            iowait: self.iowait - other.iowait,
            irq: self.irq - other.irq,
            softirq: self.softirq - other.softirq
        }
    }
}

pub fn monitor_stat() -> StatReport {
    let start = get_stat_snapshot();
    io::timer::sleep(duration::Duration::seconds(5));
    StatReport::from_stat_delta(get_stat_snapshot() - start)
}

pub fn get_stat_snapshot() -> StatSnapshot {
    // e.g. cpu  123 1234 1234 1234 1234 235 2354
    // don't match to end in case of extensions
    let cpu_re = regex!(r"^cpu(?P<cpu_no>\d*)\s+(?P<user>\d+)\s+(?P<nice>\d+)\s+(?P<system>\d+)\s+(?P<idle>\d+)\s+(?P<iowait>\d+)\s+(?P<irq>\d+)\s+(?P<softirq>\d+)");

    let stat_file = match io::File::open(&Path::new("/proc/stat")) {
        Ok(r) => { r },
        Err(e) => {
            panic!("Could not read /proc/stat: {}", e);
        }
    };
    let mut stat_reader = io::BufferedReader::new(stat_file);
    let cpu_line = match stat_reader.read_line() {
        Ok(s) => { s },
        Err(e) => {
            panic!("Error reading cpu line of /proc/stat: {}", e);
        }
    };
    match cpu_re.captures(cpu_line.as_slice()) {
        Some(caps) => {
            StatSnapshot {
                time: time::now_utc().to_timespec(),
                cpu_no: FromStr::from_str(caps.name("cpu_no").unwrap()),
                user: FromStr::from_str(caps.name("user").unwrap()).unwrap(),
                nice: FromStr::from_str(caps.name("nice").unwrap()).unwrap(),
                system: FromStr::from_str(caps.name("system").unwrap()).unwrap(),
                idle: FromStr::from_str(caps.name("idle").unwrap()).unwrap(),
                iowait: FromStr::from_str(caps.name("iowait").unwrap()).unwrap(),
                irq: FromStr::from_str(caps.name("irq").unwrap()).unwrap(),
                softirq: FromStr::from_str(caps.name("softirq").unwrap()).unwrap(),
            }
        },
        None => {
            panic!("Error parsing cpu line of /proc/stat");
        }
    }

}

impl StatReport {
    fn from_stat_delta(from: StatDelta) -> StatReport {
        let total = (from.user + from.nice + from.system + from.idle + from.iowait +
            from.irq + from.softirq) as f32;
        StatReport {
            duration: from.time,
            cpu_no: from.cpu_no,
            user: from.user as f32 / total,
            nice: from.nice as f32 / total,
            system: from.system as f32 / total,
            idle: from.idle as f32 / total,
            iowait: from.iowait as f32 / total,
            irq: from.irq as f32 / total,
            softirq: from.softirq as f32 / total,
            total_used: from.user as f32 / total
        }
    }
}
