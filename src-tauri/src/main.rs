// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod db;

use twitch_irc::{login::StaticLoginCredentials, ClientConfig, SecureTCPTransport, TwitchIRCClient, message};


#[tokio::main]
pub async fn main() {

    connection().await;
    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send_players, drop_player])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    

}

pub async fn connection(){
    tokio::spawn(async move {
        let config = ClientConfig::default();

        let (mut incoming_messages, client) = TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

        let join_handle: tokio::task::JoinHandle<()> = tokio::spawn(async move {
            while let Some(message) = incoming_messages.recv().await {
                match message {
                    message::ServerMessage::Privmsg(msg) => {
                        if msg.message_text.contains("!play") {
                            message_recived(msg);
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

pub fn message_recived(msg: message::PrivmsgMessage){
    println!("Mensaje Recibido de {}: {}:", msg.sender.name, msg.message_text);
    let _mensaje:String = format!("Mensaje Recibido de {}: {}:", msg.sender.name, msg.message_text).to_string();
    let player = msg.sender.name.clone();

    validate_and_add(player.clone());    
}

#[tauri::command]
fn send_players() -> String{

    let local_db = load_db();

    println!("Enviada a JS: {:?}", local_db);

    local_db.content
}
#[tauri::command]
fn drop_player(player: String) -> String{
    //Cargo la db
    let mut local_db = load_db();

    //Verifico que el player exista
    if player_exist(player.clone()){
        //Elimino el player
        local_db.drop_player(player.clone());
        println!("{:?}", local_db);
        //Guardo la db
        local_db.guardar_db();
        //Retorno el mensaje
        println!("{:?}", local_db);

        println!("{} eliminado de la lista de jugadores", player);
        return format!("{} eliminado de la lista de jugadores", player);
    }
    "El jugador no existe".to_string()
}

fn validate_and_add(player: String) -> bool{
    
    let mut db = load_db();

    println!("Validando: {:?}", db);

    if !db.players.contains(&player){

        //Escribe en el archivo
        db.add_player(player.clone());

        //Actualizamos el db.content con el nuevo jugador       
        db.content.push_str(&format!("{}\n", player));

        //Porcesa db.content para obtener los players y los settea en db.players
        db.settear_players();

        //Resetea el archivo de la db y reescribe los players uno por uno
        db.guardar_db();

        println!("Validado: {:?}", db);
        true
    }
    else{
        println!("Este pendejo ya está en la lista de jugadores");
        false
    }

}

fn player_exist(player: String) -> bool{
    let db = load_db();
    let players = db.players;

    players.contains(&player)
}


fn load_db()-> db::Db{
    let mut db = db::Db::new();
    db.leer_db();
    db.settear_players();
    return db;
}
