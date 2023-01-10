macro_rules! make_event{
    (
     $(#[$meta:meta])*
     $vis:vis struct $struct_name:ident {
        $(
        $(#[$field_meta:meta])*
        $field_vis:vis $field_name:ident : $field_type:ty
        ),*$(,)+
    }
    ) => {

            $(#[$meta])*
            #[derive(serde::Serialize,serde::Deserialize,Debug,Clone)]
            pub struct $struct_name{
                pub post_type: String,
                pub self_id: i64,
                pub time: i64,
                $(
                $(#[$field_meta:meta])*
                pub $field_name : $field_type,
                )*
            }
    }
}
macro_rules! make_msg_event {
    (
        $(#[$meta:meta])*
        $vis:vis struct $struct_name:ident {
           $(
           $(#[$field_meta:meta])*
           $field_vis:vis $field_name:ident : $field_type:ty
           ),*$(,)+
       }
    ) => {
        make_event!{
            $(#[$meta])*
            pub struct $struct_name{
                message_type:String,
                sub_type:String,
                message_id:i64,
                user_id:i64,
                message:String,
                raw_message:String,
                font:i32,
                sender:Sender,
                $(
                pub $field_name : $field_type,
                )*
            }

        }
    };
}
macro_rules! make_notice_event {
    (
        $(#[$meta:meta])*
        $vis:vis struct $struct_name:ident {
           $(
           $(#[$field_meta:meta])*
           $field_vis:vis $field_name:ident : $field_type:ty
           ),*$(,)+
       }
       ) => {
               make_event!{
               $(#[$meta])*
               pub struct $struct_name{
                    pub notice_type:String,
                   $(

                   pub $field_name : $field_type,
                   )*
               }

               }

       }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Sender {
    pub age: i32,
    pub nickname: String,
    pub sex: String,
    pub user_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct FileInfo {
    id: String,
    name: String,
    size: i64,
    busid: i64,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct OfflineFile {
    name: String,
    size: i64,
    url: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Anonymous {
    id: i64,
    name: String,
    flag: String,
}
make_event! {
    struct PrivateMessage{
        message_type:String,
        sub_type:String,
        message_id:i64,
        user_id:i64,
        message:String,
        raw_message:String,
        font:i32,
        sender:Sender,
    }
}
make_msg_event! {
    struct GroupMessage{
        group_id:i64,
        anonymous:Option<Anonymous>,
    }
}
make_notice_event! {
    struct GroupFileUpload{
        group_id:i64,
        user_id:i64,
        file:FileInfo,
    }
}
make_notice_event! {
    struct GroupAdminChange{
        sub_type:String,
        group_id:i64,
        user_id:i64,
    }
}
make_notice_event! {
    struct GroupMemberReduce{
        sub_type:String,
        group_id:i64,
        user_id:i64,
        operator_id:i64,
    }
}

make_notice_event! {
    struct GroupMemberIncrease{
        sub_type:String,
        group_id:i64,
        user_id:i64,
        operator_id:i64,
    }
}
make_notice_event! {
    struct GroupMute{
        sub_type:String,
        group_id:i64,
        operator_id:i64,
        user_id:i64,
        duration:i64,
    }
}
make_notice_event! {
    struct FriendAdd{
        user_id:i64,
    }
}
make_notice_event! {
    struct OfflineFileUpload{
        file:OfflineFile,
    }
}

make_notice_event! {
    struct GroupMessageRecall{
        group_id:i64,
        message_id:i64,
        user_id:i64,
        operator_id:i64,

    }
}
make_notice_event! {
    struct FriendMessageRecall{
        user_id:i64,
        message_id:i64,
    }
}
make_notice_event! {
    struct FriendPoke{
        sub_type:String,
        user_id:i64,
        sender_id:i64,
        target_id:i64,
    }
}
make_notice_event! {
    struct GroupPoke{
        sub_type:String,
        group_id:i64,
        sender_id:i64,
        target_id:i64,
    }
}
make_event! {
    struct FriendRequest{
        request_type:String,
        user_id:i64,
        comment:String,
        flag:String,
    }
}
make_event! {
    struct GroupRequest{
        request_type:String,
        sub_type:String,
        group_id:i64,
        user_id:i64,
        comment:String,
        flag:String,
    }
}
make_event! {
    struct MetaEvent{
        meta_event_type:String,
        status:String,
        interval:i64,
    }
}
#[derive(Debug)]
pub enum Event {
    PrivateMessage(PrivateMessage),
    GroupMessage(GroupMessage),
    GroupFileUpload(GroupFileUpload),
    GroupAdminChange(GroupAdminChange),
    GroupMemberReduce(GroupMemberReduce),
    GroupMemberIncrease(GroupMemberIncrease),
    GroupMute(GroupMute),
    FriendAdd(FriendAdd),
    GroupMessageRecall(GroupMessageRecall),
    FriendMessageRecall(FriendMessageRecall),
    FriendPoke(FriendPoke),
    GroupPoke(GroupPoke),
    FriendRequest(FriendRequest),
    GroupRequest(GroupRequest),
    MetaEvent(MetaEvent),
    OfflineFileUpload(OfflineFileUpload),
    Unknown,
}
impl Event {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(v:&str)->Result<Event,serde_json::Error>{
        let value:serde_json::Value=serde_json::from_str(v)?;
        get_event(&value)
    }
}
pub enum MsgEvent {
    PrivateMessage(PrivateMessage),
    GroupMessage(GroupMessage),
}
pub fn get_event(event: &serde_json::Value) -> Result<Event, serde_json::Error> {
    let post_type = event["post_type"].as_str().unwrap();
    match post_type {
        "message" => {
            let message_type = event["message_type"].as_str().unwrap();
            match message_type {
                "private" => Ok(Event::PrivateMessage(serde_json::from_value(
                    event.clone(),
                )?)),
                "group" => Ok(Event::GroupMessage(serde_json::from_value(event.clone())?)),
                _ => Ok(Event::Unknown),
            }
        }
        "notice" => {
            let notice_type = event["notice_type"].as_str().unwrap();
            match notice_type {
                "group_upload" => Ok(Event::GroupFileUpload(
                    serde_json::from_value(event.clone()).unwrap(),
                )),
                "group_admin" => Ok(Event::GroupAdminChange(
                    serde_json::from_value(event.clone()).unwrap(),
                )),
                "group_decrease" => Ok(Event::GroupMemberReduce(
                    serde_json::from_value(event.clone()).unwrap(),
                )),
                "group_increase" => Ok(Event::GroupMemberIncrease(
                    serde_json::from_value(event.clone()).unwrap(),
                )),
                "group_ban" => Ok(Event::GroupMute(
                    serde_json::from_value(event.clone()).unwrap(),
                )),
                "friend_add" => Ok(Event::FriendAdd(
                    serde_json::from_value(event.clone()).unwrap(),
                )),
                "group_recall" => Ok(Event::GroupMessageRecall(
                    serde_json::from_value(event.clone()).unwrap(),
                )),
                "friend_recall" => Ok(Event::FriendMessageRecall(
                    serde_json::from_value(event.clone()).unwrap(),
                )),
                "poke" => Ok(Event::GroupPoke(
                    serde_json::from_value(event.clone()).unwrap(),
                )),
                "offline_file" => Ok(Event::OfflineFileUpload(
                    serde_json::from_value(event.clone()).unwrap(),
                )),
                _ => Ok(Event::Unknown),
            }
        }

        "request" => {
            let request_type = event["request_type"].as_str().unwrap();
            match request_type {
                "friend" => Ok(Event::FriendRequest(
                    serde_json::from_value(event.clone()).unwrap(),
                )),
                "group" => Ok(Event::GroupRequest(
                    serde_json::from_value(event.clone()).unwrap(),
                )),
                _ => Ok(Event::Unknown),
            }
        }
        "meta_event" => Ok(Event::MetaEvent(
            serde_json::from_value(event.clone()).unwrap(),
        )),

        _ => Ok(Event::Unknown),
    }
}
