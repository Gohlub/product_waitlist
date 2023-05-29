# Product Waitlist API written in Rocket and MongoDB

For my Introduction to Rust final project, I decided to develop a Product Waitlist API utilizing Rocket, a web framework in Rust, and utilize MongoDB as backend database. 

## Capabilities
The functionalities of the API are for now constrained to POST requests, mainly adding an entry to the waitlist and sending update notices to those who provided a working email. Further features to be added include a security feature to the API, such as separating admin and guest permissions (admin having DML permissions and guest being able to update information and remove themselves from the waitlist). 

## Structure
The API is structed around the functionalities of different modules, namely api_handler, db_handler, templates along with the main module.
```
product_waitlist
└── src
    ├── api_handler
    │   ├── mod.rs
    │   └── request_handler.rs
    ├── db_handler
    │   ├── mod.rs
    │   └── db_manipulation.rs
    ├── templates
    │   ├── mod.rs
    │   └── entry_template.rs
    └── main.rs
```
The main module calls on the request_handler, handles the POST Json body (whether it is a new user entry or email header and content) to the appropriate function in the db_manipulation module. The templates module parses the Json bodies, which gives handles the form of the Json by using Structs (an analagous structure).

## Use
### Add a user to the waitlist (1)
```Rust
POST /user_entry
{
  "fname": "John",
  "lname": "Cena",
  "email": "johncena@bennington.edu"

}
```
### Send email to all users (2)

```Rust
POST /email_all
{
    "header": "Thank you for signing up!",
    "message": "We will be in touch with you soon!"
}

```
### Output:
```
All emails sent successfully! (1)

{"insertedId": {"$oid": "6474f7e103fc2dcf151aebab"}}  (2)

```

## Environment Variables

```
Connector='MongoDB driver'
email_client_username='SMTP Client Username'
email_client_password='SMTP Client Password'
smtp='SMTP Client Address'
```

## License

[MIT](https://choosealicense.com/licenses/mit/)
