#![allow(dead_code)]
use clap::{App, AppSettings, Arg};
use std::str::FromStr;

mod lib;
mod auth;
mod storage;

use crate::storage::interface::{JulieStorage,JulieDatabase,JulieDatabaseItem};
// use crate::storage::sled::{SledDb};
use crate::storage::vault::{VaultStorage, init as init_vault};

use crate::lib::aes;

fn main() {
    let mut client_storage = init_vault(JulieDatabase::Client).unwrap();
    let mut service_storage = init_vault(JulieDatabase::Service).unwrap();

    let matches = App::new("\x1b[0;92mjc\x1b[0m")
        .about("\x1b[0;94mJulie admin tools.\x1b[0m")
        .version("\x1b[0;1m0.0.9\x1b[0m")
        .author("Stackmate.Network")
        .subcommand(
            App::new("info")
                .about("Provide live stats on julie daemon.")
                .display_order(0)
        )
        .subcommand(
            App::new("client")
                .about("Interacts with the client database")
                .display_order(1)
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("register").about("Register a new client")
                )
                .subcommand(
                    App::new("read").about("Read a client")
                    .arg(
                        Arg::with_name("uid")
                        .required(true)
                        .help("The uid of the client to read."),
                ))
                .subcommand(
                    App::new("delete").about("Delete a client")
                    .arg(
                        Arg::with_name("uid")
                        .required(true)
                        .help("The uid of the client to delete."),
                ))
                .subcommand(App::new("list").about("List all existing client uids")),

        )
        .subcommand(
            App::new("service")
                .about("Interacts with the service database")
                .display_order(1)
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("register").about("Register a new service")
                    .arg(
                        Arg::with_name("name")
                        .required(true)
                        .help("The name of the service being registered."),
                    )
                    .arg(
                        Arg::with_name("key")
                        .required(true)
                        .help("Shared key to use to sign tokens for this service"),
                    )
                )
                .subcommand(
                    App::new("read").about("Read a service")
                    .arg(
                        Arg::with_name("name")
                        .required(true)
                        .help("The name of the service to read."),
                    )
                )
                .subcommand(
                    App::new("delete").about("Delete a service")
                    .arg(
                        Arg::with_name("name")
                        .required(true)
                        .help("The name of the service to delete."),
                    )
                )
                .subcommand(
                    App::new("update").about("Update an existing service shared secret key")
                    .arg(
                        Arg::with_name("name")
                        .required(true)
                        .help("The name of the service to update."),
                    )
                    .arg(
                        Arg::with_name("key")
                        .required(true)
                        .help("New key to update."),
                    )
                )
                .subcommand(App::new("list").about("List all existing service sids")),

        )
        .subcommand(
            App::new("util")
                .about("Toys")
                .display_order(1)
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("random").about("Create a random key")
                    .arg(
                        Arg::with_name("encoding")
                        .required(false)
                        .help("'hex' (default), 'base32' or 'base64'"),
                    )
                )
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
                    assert!(client_storage.create(JulieDatabaseItem::Client(client.clone())).unwrap());
                    println!("{:#?}",client);
                }
                ("list", Some(_)) => {
                    // let clients = auth::client::get_uid_indexes();
                    println!("NOT IMPLEMENTED")
                }
                ("read", Some(args)) => {
                    // let clients = auth::client::get_uid_indexes();
                    println!("{:#?}", &args.clone().value_of("uid").unwrap());

                    match client_storage.read(JulieDatabase::Client,&args.value_of("uid").unwrap()){
                        Ok(item)=>match item{
                            JulieDatabaseItem::Client(client)=>{
                                println!("{:#?}", client)
                            }
                            _=>{
                                panic!("WEIRD ERROR - SHOULD NOT HAPPEN")

                            }
                        }
                        Err(e)=>{
                            println!("{}",e)
                        }
                    }
                }
                ("delete", Some(args)) => {
                    let status = client_storage.delete(JulieDatabase::Client,&args.value_of("uid").unwrap()).unwrap();
                    println!("{}",status)
        
                }
                _ => unreachable!(),
            }
        }
        ("service", Some(push_matches)) => {
            match push_matches.subcommand() {
                    ("register", Some(args)) => {
                        match service_storage.read(JulieDatabase::Service,&args.value_of("name").unwrap()){
                            Ok(item)=> match item{
                                JulieDatabaseItem::Service(service)=>{
                                    println!("Service {} exists.", service.name);

                                }
                                _=>{
                                    panic!("WEIRD ERROR - SHOULD NOT HAPPEN");
                                }
                            }
                            Err(_)=>{
                                let service = auth::service::ServiceIdentity::new(&args.value_of("name").unwrap(),&args.value_of("key").unwrap());
                                  
                                match service_storage.create(JulieDatabaseItem::Service(service.clone())){
                                    Ok(result)=> println!("{:#?}",result),
                                    Err(e)=>println!("{:#?}",e)
                                };

                                  
                            }
                        };
                   

                        
                    }
                    ("list", Some(_)) => {
                        println!("NOT IMPLEMENTED")

                    }                    
                    ("update", Some(_)) => {
                        println!("NOT IMPLEMENTED")

                    }
                    ("delete", Some(args)) => {
                        let status = service_storage.delete(JulieDatabase::Service,&args.value_of("name").unwrap()).unwrap();
                        println!("{}",status)
                        
                    }
                    ("read", Some(args)) => {
                        println!("{:#?}", &args.clone().value_of("name").unwrap());

                        match service_storage.read(JulieDatabase::Service,&args.value_of("name").unwrap()){
                            Ok(item)=>match item{
                                JulieDatabaseItem::Service(service)=>{
                                    println!("{:#?}", service)
                                }
                                _=>panic!("WEIRD ERROR - SHOULD NOT HAPPEN")
                            }
                            Err(e)=>{
                                println!("{}",e)
                            }
                        }                      
                        
                    }
                _ => unreachable!(),
            }
        }
        ("util", Some(push_matches)) => {
            match push_matches.subcommand() {
                    ("random", Some(args)) => {
                        let encoding_str = match args.value_of("encoding"){
                            Some(string)=>string,
                            None=>"hex"
                        };

                        let encoding = match aes::Encoding::from_str(encoding_str){
                            Ok(encoding)=>{encoding},
                            Err(_)=>{aes::Encoding::Hex}
                        };
                        let random = aes::keygen(encoding);
                        println!("{:#?}",random);
                    }
                _ => unreachable!(),
            }
        }
        ("",None) => println!("No subcommand was used. try `jc help`."), 
        _ => unreachable!(),
    }

}

