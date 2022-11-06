use ssh2::Session;
use std::io::prelude::*;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Give remote host name or IP address as parameters e.g. rpi1 rpi2 rpi3 ...");

        std::process::exit(1);
    }

    let mut sessions = vec![];

    for rpi in args[1..].iter() {
        println!("Argument {rpi}");
        let sess = ssh_session("pi", format!("{rpi}:22"));
        sessions.push(sess);
    }

    for fut in sessions {
        fut.await.unwrap();
    }
}

async fn ssh_session(user: &str, host: String) -> Result<(), String> {
    println!("Connecting to {host} as user {user}");
    let tcp = match TcpStream::connect(host).await {
        Ok(v) => v,
        Err(err) => {
            println!("connect error: {err}");
            return Err("Could not connect to remote host".into());
        }
    };
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_agent(user).unwrap();

    let mut channel = sess.channel_session().unwrap();
    channel.exec("ls").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close().unwrap();
    println!("{}", channel.exit_status().unwrap());
    Ok(())
}
