use blake2::{Blake2b, Digest};
use data_encoding::BASE64;
use super::config_util;

fn get_salt() -> String {
      config_util::APP_CONFIG.get_str("tl.app.sign.salt")
                    .expect("tl.app.sign.salt is required")
}

//使用默认盐值加密，若盐值变动结果就变动了，所以只能用在临时场景 
pub fn blake2_sign_temp(text: &str) -> String {
        blake2_sign_with_salt(text,  &get_salt())
}

//使用指定盐值加密
pub fn blake2_sign_with_salt(text: &str,  salt: &str) ->  String{
        let sign = Blake2b::new() 
        .chain(salt)
        .chain(text)
        .finalize();
       BASE64.encode(&sign)
}