use log::{debug, error, info, trace, warn};
use log4rs;

pub fn init() {
    log4rs::init_file("conf/log4rs.yaml", Default::default()).unwrap();
    let test_msg_pre = "log level";
    let test_msg_suf = "test message";
    trace!("{} {} {}", test_msg_pre, "trace", test_msg_suf);
    debug!("{} {} {}", test_msg_pre, "debug", test_msg_suf);
    info!("{} {} {}", test_msg_pre, "info", test_msg_suf);
    warn!("{} {} {}", test_msg_pre, "warn", test_msg_suf);
    error!("{} {} {}", test_msg_pre, "error", test_msg_suf);
}
