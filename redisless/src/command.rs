use crate::command::Command::{Error, NotSupported};
use crate::protocol::RESP;

type Key = Vec<u8>;
type Value = Vec<u8>;
type Message = &'static str;

#[derive(Debug, PartialEq)]
pub enum Command {
    Set(Key, Value),
    Get(Key),
    Del(Key),
    Info,
    Ping,
    Quit,
    Error(Message),
    NotSupported(String),
}

fn get_bytes_vec(resp: Option<&RESP>) -> Option<Vec<u8>> {
    match resp {
        Some(RESP::String(x)) | Some(RESP::BulkString(x)) => Some(x.to_vec()),
        _ => None,
    }
}

impl Command {
    pub fn parse(v: Vec<RESP>) -> Self {
        match v.first() {
            Some(RESP::BulkString(op)) => match *op {
                b"SET" | b"set" | b"Set" => {
                    if v.len() != 3 {
                        return Error("wrong number of arguments for 'SET' command");
                    }

                    if let Some(arg1) = get_bytes_vec(v.get(1)) {
                        if let Some(arg2) = get_bytes_vec(v.get(2)) {
                            return Command::Set(arg1, arg2);
                        }
                    }

                    Error("wrong number of arguments for 'SET' command")
                }
                b"GET" | b"get" | b"Get" => {
                    if v.len() != 2 {
                        return Error("wrong number of arguments for 'GET' command");
                    }

                    if let Some(arg1) = get_bytes_vec(v.get(1)) {
                        return Command::Get(arg1);
                    }

                    Error("wrong number of arguments for 'GET' command")
                }
                b"DEL" | b"del" | b"Del" => {
                    if v.len() != 2 {
                        return Error("wrong number of arguments for 'DEL' command");
                    }

                    if let Some(arg1) = get_bytes_vec(v.get(1)) {
                        return Command::Del(arg1);
                    }

                    Error("wrong number of arguments for 'DEL' command")
                }
                b"INFO" | b"info" | b"Info" => Command::Info,
                b"PING" | b"ping" | b"Ping" => Command::Ping,
                b"QUIT" | b"quit" | b"Quit" => Command::Quit,
                cmd => NotSupported(format!(
                    "Command '{}' is not supported",
                    std::str::from_utf8(cmd).unwrap()
                )),
            },
            _ => Error("Invalid command to parse"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::command::Command;
    use crate::protocol::RESP;

    #[test]
    fn set_command() {
        let commands = vec![b"SET", b"set"];

        for cmd in commands {
            let resp = vec![
                RESP::BulkString(cmd),
                RESP::BulkString(b"mykey"),
                RESP::BulkString(b"value"),
            ];

            let command = Command::parse(resp);

            assert_eq!(command, Command::Set(b"mykey".to_vec(), b"value".to_vec()));
        }
    }
}
