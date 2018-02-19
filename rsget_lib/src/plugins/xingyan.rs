use Streamable;
use reqwest;
use regex::Regex;
use serde_json;

use utils::downloaders::flv_download;
use chrono::prelude::*;

use tokio_core::reactor::Core;

use std;

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
struct XingyanRoomInfo {
    rid: String,
    xid: usize,
    name: String,
    notice: String,
    photo: String,
    picture: String,
    playstatus: String,
    status: String,
    lock_reason: Option<String>,
    personnum: String,
    starttime: String,
    endtime: String,
    label: Vec<String>,
    shareimg: String,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
struct XingyanAds {
    title: String,
    img: String,
    linkurl: String,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
struct XingyanVideoInfo {
    streamurl: String,
    hlsurl: String,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
struct XingyanHostInfo {
    rid: String,
    nickName: String,
    avatar: String,
    gender: String,
    signature: String,
    is_anchor: String,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
struct XingyanInfo {
    roominfo: XingyanRoomInfo,
    videoinfo: XingyanVideoInfo,
    hostinfo: XingyanHostInfo,
    leftads: Vec<XingyanAds>,
}


pub struct Xingyan {
    pub url: String,
    pub room_id: String,
    host_info: XingyanInfo,
}


impl Streamable for Xingyan {
    fn new(url: String) -> Xingyan {
        let room_id_re = Regex::new(r"/([0-9]+)").unwrap();
        let cap = room_id_re.captures(&url).unwrap();
        let site_url = format!("https://xingyan.panda.tv/{}", &cap[1]);
        let resp = reqwest::get(&site_url);
        let res: Result<String, reqwest::Error> = resp.unwrap().text();
        match res {
            Ok(some) => {
                let hostinfo_re = Regex::new(r"<script>window.HOSTINFO=(.*);</script>").unwrap();
                let hi_cap = hostinfo_re.captures(&some).unwrap();
                let hi: XingyanInfo = serde_json::from_str(&hi_cap[1]).unwrap();
                Xingyan {
                    url: url.clone(),
                    room_id: String::from(&cap[1]),
                    host_info: hi,
                }
            }
            Err(why) => {
                debug!("{}", why);
                std::process::exit(1)
            }
        }
    }

    fn get_title(&self) -> Option<String> {
        Some(self.host_info.roominfo.name.clone())
    }

    fn get_author(&self) -> Option<String> {
        Some(self.host_info.hostinfo.nickName.clone())
    }

    fn is_online(&self) -> bool {
        self.host_info.roominfo.playstatus != "0"
    }

    fn get_stream(&self) -> String {
        self.host_info.videoinfo.streamurl.clone()
    }

    fn get_ext(&self) -> String {
        String::from("flv")
    }

    fn get_default_name(&self) -> String {
        let local: DateTime<Local> = Local::now();
        format!(
            "{}-{:04}-{:02}-{:02}-{:02}-{:02}-{}.{}",
            self.get_author().unwrap(),
            local.year(),
            local.month(),
            local.day(),
            local.hour(),
            local.minute(),
            self.get_title().unwrap(),
            self.get_ext()
        )
    }

    fn download(&self, core: &mut Core, path: String) -> Option<()> {
        if !self.is_online() {
            None
        } else {
            println!(
                "{} by {} ({})",
                self.get_title().unwrap(),
                self.get_author().unwrap(),
                self.room_id
            );
            flv_download(core, self.get_stream(), path)
        }
    }
}
