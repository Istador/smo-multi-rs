mod client;
mod cmds;
mod coordinator;
mod guid;
mod net;
mod server;
mod settings;
mod types;

use crate::{net::connection::Connection, types::Result};
use bytes::BytesMut;
use clap::Parser;
use client::{Client, ClientMap};
use cmds::{Cli, Command};
use coordinator::Coordinator;
use guid::Guid;
use net::{encoding::Decodable, Packet};
use server::Server;
use settings::SyncSettings;
use std::{
    collections::{HashMap, HashSet},
    io::Write,
    net::{Shutdown, SocketAddr},
    sync::Arc,
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    join,
    sync::{mpsc, RwLock},
};

use crate::{cmds::ServerCommand, types::EncodingError};

#[tokio::main]
async fn main() -> Result<()> {
    let bind_addr = "127.0.0.1:61884".parse().unwrap();
    let (to_coord, server, coordinator) = create_default_server();
    let serv_task = tokio::task::spawn(server.listen_for_clients(bind_addr));
    let coord_task = tokio::task::spawn(coordinator.handle_commands());
    let parser_task = tokio::task::spawn(parse_commands(to_coord));

    let _results = tokio::join!(serv_task, coord_task, parser_task);
    Ok(())
}

fn create_default_server() -> (mpsc::Sender<Command>, Server, Coordinator) {
    env_logger::init();
    let (to_coord, from_clients) = mpsc::channel(100);
    let settings = SyncSettings::default();
    let server = Server {
        settings: settings.clone(),
        to_coord: to_coord.clone(),
    };
    let coordinator = Coordinator {
        shine_bag: Arc::new(RwLock::new(HashSet::default())),
        from_clients,
        settings,
        clients: ClientMap::new(),
        to_clients: HashMap::new(),
    };
    (to_coord, server, coordinator)
}

async fn parse_commands(mut to_coord: mpsc::Sender<Command>) -> Result<()> {
    loop {
        let command_result = parse_command(&mut to_coord).await;

        if let Err(e) = command_result {
            println!("{}", e)
        }
    }
}

async fn parse_command(to_coord: &mut mpsc::Sender<Command>) -> Result<()> {
    let task = tokio::task::spawn_blocking(|| async { read_command() });
    let command: Cli = join!(task).0?.await?;

    Ok(to_coord.send(Command::Cli(command.cmd)).await?)
}

fn read_command() -> Result<Cli> {
    let mut input = "> ".to_string();

    print!("{}", input);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut input)?;
    let input = input.trim().split(' ');
    let cli = Cli::try_parse_from(input)?;
    Ok(cli)
}

#[tokio::test]
async fn client_connect() -> Result<()> {
    let addr = "127.0.0.1:61884".parse().unwrap();
    let (to_coord, server, coordinator) = create_default_server();
    let serv_task = tokio::task::spawn(server.listen_for_clients(addr));
    let coord_task = tokio::task::spawn(coordinator.handle_commands());

    let client = tokio::spawn(async move { fake_client(addr).await });

    let _ = tokio::join!(client);
    let cmd = Command::Server(ServerCommand::Shutdown);
    to_coord.send(cmd).await?;
    let _ = tokio::join!(serv_task, coord_task);
    Ok(())
}

async fn fake_client(addr: SocketAddr) -> Result<()> {
    let socket = tokio::net::TcpSocket::new_v4()?;
    log::debug!("Connecting to server");
    let conn = socket.connect(addr).await?;
    let mut conn = Connection::new(conn);
    log::debug!("Connected to server");

    log::debug!("Reading data from server");
    let mut result: Result<Packet> = Err(EncodingError::CustomError.into());
    while let Err(_) = result {
        let result = conn.read_packet().await;
        log::debug!("Packet: {:?}", result);
        // let read = conn.read(&mut buff).await?;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    log::debug!("Read data from server");
    log::debug!("Read packet: {:?}", result);
    Ok(())
}
