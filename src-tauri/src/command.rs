use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Clone, Debug)]
pub struct Command{
    file: String,
    content: String,
}

const PATH: &str = "../command.md";

impl Command{
    pub fn leer_db(&mut self){
        //ABRIMOS EL ARCHIVO
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
        println!("LEYENDO LA DB: {:?} || {}", file, self.content);

    }
    pub fn new() -> Command{
        Command{
            file: String::from(PATH),
            content: String::from(""),
        }
    }
    pub fn set_command(&mut self, command: String){
        //REINICIAMOS EL ARCHIVO
        match File::create(PATH){
            Ok(_) => println!("El archivo se reinició correctamente"),
            Err(e) => eprintln!("Error al crear el archivo: {}", e)
        }

        let mut file = OpenOptions::new().append(true).write(true).open(&self.file).unwrap();
        let command = format!("{}", command);

        //ESCRIBIMOS EL JUGADOR EN LA db
        match file.write_all(command.as_bytes()) {
            Ok(_) => {  },
            Err(e) => eprintln!("Esta mierda se rompió al escribir en el archivo: {}", e)
        }
        println!("SETEADO EL COMANDO. {:?} || {}", file, command);
        self.content = command;

    }
    pub fn get_command(&self) -> String{
        self.content.clone()
    }
}