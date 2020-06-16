use fltk::{
    app,
    button::*,
    frame::*,
    group::*,
    input::*,
    menu::*,
    output::*,
    valuator::*,
    window::*,
    image::*
};
use std::error::Error;
use std::path::PathBuf;
use std::collections::HashMap;
use image::GenericImageView;
use rand::seq::SliceRandom;
use rand::prelude::*;

use std::{fs, io};
use std::path::Path;

use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

#[derive(Debug, Clone)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for MyError {}

struct Habilidad<'a> {
    // The 'a defines a lifetime
    nombre: &'a str,
    dano: u8,
    cantidad: u8,

    //Entre defensa, ataque y velocidad, no pueden exceder lo 10 puntos.
}


struct Damage<'a>{
    nombre: &'a str,
    dano: u8,
    multiplicador: u8,
    efecto: u8
}


struct Fakemon<'a> {
    // The 'a defines a lifetime
    nombre: &'a str,
    codigo: u8,
    hp: i32,
    defn: u8,
    atk: u8,
    nivel: u8,
    velocidad: u8,
    experiencia: u8,
    tipo: &'a str,
    imagen: &'a str,
    habilidad1: Habilidad<'a>,
    habilidad2: Habilidad<'a>,
    habilidad3: Habilidad<'a>

    //Entre defensa, ataque y velocidad, no pueden exceder lo 10 puntos.
}

struct Enemigo<'a> {
    // The 'a defines a lifetime
    nombre: &'a str,
    hp: i32,
    defn: u8,
    atk: u8,
    nivel: u8,
    velocidad: u8,
    tipo: &'a str,

    //Entre defensa, ataque y velocidad, no pueden exceder lo 10 puntos.
}



struct Entrenador<'a> {
    // The 'a defines a lifetime
    nombre: &'a str,
    dinero: i32,
    pociones: u8,
    medallas: u8,
    sexo: &'a str
}



static entrenadorPlayer: Entrenador = Entrenador {
    nombre: "Chelox",
    dinero: 350000,
    pociones: 5,
    medallas: 0,
    sexo: "Masculino"
};

static inGlobal: i32 = 5;

fn draw_gallery() {

}


fn main() -> Result<(), Box<dyn Error>> {
    let app = app::App::default();

    let (screen_width, screen_height) = fltk::app::screen_size();
    let mut wind = Window::new(
        (screen_width / 2.0 - 250.0) as i32,
        (screen_height / 2.0 - 200.0) as i32,
        740,
        400,
        "Fakemon Battle Center",
    );


    let mut latigo = Habilidad {

        nombre:"Latigo",
        dano: 0,
        cantidad: 10

    };

    let mut burbuja = Habilidad {

        nombre:"Burbuja",
        dano: 10,
        cantidad: 15

    };

    let mut placaje = Habilidad {

        nombre:"Placaje",
        dano: 5,
        cantidad: 20

    };

    let mut ascuas = Habilidad {

        nombre:"Ascuas",
        dano: 10,
        cantidad: 20

    };

    let mut corte_hoja = Habilidad {

        nombre:"Corte hoja",
        dano: 15,
        cantidad: 15

    };

    let mut agilidad = Habilidad {

        nombre:"agilidad",
        dano: 0,
        cantidad: 10

    };

    let mut armazon = Habilidad {

        nombre:"armazon",
        dano: 0,
        cantidad: 10

    };

    let mut fueguito = Habilidad {

        nombre:"fueguito",
        dano: 0,
        cantidad: 20

    };
    
    let mut salpicar = Habilidad {

        nombre:"Salpicar",
        dano: 0,
        cantidad: 50

    };
    
    let mut rugido = Habilidad {

        nombre:"rugido",
        dano: 0,
        cantidad: 20

    };

    // FAKEMON
    let mut fake =  Fakemon { 
        nombre : "Vettel",
        codigo : 1,
        hp : 1000,
        defn : 3,
        atk : 4,
        nivel : 1,
        velocidad : 3,
        experiencia : 0,
        tipo : "fuego",
        imagen : "vettel.png",
        habilidad1 : fueguito,
        habilidad2 : ascuas,
        habilidad3 : placaje

    };

    // ENEMIGO
    let mut enemigo =  Enemigo { 

        nombre : "Stroll",
        hp : 1000,
        defn : 3,
        atk : 4,
        nivel : 1,
        velocidad : 3,
        tipo : "fuego",
    };

    let tab = Tabs::new(10, 10, 650 - 20, 400 - 20, "");
    
    let grp1 = Group::new(10, 35, 650 - 20, 400 - 45, "Principal");
    
    let _frame = Frame::new(270, 335, 90, 30, "Examples!");

    let mut frame_back = Frame::new(35, 35, 600, 400, "");
    let mut image_back = SharedImage::load(&PathBuf::from("res/fondos/title_screen.png"))?;
    image_back.scale(700, 505, true, true);
    frame_back.set_image(&image_back);

    let mut frame = Frame::new(70, 70, 100, 300, "");
    let mut image = SharedImage::load(&PathBuf::from("res/vettel.png"))?;
    image.scale(200, 200, true, true);
    frame.set_image(&image);

    let mut frame_f2 = Frame::new(270, 70, 100, 300, "");
    let mut image_f2 = SharedImage::load(&PathBuf::from("res/raikkonen.png"))?;
    image_f2.scale(200, 200, true, true);
    frame_f2.set_image(&image_f2);

    let mut frame_f3 = Frame::new(470, 70, 100, 300, "");
    let mut image_f3 = SharedImage::load(&PathBuf::from("res/albon.png"))?;
    image_f3.scale(200, 200, true, true);
    frame_f3.set_image(&image_f3);

    // STATS DE ENTRENADOR
    // DINERO
    let mut frameLogo = Frame::new(350, -100, 300, 300, "");
    let mut imageLogo = SharedImage::load(&PathBuf::from("res/iconos/money.png"))?;
    imageLogo.scale(30, 30, true, true);
    frameLogo.set_image(&imageLogo);

    // MEDALLAS
    let mut frameLogo = Frame::new(400, -100, 300, 300, "");
    let mut imageLogo = SharedImage::load(&PathBuf::from("res/iconos/potions.png"))?;
    imageLogo.scale(30, 30, true, true);
    frameLogo.set_image(&imageLogo);
    
    grp1.end();
    
    let mut grp2 = Group::new(10, 35, 650 - 30, 400 - 25, "Batalla");

    let mut frame_back = Frame::new(0, -30, 650, 400, "");
    let mut image_back = SharedImage::load(&PathBuf::from("res/fondos/jungle.jpg"))?;
    image_back.scale(650, 530, true, true);
    frame_back.set_image(&image_back);

    let mut frame_you = Frame::new(50, 50, 100, 200, "");
    let mut image_you = SharedImage::load(&PathBuf::from("res/".to_string() + &fake.imagen.to_string()))?;
    image_you.scale(200, 200, true, true);
    frame_you.set_image(&image_you);

    let mut frame_text_box = Frame::new(40, 60, 530, 400, "");
    let mut image_text_box = SharedImage::load(&PathBuf::from("res/text_box.png"))?;
    image_text_box.scale(250, 250, true, true);
    frame_text_box.set_image(&image_text_box);

    let mut randomEnemy = readPokes("str");

    let mut frame_enemy = Frame::new(50, 50, 900, 200, "");
    let mut image_enemy = SharedImage::load(&PathBuf::from("res/fakemons/".to_string() + &randomEnemy.to_string()))?;
    image_enemy.scale(200, 200, true, true);
    frame_enemy.set_image(&image_enemy);

    let mut frame_enemy_health = Frame::new(50, 150, 900, 200, "");
    let mut image_enemy_health = SharedImage::load(&PathBuf::from("res/iconos/healthbar.png"))?;
    image_enemy_health.scale(90, 90, true, true);
    frame_enemy_health.set_image(&image_enemy_health);

    // BOTONES DE FAKEMON
    let mut but_p = Button::new(50, 200, 80, 40, fake.habilidad1.nombre);
    let mut but_p2 = Button::new(50, 250, 80, 40, fake.habilidad2.nombre);
    let mut but_p3 = Button::new(50, 300, 80, 40, fake.habilidad3.nombre);
    but_p.set_color(Color::from_rgb(3, 110, 250));
    but_p2.set_color(Color::from_rgb(3, 110, 250));
    but_p3.set_color(Color::from_rgb(3, 110, 250));



    wind.end();
    wind.show();

    let (s, r) = app::channel::<Message>();

    but_p.emit(s, Message::Increment);
    but_p2.emit(s, Message::Decrement);
    but_p3.emit(s, Message::Decrement);
    //but_p3.emit(s, Message::Decrement);



    grp2.end();

    // TAB DE TIENDA
    let grp3 = Group::new(10, 35, 650 - 30, 400 - 25, "Tienda");

    let mut frame_back = Frame::new(0, 0, 650, 400, "");
    let mut image_back = SharedImage::load(&PathBuf::from("res/fondos/tienda.png"))?;
    image_back.scale(650, 400, true, true);
    frame_back.set_image(&image_back);
    
    grp3.end();
    
    tab.end();

    wind.make_resizable(false);
    wind.end();
    wind.show();

    while app.wait()? {
        let label: i32 = 0;//frame.label().parse()?;
        
        match r.recv() {
            Some(Message::Increment) => cha(&mut frame_enemy, &mut enemigo),
            Some(Message::Decrement) => cha(&mut frame_enemy, &mut enemigo),
            None => (),
        }
    }
    
    app.run()?;
    Ok(())
}


fn readPokes(input: &str) -> String {
    let paths = fs::read_dir("./res/fakemons/").unwrap();

    let names = paths.map(|entry| {
        let entry = entry.unwrap();

        let entry_path = entry.path();

        let file_name = entry_path.file_name().unwrap();

        let file_name_as_str = file_name.to_str().unwrap();

        let file_name_as_string = String::from(file_name_as_str);

        file_name_as_string
    }).collect::<Vec<String>>();

    let mut rng = rand::thread_rng();

    println!("{:?}", names.choose(&mut rand::thread_rng()));
    let mut randomNumber = rng.gen_range(0, 14);
    let mut randomPokemon = &names[randomNumber as usize];
    println!("RANDOM: {}", randomPokemon);

    randomPokemon.to_string()

}

fn cha(frame_enemy: &mut Frame, enemigo: &mut Enemigo) {
    change(frame_enemy, enemigo);
}

fn change(frame_enemy: &mut Frame, enemigo: &mut Enemigo) -> Result<(), Box<dyn Error>> {

    frame_enemy.set_label(&format!("HP: {}",enemigo.hp));

    let mut randomEnemy = readPokes("str");

    let mut image_enemy = SharedImage::load(&PathBuf::from("res/fakemons/".to_string() + &randomEnemy.to_string()))?;
    image_enemy.scale(200, 200, true, true);
    frame_enemy.set_image(&image_enemy);

    Ok(())
}