use clap::{App, AppSettings, Arg};

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
                    App::new("register") // Subcommands can have their own subcommands,
                        // which in turn have their own subcommands
                        .about("Registers a new client")
                        .arg(
                            Arg::with_name("repo")
                                .required(true)
                                .help("The remote repo to push things to"),
                        ),
                )
                .subcommand(App::new("delete").about("Deletes and existing client"))
                .subcommand(App::new("list").about("Lists all existing clients")),

        )
        .subcommand(
            App::new("service")
                .about("Interacts with the service database")
                .display_order(1)
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("register") // Subcommands can have their own subcommands,
                        // which in turn have their own subcommands
                        .about("Registers a new service")
                        .arg(
                            Arg::with_name("repo")
                                .required(true)
                                .help("The remote repo to push things to"),
                        ),
                )
                .subcommand(App::new("delete").about("Deletes and existing service"))
                .subcommand(App::new("list").about("Lists all existing services")),

        )
        .get_matches();
  


    match matches.subcommand() {
        ("info", Some(clone_matches)) => {
            println!("Asked for info. Got none right now :(");
        }
        ("client", Some(push_matches)) => {
            match push_matches.subcommand() {
                ("register", Some(remote_matches)) => {
                    println!("'jcli client register' was called");
                }
                ("list", Some(_)) => {
                    println!("'jcli client list' was called");
                }
                ("delete", Some(_)) => {
                    println!("'jcli client delete' was called");
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
