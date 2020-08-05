#![allow(unused)]

pub mod config_util;
pub mod db_util;
pub mod log_util;
pub mod result;
pub mod sign_util;

#[cfg(test)]
mod tests {
    #[test]
    fn log_util_init_test() {
        use super::log_util;

        log_util::init();
    }

    #[test]
    fn config_util_test() {
        use super::config_util;
        assert_eq!(config_util::is_prod(), false);
    }

    #[test]
    fn sign_util_test() {
        use super::sign_util;
        let sign = sign_util::blake2_sign("hello world");
        println!("sign:{}", sign);
    }

    #[test]
    fn ajax_result_test() {
        use super::result;

        let ajax_result = result::AjaxResult::<bool>::success_without_data();
        assert_eq!(ajax_result.get_msg(), "success");
        match ajax_result.get_data() {
            Some(v) => assert_eq!(v.len(), 0),
            None => assert_eq!(true, true),
        }
    }
}
