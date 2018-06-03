extern crate chrono;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate indicatif;
#[macro_use]
extern crate log;
extern crate md5;
extern crate regex;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;
extern crate http;
extern crate url;
extern crate hls_m3u8;

use utils::error::StreamError;

pub type HttpsClient = hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>;

pub trait Downloadable {
    fn new(url: String) -> Self;
    fn get_title(&self) -> Option<String>;
    fn get_author(&self) -> Option<String>;
    fn get_size(&self) -> Option<String>;
    fn get_stream(&self) -> String;
}

pub trait Streamable {
    fn new(client: HttpsClient, url: String) -> Result<Box<Self>, StreamError>
    where
        Self: Sized;
    fn get_title(&self) -> Option<String>;
    fn get_author(&self) -> Option<String>;
    //fn get_stream(&self) -> <T: Stream>
    fn is_online(&self) -> bool;
    fn get_stream(&self) -> String;
    fn get_ext(&self) -> String;
    fn get_default_name(&self) -> String;
    fn download(&self, client: HttpsClient, path: String) -> Result<(), StreamError>;
}
pub mod utils;
pub mod plugins;
