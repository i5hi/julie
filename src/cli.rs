// Working with subcommands is simple. There are a few key points to remember when working with
// subcommands in clap. First, s are really just Apps. This means they can have their own
// settings, version, authors, args, and even their own subcommands. The next thing to remember is
// that subcommands are set up in a tree like hierarchy.
//
// An ASCII art depiction may about explain this better. Using a fictional version of git as the demo
// subject. Imagine the following are all subcommands of git (note, the author is aware these aren't
// actually all subcommands in the real git interface, but it makes explanation easier)
//
//            Top Level App (git)                         TOP
//                           |
//    -----------------------------------------
//   /             |                \          \
// clone          push              add       commit      LEVEL 1
//   |           /    \            /    \       |
//  url      origin   remote    ref    name   message     LEVEL 2
//           /                  /\
//        path            remote  local                   LEVEL 3
//
// Given the above fictional subcommand hierarchy, valid runtime uses would be (not an all inclusive
// list):
//
// $ git clone url
// $ git push origin path
// $ git add ref local
// $ git commit message
//
// Notice only one command per "level" may be used. You could not, for example, do:
//
// $ git clone url push origin path
//
// It's also important to know that subcommands each have their own set of matches and may have args
// with the same name as other subcommands in a different part of the tree hierarchy (i.e. the arg
// names aren't in a flat namespace).
//
// In order to use subcommands in clap, you only need to know which subcommand you're at in your
// tree, and which args are defined on that subcommand.
//
// Let's make a quick program to illustrate. We'll be using the same example as above but for
// brevity sake we won't implement all of the subcommands, only a few.

use clap::{App, AppSettings, Arg};

fn main() {
    // this is where you define the top level app spec (currently called git)
    let matches = App::new("jcli")
        .about("A cli tool to interact with julie's database")
        .version("1.0")
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
  

    // This is where we handle each sub command

    // `ArgMatches::subcommand` returns a tuple of both the name and matches
    match matches.subcommand() {
        ("info", Some(clone_matches)) => {
            // Now we have a reference to clone's matches
            println!("Asked for info. Got none right now :(");
        }
        // push is an example for handling nested subcommands
        ("client", Some(push_matches)) => {
            // Now we have a reference to push's matches
            match push_matches.subcommand() {
                ("register", Some(remote_matches)) => {
                    // Now we have a reference to `push remote`'s matches
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
            // Now we have a reference to push's matches
            match push_matches.subcommand() {
                ("register", Some(remote_matches)) => {
                    // Now we have a reference to `push remote`'s matches
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
       
        ("",None) => println!("No subcommand was used"), // If no subcommand was used it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }

}
