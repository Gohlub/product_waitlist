use crate::{template::entry_template::{WaitlistEntry, TemporaryWaitlistEntry, HeaderAndMessage}, db_handler::db_manipulation::MongoInstance};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};
use rocket::response::status;
/*
    Request_handler.rs is the default route handler for all our HTTP requests.
    The functionality of now lies mostly within post requests, more namely creating
    an entry to the waitlist (which is stored in a MongoDB database). The other
    functionality is for the user to share a message with all the database entries,
    as one of the fields involves an email address entry.
 */

// Our POST request, 
#[post("/user_entry", data = "<new_entry>")]
pub fn create_entry(
    db: &State<MongoInstance>,
    new_entry: Json<TemporaryWaitlistEntry>
) -> Result<Json<InsertOneResult>, status::Custom<String>>{
    //Creating a template for our database entry based on a Json message body from the request
    let input = WaitlistEntry{
        id: None,
        fname: new_entry.fname.to_owned(),
        lname: new_entry.lname.to_owned(),
        date: "".to_string(),
        time: "".to_string(),
        email: new_entry.email.to_owned()
    };
    //calling the handler that manipulates the database, in this case for adding a new entry
    let entry = db.create_entry(input)
    .map_err(|err| status::Custom(Status::InternalServerError, format!("Failed to create entry: {}", err)))?;

Ok(Json(entry))
}

#[post("/email_all", format = "json", data = "<header_message>")]
pub async fn send_mail(db: &State<MongoInstance>, header_message: Json<HeaderAndMessage>) ->  Result<status::Custom<String>, status::Custom<String>> {
    //Creating the header and contents for our message from the Json body post request.
    let message = HeaderAndMessage{
        id: None,
        header: header_message.header.to_owned(),
        message: header_message.message.to_owned()
    };
    /*  
       Handler for extrapolaing the email entries and parsing the message to the
       appropriate SMTP server
     */
    if let Err(err) = db.fetch_mail_and_send(message).await {
        // Handling errors
        let error_message = format!("Failed to send emails: {}", err);

        return Err(status::Custom(Status::InternalServerError, error_message));
    }else{
        let message = format!("All emails sent succesfully");

        return Ok(status::Custom(Status::Ok, message));
    }


}
