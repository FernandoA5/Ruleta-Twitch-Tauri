use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Clone, Debug)]
pub struct Channel{
    file: String,
    content: String,
}

const PATH: &str = "/channel.md";

impl Channel{
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
        println!("LEYENDO LA DB: {:?} || {}", file, self.content);

    }
    pub fn new() -> Channel{
        Channel{
            file: String::from(PATH),
            content: String::from(""),
        }
    }
    pub fn set_channel(&mut self, channel: String){
        //REINICIAMOS EL ARCHIVO
        match File::create(PATH){
            Ok(_) => println!("El archivo se reinició correctamente"),
            Err(e) => eprintln!("Error al crear el archivo: {}", e)
        }

        let mut file = OpenOptions::new().append(true).write(true).open(&self.file).unwrap();
        let channel = format!("{}", channel);

        //ESCRIBIMOS EL JUGADOR EN LA db
        match file.write_all(channel.as_bytes()) {
            Ok(_) => {  },
            Err(e) => eprintln!("Esta mierda se rompió al escribir en el archivo: {}", e)
        }
        println!("SETEADO EL CANAL. {:?} || {}", file, channel);
        self.content = channel;

    }
    pub fn get_channel(&self)-> String{
        self.content.clone()
    }

}