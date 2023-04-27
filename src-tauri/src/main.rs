// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod db;

use twitch_irc::{login::StaticLoginCredentials, ClientConfig, SecureTCPTransport, TwitchIRCClient, message};


#[tokio::main]
pub async fn main() {

    //CARGAMOS LA DB
    let db = load_db();

    //CARGAMOS LOS PLAYERS
    cargar_players(&db);


    connection(db.clone()).await;
    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send_players])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    

}

pub async fn connection(db: db::Db){
    tokio::spawn(async move {
        let config = ClientConfig::default();

        let (mut incoming_messages, client) = TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

        let join_handle: tokio::task::JoinHandle<()> = tokio::spawn(async move {
            while let Some(message) = incoming_messages.recv().await {
                match message {
                    message::ServerMessage::Privmsg(msg) => {
                        if msg.message_text.contains("!play") {
                            message_recived(msg, &db);
                        }
                    }
                    _ => {}
                }
            }
        });
        
        client.join("al_css_".to_owned()).unwrap();
        println!("Escuchando...");
        
        join_handle.await.unwrap();

    });

}

pub fn message_recived(msg: message::PrivmsgMessage, db: &db::Db){
    println!("Mensaje Recibido de {}: {}:", msg.sender.name, msg.message_text);
    let _mensaje:String = format!("Mensaje Recibido de {}: {}:", msg.sender.name, msg.message_text).to_string();
    let player = msg.sender.name.clone();

    validate(db.clone(), player);
}

#[tauri::command]
fn send_players() -> String{
    let local_db = load_db();
    let players = cargar_players(&local_db);
    let mut cadena: String = String::new();
    for player in players{
        cadena.push_str(&player);
        cadena.push_str("\n");
    }
    println!("Esto sucede: {}", cadena);
    cadena

}

fn load_db()-> db::Db{
    let mut db = db::Db::new();
    println!("{:?}", db);
    db.leer_db();
    db
}

fn validate(db: db::Db, player: String){
    let players = db.obtener_players();

    if !players.contains(&player){
        db.add_player(player.clone());
        println!("{} agregado a la lista de jugadores", player);
    }
    else{
        println!("Este pendejo ya está en la lista de jugadores");
    }

}
fn cargar_players(db: &db::Db) -> Vec<String>{
    let players = db.obtener_players();
    println!("{:?}", players);
    players
}
