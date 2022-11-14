use easy_repl::{Repl, CommandStatus, command};

mod comm;

fn main() {

    let mut com = comm::Comm::new();

    let mut repl = Repl::builder()
    .add("namespace", command! {
        "Set namespace for keys",
        (name: String) => |name| {
            println!("Hello {}!", name);
            com.namespace = name;
            Ok(CommandStatus::Done)
            
        }
    })
    .add("host", command! {
        "set server host",
        (host: String) => |host| {
            println!("Setting host to {}",host);
            com.host = host;
            Ok(CommandStatus::Done)
        }
    })
    .add("set", command! {
        "set key / value",
        (key: String, value: String) => |_key, value| {
            let mut result = com.set( _key, value);
            println!("OK. {}", value);
            Ok(CommandStatus::Done)
        }
    })
    .add("get", command! {
        "get value",
        (key: String) => |_key| {
            let mut result = com.get( _key);
            println!("{}", result);
            Ok(CommandStatus::Done)
        }
    })
    
    .build().expect("Failed to create repl");
    repl.run().expect("Critical REPL error");
}

