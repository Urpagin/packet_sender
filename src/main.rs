use std::{io::Write, net::TcpStream};

use log::{error, info};

const ADDRESS: &str = "127.0.0.1:25565";

fn main() {
    init_logging();

    println!(
        "\n\nTHIS PROGRAM ALLOWS YOU TO SEND BYTES OF DATA TO A SERVER.\nHOWEVER, THIS PROGRAM DOES NOT SHOW THE SERVER RESPONSE.\n\n"
    );

    info!("Server address: {ADDRESS}\n");

    if do_keep_connection_alive() {
        let mut conn: TcpStream = connect(ADDRESS).expect("Failed to create a connection");
        loop {
            print!("\n ---------------------------------------------------------------- \n\n");
            let packet: &[u8] = &get_user_packet()[..];

            if packet.is_empty() {
                continue;
            }

            conn.write_all(packet).expect("Failed to send packet");
            info!("Sent: {:?}", packet);
        }
    } else {
        loop {
            let mut conn: TcpStream = connect(ADDRESS).expect("Failed to create a connection");

            print!("\n ---------------------------------------------------------------- \n\n");
            let packet: &[u8] = &get_user_packet()[..];

            if packet.is_empty() {
                continue;
            }

            conn.write_all(packet).expect("Failed to send packet");
            info!("Sent: {:?}", packet);
        }
    }
}

fn init_logging() {
    let mut builder = env_logger::Builder::new();

    builder.filter_level(log::LevelFilter::Debug);

    builder.init();
}

fn connect(address: &str) -> std::io::Result<TcpStream> {
    let stream = TcpStream::connect(address)?;
    info!("Connected to server");

    Ok(stream)
}

fn do_keep_connection_alive() -> bool {
    loop {
        print!("Would you like to:\n[1]: Keep a single connection open\n[2]: Open a new connection each time\n-> ");
        std::io::stdout().flush().expect("Failed to flush stdout");

        let mut buf = String::new();
        let _ = std::io::stdin()
            .read_line(&mut buf)
            .expect("Failed to get user input");
        buf = buf.trim().to_string();

        if buf == "1" {
            return true;
        } else if buf == "2" {
            return false;
        } else {
            continue;
        }
    }
}

fn get_user_packet() -> Vec<u8> {
    loop {
        print!("Please input your packet e.g.: ('1 2 3')\n-> ");
        std::io::stdout().flush().expect("Failed to flush stdout");

        match get_user_bytes() {
            Ok(bytes) => return bytes,
            Err(_) => {
                error!("Invalid input, please try again.");
                continue;
            }
        }
    }
}

fn get_user_bytes() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;

    let bytes = buf
        .trim()
        .split(' ')
        .filter_map(|e| e.parse::<u8>().ok())
        .collect::<Vec<u8>>();

    Ok(bytes)
}
