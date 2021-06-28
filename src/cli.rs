use clap::{App, AppSettings, Arg};
mod auth;
mod lib;

fn main() {
    let matches = App::new("jcli")
        .about("A cli tool to interact with julie's database")
        .version("0.0.3")
        .author("Stackmate.Network")
        .subcommand(
            App::new("info")
                .about("Provides live stats on julie.")
                .display_order(0)
                .arg(Arg::with_name("repo").help("The repo to clone").required(true)),
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
                )
                .subcommand(App::new("delete").about("Deletes and existing service"))
                .subcommand(App::new("list").about("Lists all existing services")),

        )
        .get_matches();
  


    match matches.subcommand() {
        ("info", Some(_)) => {
            let clients = auth::client::get_uid_indexes();
            println!("{:#?}",clients);
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
                ("register", Some(remote_matches)) => {
                    println!("'jcli service register' was called");
                }
                ("list", Some(_)) => {
                    println!("'jcli service list' was called");
                }
                ("delete", Some(_)) => {
                    println!("'jcli service delete' was called");
                }
                _ => unreachable!(),
            }
        }
       
        ("",None) => println!("No subcommand was used"), 
        _ => unreachable!(),
    }

}
