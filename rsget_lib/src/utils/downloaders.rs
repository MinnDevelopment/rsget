use std::fs::File;
use std::io::Write;
use futures::{Future, Stream};
use hyper;
use tokio_core::reactor::Core;
use hyper::header::Location;

use indicatif::ProgressBar;

fn get_redirect_url(url: String) -> String {
    let mut core = match Core::new() {
        Ok(c) => c,
        Err(why) => panic!("why: {}", why),
    };
    
    let client = hyper::Client::new(&core.handle());

    let uri = match url.parse() {
        Ok(u) => u,
        Err(why) => panic!("why: {}", why),
    };

    let work = client.get(uri);
    let res = core.run(work).unwrap();
    
    match res.headers().get::<Location>() {
        Some(loc) => loc.parse::<String>().unwrap(),
        None => url,
    }
}


pub fn flv_download(url: String, path: String) -> Option<()> 
{
    let real_url = get_redirect_url(url);
    
    let mut core = match Core::new() {
        Ok(c) => c,
        Err(why) => panic!("why: {}", why),
    };
    let client = hyper::Client::new(&core.handle());
    
    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(why) => panic!("WHY: {}", why),
    };
    
    let uri = match real_url.parse() {
        Ok(u) => u,
        Err(why) => panic!("why: {}", why),
    };
    let mut size: f64 = 0.0;
    let spinner = ProgressBar::new_spinner();
    let work = client.get(uri).and_then(|res| {
        res.body().for_each(|chunk| {
            spinner.tick();
            size = size + (chunk.len() as f64);
            spinner.set_message(&format!("Size: {:.2} MB", size/1000.0/1000.0));
            file
                .write_all(&chunk)
                .map_err(From::from)            
        })
    });
    match core.run(work) {
        Ok(_) => Some(()),
        Err(why) => {
            warn!("Core: {}", why);
            None
        },
    }
}
