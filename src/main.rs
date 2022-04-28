use mongodb::Collection;
pub use mongodb::{Client, options::ClientOptions};
pub use mongodb::bson::{doc, Document};

pub use std::error::Error;
pub use tokio;
pub use std::io;

mod Obj;
use Obj::{User,status};

use terminal_menu::{menu, label, button, run, mut_menu};

#[tokio::main]
#[allow(unused)]
async fn main() -> Result<(), Box<dyn Error>> {

    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    client_options.app_name = Some("My App".to_string());
    let client = Client::with_options(client_options)?;
    let db = client.database("ObverseDatabase");
    let collection = db.collection::<Document>("Rust");
    
    let menu = menu(vec![

        // label:
        //  not selectable, usefule as a title, separator, etc...
        label("----------------------"),
        label("terminal-menu"),
        label("use wasd or arrow keys"),
        label("enter to select"),
        label("'q' or esc to exit"),
        label("-----------------------"),

        // button:
        //  exit the menu
        button("Create"),
        button("Read"),
        button("Update"),
        button("Delete"),
        button("Exit"),

    ]);
    run(&menu);

    println!("Selected: {}",mut_menu(&menu).selected_item_name());

   let scelta:i32=0;

   if mut_menu(&menu).selected_item_name().eq("Create"){

        let mut  username:String = String::new();
        let mut  email:String = String::new();
        let mut password:String = String::new();
        let mut s:String = String::new();
        println!("Insert Username");
        username = readln(username);
        println!("Insert Email");
        email = readln(email);
        println!("Insert Password");
        password = readln(password);

        let username:String = String::from(username.trim());
        let email:String = String::from(email.trim());
        let password:String = String::from(password.trim());

        println!("1. Online");
        println!("2. Offline");

        s = readln(s);
        let s :i32 = s.trim().parse().expect("Dis");

        let mut status:status = status::def;
        match s {
            1 =>  status = status::online,
            2 =>  status = status::offline, 
            _ =>println!("errore"),
        };

        let user = User{
            username:username,
            email:email,
            password:password,
            status:status,
        };

            let mut st:String = String::new();

            match user.status {
                status::online => st = String::from("online"),
                status::offline => st = String::from("offline"),
                status::def => st= String::from("def"),
            }
        
            let u = doc! {"username":&user.username,"email":&user.email, "password":&user.password,"Status":&st};
        
        collection.insert_one(u, None).await?;
        println!("Data entered successfully");
            
    }else if mut_menu(&menu).selected_item_name().eq("Read"){

        let mut email:String = String::new();
        println!("Insert Email of the old user");
        let mut email = readln(email);
        let mut email = email.trim();
    
        let result = collection.find_one(
            doc! {
               "email": email
            },
            None,
         ).await?;

         println!("{:#?}",result);

    }else if mut_menu(&menu).selected_item_name().eq("Update") {

        let mut email:String = String::new();

        println!("Insert Email of the old user");
            let mut email = readln(email);

            let mut email = email.trim();

        println!("Insert the new Email");
        let mut new_email:String = String::new();
        let mut new_email = readln(new_email);
        let mut new_email = new_email.trim();

        let update_result = collection.update_one(

            doc! {
                 "email": email
            },
            doc! {
               "$set": { "email": new_email }
            },
            None,
         ).await?;
         println!("Updated {} document", update_result.modified_count);

    }else if mut_menu(&menu).selected_item_name().eq("Delete"){

        let mut email:String = String::new();

        println!("Insert Email of the old user");
            let mut email = readln(email);

            let mut email = email.trim();

            let delete_result = collection.delete_one(
                doc! {
                   "email": email
                },
                None,
             ).await?;
             println!("Deleted {} documents", delete_result.deleted_count);

    }


Ok(())
}

fn readln( mut var:String ) ->String{

    io::stdin()
    .read_line(&mut var)
   .expect("Errore");

    return  var 
}


//#[tokio::main]
pub async fn create(user:&User,collection:Collection<Document>)-> mongodb::error::Result<()>{

    Ok(())
}

#[tokio::main]
pub async fn update(email:String, update:&User ,collection:mongodb::Collection<Document>) -> mongodb::error::Result<()>{

    let filter = doc! {
        "Email":email
    };

    let mut st:String = String::new();

    match update.status {
        status::online => st = String::from("online"),
        status::offline => st = String::from("offline"),
        status::def => st= String::from("def"),
    }

    let update:Document = doc! {"Username":&update.username,"Email":&update.email, "Password":&update.password, "Status":st};

    collection.update_one(filter , update, None).await.expect("Error Uploading");
    Ok(())
}



#[tokio::main]
pub async fn read(){

}

/*

// useful function to see database names

pub fn view(){
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
}

*/
