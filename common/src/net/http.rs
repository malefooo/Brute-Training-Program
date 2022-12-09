use std::any::Any;
use crate::net::http::Method::{GET, POST};

/**
 * 提供http相关的api
 */



/**
 * 请求方法
 */
#[derive(Debug)]
enum Method {
    GET,
    POST,
}

impl From<String> for Method {
    fn from(s: String) -> Self {
        match s.as_str() {
            "GET" => GET,
            _ => POST
        }
    }
}


