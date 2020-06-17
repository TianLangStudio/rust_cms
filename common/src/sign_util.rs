use blake2::{Blake2b, Digest};
use data_encoding::BASE64;
use super::config_util;

fn get_salt() -> String {
      config_util::APP_CONFIG.get_str("tl.app.sign.salt")
                    .expect("tl.app.sign.salt is required")
}
pub fn blake2_sign(text: &str) -> String {
        let sign = Blake2b::new() 
                .chain(get_salt())
                .chain(text)
                .finalize();
        BASE64.encode(&sign)
}