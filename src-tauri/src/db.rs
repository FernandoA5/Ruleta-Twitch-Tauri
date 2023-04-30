use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Clone, Debug)]
pub struct Db{
    pub file: String,
    pub content: String,
    pub players: Vec<String>,
}

const PATH: &str = "/db.md";

impl Db {
    pub fn leer_db(&mut self){

        let file = match File::open(&self.file){
            Ok(file) => {
                //println!("El archivo existe y se abrió correctamente");
                file
            },
            Err(_) => {
                println!("El puto archivo no existe, se creará uno nuevo");
                File::create(PATH).expect("Error al crear el archivo")
            }
        };
        let mut buf_reader = BufReader::new(&file);
        let mut contenido = String::new();
        buf_reader.read_to_string(&mut contenido).expect("Error al leer el archivo");

        self.content = contenido;
        //println!("{:?}", file);

    }


    pub fn new() -> Db{
        Db{
            file: String::from(PATH),
            content: String::from(""),
            players: Vec::new(),
        }
    }


    pub fn settear_players(&mut self){
        let lineas = self.content.lines();

        let mut players: Vec<String> = Vec::new();

        for linea in lineas {
            players.push(linea.to_string());
        }      
        
        self.players = players.clone();
    }


    pub fn add_player(&self, player: String){
        let mut file = OpenOptions::new().append(true).write(true).open(&self.file).unwrap();
        let player = format!("{}\n", player);

        //ESCRIBIMOS EL JUGADOR EN LA db
        match file.write_all(player.as_bytes()) {
            Ok(_) => {  },
            Err(e) => eprintln!("Esta mierda se rompió al escribir en el archivo: {}", e)
        }

    }


    pub fn drop_player(&mut self, player: String){
        let index = match self.players.iter().position(|x| *x == player){
            Some(index) => index,
            None => {
                println!("El jugador no existe");
                return;
            }
        };   

        self.players.remove(index);
    }

    pub fn guardar_db(&mut self){

        //Reiniciamos el archivo de la DB
        match File::create(PATH){
            Ok(_) => println!("El archivo se reinició correctamente"),
            Err(e) => eprintln!("Error al crear el archivo: {}", e)
        }
        //Reescribimos todos los usuarios
        for player in &self.players{
            self.add_player(player.clone());
        }
    }
    pub fn clear_players(&mut self){
        //Vaciamos el vector de players
        self.players.clear();

        //Reiniciamos el archivo de la DB
        match File::create(PATH){
            Ok(_) => println!("El archivo se reinició correctamente"),
            Err(e) => eprintln!("Error al crear el archivo: {}", e)
        }
    }
}