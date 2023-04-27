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

const PATH: &str = "../db.txt";

impl Db {
    pub fn leer_db(&mut self){

        let file = match File::open(&self.file){
            Ok(file) => {
                println!("El archivo existe y se abrió correctamente");
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


        self.obtener_players();
        println!("{:?}", file);

    }
    pub fn new() -> Db{
        Db{
            file: String::from(PATH),
            content: String::from(""),
            players: Vec::new(),
        }
    }
    pub fn obtener_players(&self) -> Vec<String>{
        let lineas = self.content.lines();

        let mut players: Vec<String> = Vec::new();

        for linea in lineas {
            players.push(linea.to_string());
        }      
        
        players
    }
    pub fn add_player(&self, player: String){

        let mut file = OpenOptions::new().append(true).write(true).open(&self.file).unwrap();
        let player = format!("{}\n", player);
        match file.write_all(player.as_bytes()) {
            Ok(_) => println!("Esto funciona"),
            Err(e) => eprintln!("Esta mierda se rompió al escribir en el archivo: {}", e)
        }

    }
}