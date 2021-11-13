use external_ip;
//use futures::executor::block_on;
use slack_hook::{PayloadBuilder, Slack};
use std::env;
use std::net::IpAddr;
use std::{thread, time};
use chrono::prelude::Local;
use tokio::time::timeout;
use std::time::Duration;
//use url::Url;

#[tokio::main]
async fn main() {
    loop {
        println!("in outer loop");
        for _ in 0..5 {
            println!("in the inner loop, getting ip");
            let result = external_ip::get_ip();
            let value = timeout(Duration::from_millis(1000), result).await;
        
            //let value: Option<IpAddr> = block_on(result);
            println!("after get ip");
            let address: String;
            match value {
                Err(_) => address = "error".to_string(),
                Ok(_) => match value.unwrap().unwrap() {
                    IpAddr::V4(ipv4) => address = ipv4.to_string(),
                    IpAddr::V6(ipv6) => address = ipv6.to_string(),
                },
            };
            println!("got ip: {}", address);

            let hook_url: String = match env::var("SLACK_HOOK_URL") {
                Ok(v) => v,
                Err(_v) => "a".to_string(),
            };
            println!("hook url is {}", hook_url);

            let slack = Slack::new(
                &hook_url[..],
            ).unwrap();

            let p = PayloadBuilder::new()
                .text(address)
                .channel("#ip")
                .username("IpNotifier")
                .icon_emoji(":chart_with_upwards_trend:")
                .build()
                .unwrap();

            println!("built payload, sending");
            let res = slack.send(&p);
            match res {
                Ok(()) => println!("ok"),
                Err(x) => println!("ERR: {:?}", x),
            };
            println!("sent");
        }

        let local = Local::now();
        println!("time now is {}, sleeping for half an hour", local);
        let half_hour = time::Duration::from_secs(30 * 60);
        thread::sleep(half_hour);
        println!("after sleep");
    }
}
