extern crate dotenv;
extern crate lettre;
use chrono::Local;
use std::env;
use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message, message::Mailbox,
    Tokio1Executor,
};
use lettre::message::{header::ContentType};



use mongodb::{
   results::{ InsertOneResult},
   sync::{Client, Collection},
};
// Importing our struct from the template module
use crate::template::entry_template::WaitlistEntry;
use crate::template::entry_template::HeaderAndMessage;
// A struct that will be the instance of our Database connection
pub struct MongoInstance{
    col: Collection<WaitlistEntry>
}
// refactor it so it includes an option as return
impl MongoInstance {
    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        // Loading database credenttials and establishing the DB connection
        dotenv::from_filename("credentials.env").ok();
    
        let uri = env::var("Connector")
            .map_err(|_| "Failed to load the connector environment variable")?;
        
        let client = Client::with_uri_str(&uri)
            .map_err(|err| format!("Failed to connect to MongoDB: {}", err))?;
        
        let db = client.database("rustDB");
        let col = db.collection("WaitlistEntry");
    
        Ok(MongoInstance { col })
    }

    pub fn create_entry(&self, new_entry: WaitlistEntry) -> Result<InsertOneResult,Box<dyn std::error::Error>>{
        // Creating Metadata and fully formating our database entry
        let local_time = Local::now();
        let formatted_date = local_time.format("%m/%d/%Y").to_string();
        let formatted_time = local_time.format("%H:%M:%S").to_string();

        let new_doc = WaitlistEntry {
            id: None,
            fname: new_entry.fname,
            lname: new_entry.lname,
            date: formatted_date,
            time: formatted_time,
            email: new_entry.email
        };

        // Error handling the database entry, and entering
        let dbinput = self.col.insert_one(new_doc, None)?;
        Ok(dbinput)
    }

    pub async fn fetch_mail_and_send(
        &self,
        new_email: HeaderAndMessage
    ) -> Result<(), Box<dyn std::error::Error>> {

        // Loading the email header and content
        let header_and_message = HeaderAndMessage {
            id: None,
            header: new_email.header,
            message: new_email.message,
        };
        // Collecting all the email fields from the databse
        let field_values: Vec<String> = self.col
            .find(None, None)
            .unwrap()
            .filter_map(|doc| doc.ok())
            .map(|entry| entry.email)
            .collect();

        //Loading SMTP server credentials
        dotenv::from_filename("credentials.env").ok(); 

         let username = env::var("email_client_username")
            .map_err(|_| "Failed to load the username environment variable")?;

        let password = env::var("email_client_password")
            .map_err(|_| "Failed to load the password environment variable")?;

        println!("Ime i prezime, {}, {}", &username, &password);
    
        let creds = Credentials::new(
            username.to_string(),
            password.to_string(),
        );

        let smtp_server = env::var("smtp")
            .map_err(|_| "Failed to load the smtp environment variable")?;

            println!("smtp, {}", &smtp_server);

        // creating the connection with the email transport handler
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&smtp_server)? 
        .credentials(creds)
        .build();
        
        // Sending the email
        let _output = send_mail_smtp(&mailer, &header_and_message, field_values).await?;


        Ok(())
    }

}


pub async fn send_mail_smtp ( 
    mailer: &AsyncSmtpTransport<Tokio1Executor>,
    content: &HeaderAndMessage,
    emails: Vec<String> )
 ->  Result<(), Box<dyn std::error::Error>>{
    //Looping through the email vector, constructing the email and sending
    for email in emails{

        let email = Message::builder()
            .from(Mailbox::new(None, "myapp@noreply.com".parse().unwrap()))
            .reply_to(Mailbox::new(None, "myapp@noreply.com".parse().unwrap()))
            .to(Mailbox::new(None, email.parse().unwrap()))
            .subject(content.header.clone())
            .header(ContentType::TEXT_PLAIN)
            .body(content.message.clone())?;

        match mailer.send(email).await {
            Ok(_) => println!("Email sent successfully."),
            Err(e) => println!("Failed to send email: {}", e),
        }
        
    }
    Ok(())
}


