use clap::{App, AppSettings, Arg};
use reqwest::blocking;

mod auth;
mod lib;

fn main() {
    let matches = App::new("jc")
        .about("A cli tool to interact with julie's database")
        .version("0.0.3")
        .author("Stackmate.Network")
        .subcommand(
            App::new("info")
                .about("Provides live stats on julie.")
                .display_order(0)
        )
        .subcommand(
            App::new("client")
                .about("Interacts with the client database")
                .display_order(1)
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("register").about("Registers a new client")
                )
                .subcommand(
                    App::new("delete").about("Deletes and existing client")
                    .arg(
                        Arg::with_name("uid")
                        .required(true)
                        .help("The uid of the client to delete."),
                ))
                .subcommand(App::new("list").about("Lists all existing client uids")),

        )
        .subcommand(
            App::new("service")
                .about("Interacts with the service database")
                .display_order(1)
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("register").about("Registers a new service")
                    .arg(
                        Arg::with_name("name")
                        .required(true)
                        .help("The name of the service being registered."),
                    )
                )
                .subcommand(
                    App::new("delete").about("Deletes and existing service")
                    .arg(
                        Arg::with_name("name")
                        .required(true)
                        .help("The name of the client to delete."),
                ))
                .subcommand(App::new("list").about("Lists all existing service sids")),

        )
        .get_matches();
  


    match matches.subcommand() {
        ("info", Some(_)) => {

            match reqwest::blocking::get("http://localhost:3030/auth/health"){
                Ok(response)=>{
                    println!("{:?}",  response.text().unwrap());
                }
                Err(_)=>{
                    println!("jd offline.");
                }
            }   
        
           
        }
        ("client", Some(push_matches)) => {
            match push_matches.subcommand() {
                ("register", Some(_)) => {
                    let client = auth::client::ClientAuth::new();
                    println!("{:#?}",client);
                }
                ("list", Some(_)) => {
                    let clients = auth::client::get_uid_indexes();
                    println!("{:#?}",clients);
                }
                ("delete", Some(args)) => {
                    match auth::client::ClientAuth::read(&args.value_of("uid").unwrap()){
                        Some(client)=>{
                            let status = client.delete();
                            println!("{:#?}",status);
                        }
                        None=>println!("Provided UID is not registered.")
                    };
                    
                }
                _ => unreachable!(),
            }
        }
       
        ("service", Some(push_matches)) => {
            match push_matches.subcommand() {
                    ("register", Some(args)) => {
                        let service = auth::service::ServiceIdentity::new(&args.value_of("name").unwrap());
                        println!("{:#?}",service);
                    }
                    ("list", Some(_)) => {
                        let services = auth::service::get_name_indexes();
                        println!("{:#?}",services);
                    }
                    ("delete", Some(args)) => {
                        match auth::service::ServiceIdentity::init(&args.value_of("name").unwrap()){
                            Some(service)=>{
                                let status = service.delete();
                                println!("{:#?}",status);
                            }
                            None=>println!("Provided name is not registered.")
                        };
                        
                    }
                _ => unreachable!(),
            }
        }
       
        ("",None) => println!("No subcommand was used"), 
        _ => unreachable!(),
    }

}

