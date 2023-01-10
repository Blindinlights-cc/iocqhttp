#![allow(clippy::let_unit_value)]

use api::HttpApi;
use error::Error;
pub mod api;
pub mod bus;
pub mod error;
pub mod event;
pub mod message;
pub use api::*;
pub use bus::*;
pub use event::*;
pub use error::*;
pub use message::*;
use event::Event;
use log::{log, Level};
use reqwest::Response;
use rocket::{routes, Config};
#[derive(Default)]
pub struct CQHttp<T: Default> {
    api: HttpApi,
    //sever_app:Option<Rocket<Build>>,
    access_token: Option<String>,
    secret: Option<String>,
    api_root: String,
    api_timeout: Option<u64>,
    bus: bus::EventBus<T>,
}
type AsyncFunc<T> = Box<dyn bus::EventHandler<Output = (), Args = T>>;
#[allow(clippy::new_without_default)]
impl<T> CQHttp<T> 
where
    T: Default,
{
    pub fn new(api_root: &str) -> Self {
        Self {
            api_root: api_root.to_string(),
            ..Default::default()
        }
    }
    pub fn run_server(&mut self, ip: std::net::IpAddr, port: u16) -> rocket::Rocket<rocket::Build> {
        let config = if let Some(key) = self.secret.clone() {
            Config::figment()
                .merge(("address", ip.to_string()))
                .merge(("port", port))
                .merge(("secret_key", key))
        } else {
            Config::figment()
                .merge(("address", ip.to_string()))
                .merge(("port", port))
        };
        let config = Config::from(config);
        rocket::custom(config)
            .mount("/", routes![handle_event])
            .manage(self.secret.clone())
    }
    pub fn access_token(mut self, token: &str) -> Self {
        self.access_token = Some(token.to_string());
        self
    }
    pub fn secret(mut self, secret: &str) -> Self {
        self.secret = Some(secret.to_string());
        self
    }
    pub fn api_timeout(mut self, timeout: u64) -> Self {
        self.api_timeout = Some(timeout);
        self
    }
    pub fn build_api(&mut self) {
        self.api = HttpApi::new(&self.api_root, self.access_token.clone(), self.api_timeout);
    }
    pub async fn send<U: ToString>(&self, event: &Event, message: U) -> Result<Response, Error> {
        match event {
            Event::GroupMessage(e) => {
                self.api
                    .send_group_msg(e.group_id, message, false, e.self_id)
                    .await
            }
            Event::PrivateMessage(e) => {
                self.api
                    .send_private_msg(e.sender.user_id, message, false, e.self_id)
                    .await
            }
            _ => {
                Err(Error::ReplyEvent)
            }
        }
    }
    pub fn subscribe(&mut self,name:&str,handler:AsyncFunc<T>){
        self.bus.subscribe(name,handler);
    }
    pub fn unsubscribe(&mut self,name:&str){
        self.bus.unsubscribe(name);
    }
    pub fn hook_before(&mut self,name:&str,handler:AsyncFunc<T>){
        self.bus.hook_before(name,handler);
    }
    pub fn unhook_before(&mut self,name:&str){
        self.bus.unhook_before(name);
    }
}

#[rocket::post("/", data = "<data>")]
fn handle_event(data: String) {
    let event = Event::from_str(&data);
    if let Err(e) = event {
        log!(Level::Error, "{}", e);
        return;
    }
    let event = event.unwrap();
    log_event(&event);
    todo!()
}
fn log_event(event: &Event) {
    match event {
        Event::GroupMessage(e) => {
            let group_id = e.group_id;
            let sender_id = e.sender.user_id;
            let sender_name = e.sender.nickname.clone();
            let message = e.message.clone();
            let info = format!(
                "收到群{}的消息{}({}):{}",
                group_id, sender_id, sender_name, message
            );
            log!(Level::Info, "{}", info);
        }
        Event::PrivateMessage(e) => {
            let sender_id = e.sender.user_id;
            let sender_name = e.sender.nickname.clone();
            let message = e.message.clone();
            let info = format!("收到私聊消息{}({}):{}", sender_id, sender_name, message);
            log!(Level::Info, "{}", info);
        }
        _ => {}
    }
}
