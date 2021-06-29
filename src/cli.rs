#![allow(dead_code)]
use clap::{App, AppSettings, Arg};
use std::str::FromStr;

mod auth;
mod lib;

use crate::lib::aes;
/**
 * 
 * name: "\x1b[0;1mpikachu\x1b[0m"
version: "\x1b[0;94m0.4.6\x1b[0m"
author: dev@stackmate.net
about: "\x1b[0;94msatsbank.io client; with sovereign bitcoin tools\x1b[0m"
before_help: "\x1b[0;92m*******************************************vires***************************************************\x1b[0m"
after_help:  "\x1b[0;92m****************************************in.numeris**************************************************\x1b[0m"
usage: pikachu [SUBCOMMAND] [ARGS]

 */
fn main() {
    let matches = App::new("\x1b[0;92mjc\x1b[0m")
        .about("\x1b[0;94mJulie admin tools.\x1b[0m")
        .version("\x1b[0;1m0.0.9\x1b[0m")
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
                    .arg(
                        Arg::with_name("key")
                        .required(true)
                        .help("Shared key to use to sign tokens for this service"),
                    )
                )
                .subcommand(
                    App::new("delete").about("Deletes and existing service")
                    .arg(
                        Arg::with_name("name")
                        .required(true)
                        .help("The name of the client to delete."),
                    )
                )
                .subcommand(App::new("list").about("Lists all existing service sids")),

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
                        let service = auth::service::ServiceIdentity::new(&args.value_of("name").unwrap(),&args.value_of("key").unwrap());
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
        ("util", Some(push_matches)) => {
            match push_matches.subcommand() {
                    ("random", Some(args)) => {
                        let encoding_str = match args.value_of("encoding"){
                            Some(string)=>string,
                            None=>"hex"
                        };

                        let encoding = match aes::Encoding::from_str(encoding_str){
                            Ok(encoding)=>{encoding},
                            Err(e)=>{aes::Encoding::Hex}
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

