use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Escape a str to be used as a CQ code.
/// `comma` is boolean to control whether to escape comma or not.
pub fn escape(s: &str,comma:bool) -> String {
    let s=s.replace('&', "&amp;")
        .replace('[',"&#91;")
        .replace(']',"&#93;");
    if comma {
        s.replace(',',"&#44;")
    } else {
        s
    }
}

/// Unescape CQ code.
pub fn unescape(s: &str) -> String {
    s.replace("&#44;", ",")
        .replace("&#91;", "[")
        .replace("&#93;", "]")
        .replace("&amp;", "&")
     
}
#[derive(Default,PartialEq,Debug,Serialize,Deserialize)]
pub struct MessageSegment{
    #[serde(rename(serialize="type",deserialize="type"))]
    name:String,
    data:HashMap<String,String>,
}
impl MessageSegment {
    pub fn new(name:&str,data:Option<HashMap<String,String>>)->MessageSegment{
        MessageSegment{
            name:name.to_string(),
            data:data.unwrap_or_default(),
        }
    }
    pub fn name(&self)->String{
        self.name.clone()
    }
    pub fn data(&self)->HashMap<String,String>{
        self.data.clone()
    }
    /// Create a text CQ code.
    pub fn text(text:&str)->MessageSegment{
        
        let mut data=HashMap::new();
        data.insert("text".to_string(),text.to_string());
        MessageSegment::new("text",Some(data))
    }

    /// Create a QQ emoji CQ code.
    /// `id` is the id of the emoji.
    /// see [QQ Emoji id list](https://github.com/richardchien/coolq-http-api/wiki/%E8%A1%A8%E6%83%85-CQ-%E7%A0%81-ID-%E8%A1%A8)
    pub fn face(id:&i32)->MessageSegment{
        
        let mut data=HashMap::new();
        data.insert("id".to_string(),id.to_string());
        MessageSegment::new("face",Some(data))
    }
    /// Create a "at" someone CQ code.
    /// 'qq' is the qq number of the person you want to at.
    pub fn at(qq:i64)->MessageSegment{
        let mut data=HashMap::new();
        data.insert("qq".to_string(),qq.to_string());
        MessageSegment::new("at",Some(data))
    }
    /// Create a emoji CQ code.
    /// `id` is the id of the emoji.
    /// 
    pub fn emoji(id:i32)->MessageSegment{
        let mut data=HashMap::new();
        data.insert("id".to_string(),id.to_string());
        MessageSegment::new("emoji",Some(data))
    }

    /// Create a image CQ code.
    /// `file`:the url of the image.
    /// 
    /// `cache`: boolean to control whether to cache the image or not.
    /// 
    /// `proxy`:boolean to control whether to use proxy or not.
    /// 
    /// `id`:special effect id,defualt is 40000
    /// 
    /// `ty`:imge type, "flash" or "show"
    /// 
    /// `sub_type`: image subtype,only in group message
    /// 
    pub fn image(file:&str,cache:Option<bool>,id:Option<u32>,ty:Option<&str>,sub_type:Option<u32>)->MessageSegment{
        let mut data=HashMap::new();
        data.insert("file".to_string(),file.to_string());
        if let Some(cache)=cache{
            data.insert("cache".to_string(),(cache as u8).to_string());
        }
        if let Some(id)=id{
            data.insert("id".to_string(),id.to_string());
        }
        if let Some(ty)=ty{
            data.insert("type".to_string(),ty.to_string());
        }
        if let Some(sub_type)=sub_type{
            data.insert("subType".to_string(),sub_type.to_string());
        }
        MessageSegment::new("image",Some(data))
    }
    /// Create a record CQ code.
    /// `file`:the url of the record.
    /// 
    /// `magic`:boolean to control whether to use magic or not.
    /// 
    /// `cache`:boolean to control whether to cache the record or not.
    /// 
    /// `proxy`:boolean to control whether to use proxy or not.
    /// 
    /// `timeout`:download timeout
    pub fn record(file:&str,magic:Option<bool>,cache:Option<bool>,proxy:Option<bool>,timeout:Option<i32>)->Self{
        let mut data=HashMap::new();
        data.insert("file".to_string(),file.to_string());
        if let Some(magic)=magic{
            data.insert("magic".to_string(),(magic as u8).to_string());
        }
        if let Some(cache)=cache{
            data.insert("cache".to_string(),(cache as u8).to_string());
        }
        if let Some(proxy)=proxy{
            data.insert("proxy".to_string(),(proxy as u8).to_string());
        }
        if let Some(timeout)=timeout{
            data.insert("timeout".to_string(),timeout.to_string());
        }
        MessageSegment::new("record",Some(data))
    }

    /// Create a finger-guessing game CQ code.
    pub fn rps()->Self{
        MessageSegment::new("rps",None)
    }


}
impl  ToString for MessageSegment{
    fn to_string(&self) -> String {
        if self.name=="text"{
            return self.data.get("text").unwrap().to_string()
        }
        let mut s=format!("[CQ:{}",self.name);
        self.data.iter().for_each(|(k,v)|{
            s.push_str(&format!(",{}",escape(&format!("{}={}",k,v),true)));
        });
        s.push(']');
        s
    }
}
#[derive(Default)]
pub struct Message{
    segments:Vec<MessageSegment>
}
impl Message {
    pub fn new()->Self{
        Self::default()
    }
    pub fn append(&mut self,seg:MessageSegment){
        self.segments.push(seg)
    }
    
}
impl ToString for Message {
    fn to_string(&self) -> String {
        let mut msg=String::new();
        self.segments.iter().for_each(|s|{
            msg.push_str(s.to_string().as_str())
        });
        msg
    }
    
}
impl From<String> for Message {
    fn from(value: String) -> Self {
        let seg=MessageSegment::text(value.as_str());
        let mut msg=Message::new();
        msg.append(seg);
        msg
    }
}
impl From<&str> for Message {
    fn from(value: &str) -> Self {
        let seg=MessageSegment::text(value);
        let mut msg=Message::new();
        msg.append(seg);
        msg
    }
}
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_json(){
        let mut data=HashMap::new();
        data.insert("text".to_string(),"hello".to_string());
        let code=MessageSegment::new("text",Some(data));
        let s=serde_json::to_string(&code).unwrap();
        println!("{}",s);
        let code: MessageSegment = serde_json::from_str(&s).unwrap();
        println!("{:?}",code);
    }
    #[test]
    fn test_image(){
        let code=MessageSegment::image("http://www.baidu.com",Some(true),Some(40000),Some("flash"),Some(1));
        println!("{}",code.to_string());
        let json=serde_json::to_string(&code).unwrap();
        println!("{}",json);
    }
    #[test]
    fn test_record(){
        let code=MessageSegment::record("http://www.baidu.com",Some(true),Some(true),Some(true),Some(10));
        println!("{}",code.to_string());
        let json=serde_json::to_string(&code).unwrap();
        println!("{}",json);
    }
}
