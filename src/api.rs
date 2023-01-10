use reqwest::{ Response,Body};
use crate::error::Error;
/// use HTTP to call Onebot API
#[derive(Default)]
pub struct HttpApi {
    api_root: String,
    access_token: Option<String>,
    timeout: Option<u64>,
    //action: String,
}
impl HttpApi {
    pub fn new(api_root: &str, access_token: Option<String>, timeout: Option<u64>) -> HttpApi {
        HttpApi {
            api_root: api_root.to_string(),
            access_token,
            timeout,
        }
    }
    pub async fn call_actions<T:Into<Body>>(&self,action: &str,data:T) -> Result<Response, Error> {
        let client = if let Some(timeout) =self.timeout  {
            reqwest::ClientBuilder::new().timeout(std::time::Duration::from_secs(timeout)).build()?
        }else{
            reqwest::Client::new()
        };
        let url = format!("{}/{}", self.api_root, action);
        let res = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token.clone().unwrap_or_default()))
            .body(data)
            .send()
            .await?;
        Ok(res)
    }
    /// 
    /// 发送私聊消息\
    /// 
    /// `user_id`:目标QQ\
    /// `message`:消息内容\  
    /// `auto_escape`:消息内容是否作为纯文本发送（即不解析 CQ 码)
    /// `self_id`:机器人QQ
    pub async fn send_private_msg<T:ToString>(&self,user_id:i64,message:T,auto_space:bool,self_id:i64)->Result<Response,Error>{
        let auto_space=auto_space as u8; 
        let json=serde_json::json!({
            "user_id":user_id,
            "message":message.to_string(),
            "auto_escape":auto_space,
            "self_id":self_id
        });
        let data=serde_json::to_string(&json).unwrap();
        self.call_actions("send_private_msg",data).await
    }
    /// 
    /// 发送群消息\
    /// `group_id`:目标群\
    /// `message`:消息内容\
    /// `auto_escape`:消息内容是否作为纯文本发送（即不解析 CQ 码)
    /// `self_id`:机器人QQ
    pub async fn send_group_msg<T:ToString>(&self,group_id:i64,message:T,auto_space:bool,self_id:i64)->Result<Response,Error>{
        let json=serde_json::json!({
            "group_id":group_id,
            "message":message.to_string(),
            "auto_escape":auto_space,
            "self_id":self_id
        });
        let data=serde_json::to_string(&json).unwrap();
        self.call_actions("send_group_msg",data).await
    }
    ///
    /// 获取消息的真实ID\
    /// `message_id`:消息ID\
    /// `self_id`:机器人QQ
    pub async fn get_msg(&self,msg_id:i64,self_id:i64)->Result<Response,Error>{
        let json=serde_json::json!({
            "message_id":msg_id,
            "self_id":self_id
        });
        let data=serde_json::to_string(&json).unwrap();
        self.call_actions("get_msg",data).await
    }
    pub async fn send_msg(&self,msg_type:MsgType,group_id:i64,user_id:i64,message:String,auto_space:bool,self_id:i64)->Result<Response,Error>{
        match msg_type{
            MsgType::Private=>self.send_private_msg(user_id,message,auto_space,self_id).await,
            MsgType::Group=>self.send_group_msg(group_id,message,auto_space,self_id).await,
            MsgType::Discuss=>unimplemented!(),
        }
    }
}
pub enum MsgType{
    Private,
    Group,
    Discuss,
}
pub struct WebSocketReverseApi{

}
