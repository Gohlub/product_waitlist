use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct WaitlistEntry{
   #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   pub id: Option<ObjectId>,
   pub fname: String,
   pub lname: String,
   pub date: String,
   pub time: String,
   pub email: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TemporaryWaitlistEntry{
   #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   pub id: Option<ObjectId>,
   pub fname: String,
   pub lname: String,
   pub email: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderAndMessage{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub header: String,
    pub message: String,
}


