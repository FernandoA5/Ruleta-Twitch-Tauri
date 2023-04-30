// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod db;
pub mod channel;
pub mod command;

use twitch_irc::{login::StaticLoginCredentials, ClientConfig, SecureTCPTransport, TwitchIRCClient, message};

static mut CONN: Option<tokio::task::JoinHandle<()>> = None;
//---------------------------------------------MAIN------------------------------------------------
#[tokio::main]
pub async fn main() {

    //CONFIGURAMOS LA APP TAURI
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        start_connection_twitch,
        send_players, 
        drop_player, 
        clear_players, 
        add_player,
        set_channel,
        get_channel,
        set_command,
        get_command
        ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}
//---------------------------------------------CONEXIÓN CON TWITCH------------------------------------------------
pub async fn connection(channel: String, command: String) -> tokio::task::JoinHandle<()>{
   println!(" >>Iniciando conexión con el canal {}", channel);
    tokio::spawn(async move {
        println!("  >>>Dentro del tokio spawn");
        let config = ClientConfig::default();

        let (mut incoming_messages, client) = TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

        let join_handle: tokio::task::JoinHandle<()> = tokio::spawn(async move {
            while let Some(message) = incoming_messages.recv().await {
                match message {
                    message::ServerMessage::Privmsg(msg) => {
                        let mut command_validado = String::new();
                        if command.is_empty(){
                            command_validado = "!play".to_string();
                        }

                        if msg.message_text.contains(command_validado.as_str()) {
                            message_recived(msg);
                        }
                    }
                    _ => {}
                }
            }
        });
        let mut db_channel = channel::Channel::new();
        db_channel.leer_db();
        println!("    >>>El canal es {}", channel);
        if channel.is_empty(){
            client.join("al_css_".to_owned()).unwrap();
            println!("    >>>ESTO ESTÁ VACÍO");
            db_channel.set_channel("al_css_".to_string());
                
        }else{
            match client.join(channel.clone()){
                Ok(_) => {
                    println!("    >>>Conectado al canal {}", channel);
                },
                Err(e) => {
                        println!("    >>>Error al conectarse al canal {}: {}", channel, e);
                        client.join("al_css_".to_owned()).unwrap();
                        println!("    >>>SE SUPONE QUE AQUÏ SE REESCRIBE EL ARCHIVO");
                    db_channel.set_channel("al_css_".to_string());
                    db_channel.leer_db();
                }
            }
        }
        println!("Escuchando...\n");
        
        join_handle.await.unwrap();

    })

}

pub fn message_recived(msg: message::PrivmsgMessage){
    println!("Mensaje Recibido de {}: {}:", msg.sender.name, msg.message_text);
    let _mensaje:String = format!("Mensaje Recibido de {}: {}:", msg.sender.name, msg.message_text).to_string();
    let player = msg.sender.name.clone();

    validate_and_add(player.clone());    
}
/*---------------------------------------------COMANDOS DE TAURI------------------------------------------------*/
#[tauri::command]
async fn set_channel(channel: String) -> String{
    println!("\n>>>Iniciando SET_CHANNEL con el canal {}", channel);
    unsafe{
        //DETENER LA CONEXIÓN ACTUAL
        if let Some(conn) = CONN.take(){
            conn.abort();
        }
        //HAY QUE SETTEAR EL CANAL A LA BASE DE DATOS.
        let mut db_channel = channel::Channel::new(); //CREO UNA NUEVA INSTANCIA DE CHANNEL
        db_channel.leer_db(); //LEO LA DB (SI NO EXISTE LA CREA, Y SI EXISTE LA CARGA)
        db_channel.set_channel(channel.clone()); //SETTEO EL CANAL (ESTO SETEA LA DB Y ESCRIBE EN EL ARCHIVO)
        
    }

    println!("\n");

    format!("{}", channel.clone())
    
}
#[tauri::command]
fn set_command(command: String)->String{
    println!("\n>>>Iniciando SET_COMMAND con el comando {}", command);
    unsafe{
        //DETENER LA CONEXIÓN ACTUAL
        if let Some(conn) = CONN.take(){
            conn.abort();
        }
        //HAY QUE SETTEAR EL COMMANDO A LA BASE DE DATOS.
        let mut db_command = command::Command::new(); //CREO UNA NUEVA INSTANCIA DE COMMAND
        db_command.leer_db(); //LEO LA DB (SI NO EXISTE LA CREA, Y SI EXISTE LA CARGA)
        db_command.set_command(command.clone()); //SETTEO EL COMANDO (ESTO SETEA LA DB Y ESCRIBE EN EL ARCHIVO)
    }
    println!("\n");
    format!("{}", command.clone())
}

#[tauri::command]
fn get_channel() -> String{
    println!("\n>>>>Ejecutado GET_CHANNEL: ");
    let mut db_channel = channel::Channel::new();
    db_channel.leer_db();

    println!("Canal devuelto: {}\n", db_channel.get_channel());
    db_channel.get_channel()
}

#[tauri::command]
fn get_command() -> String{
    println!("\n>>>>Ejecutado GET_COMMAND: ");
    let mut db_command = command::Command::new();
    db_command.leer_db();

    println!("Comando devuelto: {}\n", db_command.get_command());
    db_command.get_command()
}

#[tauri::command]
async fn start_connection_twitch() -> String{
    println!("\n>>>Iniciando START_CONNECTION_TWITCH");
    start_connection().await;
    "Conexión iniciada".to_string()
}
//----------------------------------------------------------------------------------------------------------



#[tauri::command]
fn send_players() -> String{
    let local_db = load_db();
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

#[tauri::command]
fn clear_players() -> String{
    //Cargo la db
    let mut local_db = load_db();
    //Elimino todos los players
    local_db.clear_players();
    //Guardo la db
    local_db.guardar_db();
    //Retorno el mensaje
    println!("Lista de jugadores vaciada");
    "Lista de jugadores vaciada".to_string()
}

#[tauri::command]
fn add_player(player: String) -> String {
    //VALIDAR QUE EL PLAYER NO ESTÉ EN LA LISTA
    if validate_and_add(player.clone()){ //LA FUNCIÓN VALIDATE AND ADD AGREGA EL PLAYER A LA DB
        format!("{} agregado a la lista de jugadores", player)
    }
    else{
        format!("{} ya está en la lista de jugadores", player)
    }
}

//---------------------------------------------OTRAS FUNCIONES------------------------------------------------

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

async fn start_connection(){
    let mut channel = channel::Channel::new(); // NUEVA INSTANCIA DE CHANNEL
    channel.leer_db(); // ESTO LEE EL ARCHIVO.

    let mut command = command::Command::new(); // NUEVA INSTANCIA DE COMMAND
    command.leer_db(); // ESTO LEE EL ARCHIVO.
    
    unsafe{
        //AQUÍ YA LLEGA VACIO
        println!(">>>Iniciando start_connection: {:?} || {}", channel, channel.get_channel());
        CONN =  Some(connection(channel.get_channel(), command.get_command()).await);   
    }
}