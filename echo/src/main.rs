use std::io::{self, Read, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    src: String,
    dest: String,
    #[serde(flatten)]
    body: Body
}

#[derive(Serialize, Deserialize, Debug)]
struct Body {
    #[serde(rename = "type")]
    body_type: String,
    msg_id: Option<u32>,
    in_reply_to: Option<u32>,
    echo: Option<String>
}

fn main() -> io::Result<()> {
    loop {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        let my_data: Message = serde_json::from_str(&buffer)?;
        println!("Deserialized data: {:?}", my_data);

        if my_data.body.body_type == "echo" {
            let body = Body {
                body_type: String::from("echo_ok"),
                msg_id: my_data.body.msg_id,
                in_reply_to: my_data.body.msg_id,
                echo: my_data.body.echo,
            };

            let outmsg = Message {
                src: my_data.dest,
                dest: my_data.src,
                body: body,
            };

            match serde_json::to_string(&outmsg) {
                Ok(json_string) => {
                    println!("Serialized JSON: {}", json_string);
                    let mut stdout = io::stdout();
                    stdout.write_all(json_string.as_bytes()).unwrap();
                    stdout.flush().unwrap();
                },
                Err(e) => eprintln!("Serialization error: {}", e),
            }


        }
    }
}
