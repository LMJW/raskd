use clap::{App, Arg};
use raskd::models::{Incoming, Outgoing};
mod fmtjs;

fn main() {
    let cmds = App::new("RASK command line tool")
        .version("0.1.0")
        .author("LMJW")
        .about("command line tool to do task management")
        .subcommand(
            App::new("start")
                .about("start a timed task running on the background")
                .arg(
                    Arg::with_name("task_name")
                        .help("set the name of the task")
                        .required(true),
                ),
        )
        .subcommand(
            App::new("ls")
                .about("list the current running tasks")
                .arg(Arg::with_name("all").short("a").help("list all"))
                .arg(Arg::with_name("task").short("t").help("list all tasks"))
                .arg(Arg::with_name("timer").short("r").help("list all timers"))
                .arg(Arg::with_name("todo").short("d").help("list all todos")),
        )
        .subcommand(
            App::new("startin")
                .about("start a task in X minues")
                .arg(
                    Arg::with_name("TIME")
                        .help("set the time before start. e.g. (5m, 10s, 1h)")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("task_name")
                        .help("set the name of the task")
                        .required(true),
                ),
        )
        .subcommand(
            App::new("pause")
                .about("pause a current running task or timer. [UNIMPLEMENTED]")
                .arg(Arg::with_name("task_id").help("the task id (integer) to stop"))
                .arg(Arg::with_name("task_name").help("the name (string) of the task")),
        )
        .subcommand(
            App::new("stop")
                .about("stop a current running task or timer")
                .arg(Arg::with_name("task_id").help("the task id (integer) to stop"))
                .arg(Arg::with_name("task_name").help("the name (string) of the task")),
        )
        .subcommand(
            App::new("timer")
                .about("start a background timer")
                .subcommand(
                    App::new("start")
                        .about("start a background active timer")
                        .arg(
                            Arg::with_name("timer name")
                                .help("the name of the timer")
                                .required(true),
                        ),
                )
                .subcommand(
                    App::new("pause")
                        .about("pause a background active timer")
                        .arg(
                            Arg::with_name("timer name or id")
                                .help("the name(string) or id(int) of the timer")
                                .required(true),
                        ),
                )
                .subcommand(
                    App::new("stop")
                        .about("stop a background active timer")
                        .arg(
                            Arg::with_name("timer name or id")
                                .help("the name(string) or id(int) of the timer")
                                .required(true),
                        ),
                ),
        )
        .subcommand(
            App::new("todo").about("add a todo in the list").arg(
                Arg::with_name("todo name")
                    .help("the name of todo task")
                    .required(true),
            ),
        )
        .get_matches();

    let url = "http://localhost:22022";
    let client = reqwest::blocking::Client::new();

    match cmds.subcommand() {
        ("start", Some(sub)) => {
            let name = sub.value_of("task_name").unwrap().to_string();
            let tasktype = "task".to_string();
            let json = Incoming::Create {
                name: name,
                tasktype,
            };
            let path = format!("{}/{}", url, "task");
            match client.post(&path).json(&json).send() {
                Ok(res) => {
                    fmtjs::fmt_one(res.json::<Outgoing>().unwrap());
                }
                Err(e) => unimplemented!(),
            }
        }
        ("ls", Some(sub)) => {
            let path = format!("{}/{}", url, "task");
            match client.get(&path).send() {
                Ok(res) => {
                    // eprintln!("{:#?}", res.json::<Vec<Outgoing>>().unwrap());
                    fmtjs::fmt_many(res.json::<Vec<Outgoing>>().unwrap());
                }
                Err(e) => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    };
}
