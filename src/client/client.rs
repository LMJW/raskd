use clap::{App, Arg};
use raskd::models::{Incoming, Outgoing, Status};
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
                .arg(
                    Arg::with_name("completed")
                        .short("c")
                        .help("list all completed tasks"),
                )
                .arg(Arg::with_name("task").short("t").help("list all tasks"))
                .arg(Arg::with_name("timer").short("r").help("list all timers"))
                .arg(Arg::with_name("todo").short("d").help("list all todos")),
        )
        .subcommand(
            App::new("startin")
                .about("[UNIMPLEMENTED]: start a task in X minues")
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
                .about("[UNIMPLEMENTED]: pause a current running task or timer")
                .arg(
                    Arg::with_name("task_id_or_name")
                        .help("the task id (integer) or the name (string) to stop")
                        .required(true),
                ),
        )
        .subcommand(
            App::new("stop")
                .about("stop a current running task or timer")
                .arg(
                    Arg::with_name("task_id_or_name")
                        .help("the task id (integer) or the name (string) to stop")
                        .required(true),
                ),
        )
        .subcommand(
            App::new("timer")
                .about("[UNIMPLEMENTED]: start a background timer")
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
                        .about("[UNIMPLEMENTED]: pause a background active timer")
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
            App::new("todo")
                .about("[UNIMPLEMENTED]: add a todo in the list")
                .arg(
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
                Err(e) => eprintln!("{:#?}", e),
            }
        }
        ("ls", Some(sub)) => {
            let path = match sub.args.is_empty() {
                false => {
                    let mut vals = Vec::<String>::new();
                    for key in sub.args.keys() {
                        vals.push(format!("{}=true", key));
                    }
                    let params = vals.join("&");
                    format!("{}/{}?{}", url, "task", params)
                }
                true => format!("{}/{}", url, "task"),
            };

            match client.get(&path).send() {
                Ok(res) => {
                    fmtjs::fmt_many(res.json::<Vec<Outgoing>>().unwrap());
                }
                Err(e) => eprintln!("{:#?}", e),
            }
        }
        ("stop", Some(sub)) => {
            let path = format!("{}/{}", url, "task");
            let val = sub.value_of("task_id_or_name").unwrap().to_string();
            let id = val.parse::<i64>();
            let json = match id {
                Ok(id) => Incoming::Update {
                    id: Some(id),
                    name: None,
                    action: Status::Stop,
                },
                Err(_) => Incoming::Update {
                    id: None,
                    name: Some(val),
                    action: Status::Stop,
                },
            };
            match client.patch(&path).json(&json).send() {
                Ok(res) => {
                    fmtjs::fmt_one(res.json::<Outgoing>().unwrap());
                }
                Err(e) => eprintln!("{:#?}", e),
            }
        }
        _ => {
            eprintln!("command unimplemented");
            eprintln!("use `-h` to check other options");
        }
    };
}
