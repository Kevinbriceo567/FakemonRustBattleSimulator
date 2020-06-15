use fltk::{app::*, button::*, frame::*, window::*, image::*};
use std::error::Error;
use std::path::PathBuf;
use std::collections::HashMap;
use image::GenericImageView;
//use rand::prelude::*;

use std::{fs, io};
use std::path::Path;

use std::fmt;

#[derive(Debug)]
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

fn main() -> Result<(), Box<dyn Error>> {

    println!("{}", inGlobal);

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

    // Vettel

    let mut pokemon1 =  Fakemon { 

        nombre : "Vettel",
        codigo : 1,
        hp : 1000,
        defn : 3,
        atk : 4,
        nivel : 1,
        velocidad : 3,
        experiencia : 0,
        tipo : "fuego",
        habilidad1 : fueguito,
        habilidad2 : ascuas,
        habilidad3 : placaje

    };

    let mut pokemon2 =  Fakemon { 

        nombre : "Raikkonen",
        codigo : 4,
        hp : 1000,
        defn : 5,
        atk : 4,
        nivel : 1,
        velocidad : 1,
        experiencia : 0,
        tipo : "Agua",
        habilidad1 : agilidad,
        habilidad2 : salpicar,
        habilidad3 : rugido

    };

    let mut pokemon3 =  Fakemon { 

        nombre : "Albon",
        codigo : 7,
        hp : 1000,
        defn : 2,
        atk : 4,
        nivel : 1,
        velocidad : 4,
        experiencia : 0,
        tipo : "Planta",
        habilidad1 : latigo,
        habilidad2 : corte_hoja,
        habilidad3 : armazon

    };

    println!("\n{}", pokemon1.nombre);

    //VENTANA PRINCIPAL

    let app = App::default().set_scheme(AppScheme::Gleam);
    let mut wind = Window::new(200, 200, 600, 500, "Fakemon Battle Center");

    let mut frame = Frame::new(50, 50, 100, 300, "");
    let mut image = SharedImage::load(&PathBuf::from("res/vettel.png"))?;
    image.scale(200, 200, true, true);
    frame.set_image(&image);

    // LOGO
    let mut frameLogo = Frame::new(250, -100, 100, 300, "");
    let mut imageLogo = SharedImage::load(&PathBuf::from("res/logo.png"))?;
    imageLogo.scale(200, 200, true, true);
    frameLogo.set_image(&imageLogo);


    //STATS EN SELECCION POK1

    let mut frame = Frame::new(50, 180, 100, 300, &format!("HP: {}",pokemon1.hp));
    let mut frame = Frame::new(50, 200, 100, 300, &format!("DEF: {}",pokemon1.defn));
    let mut frame = Frame::new(50, 220, 100, 300, &format!("ATK: {}",pokemon1.atk));
    let mut frame = Frame::new(50, 240, 100, 300, &format!("ASPD: {}",pokemon1.velocidad));
    let mut frame = Frame::new(50, 260, 100, 300, &format!("Tipo: {}",pokemon1.tipo));
   

    let mut frame_f2 = Frame::new(250, 50, 100, 300, "");
    let mut image_f2 = SharedImage::load(&PathBuf::from("res/raikkonen.png"))?;
    image_f2.scale(200, 200, true, true);
    frame_f2.set_image(&image_f2);

    //STATS EN SELECCION POK2

    let mut frame2 = Frame::new(250, 180, 100, 300, &format!("HP: {}",pokemon2.hp));
    let mut frame2 = Frame::new(250, 200, 100, 300, &format!("DEF: {}",pokemon2.defn));
    let mut frame2 = Frame::new(250, 220, 100, 300, &format!("ATK: {}",pokemon2.atk));
    let mut frame2 = Frame::new(250, 240, 100, 300, &format!("ASPD: {}",pokemon2.velocidad));
    let mut frame = Frame::new(250, 260, 100, 300, &format!("Tipo: {}",pokemon2.tipo));

    let mut frame_f3 = Frame::new(450, 50, 100, 300, "");
    let mut image_f3 = SharedImage::load(&PathBuf::from("res/albon.png"))?;
    image_f3.scale(200, 200, true, true);
    frame_f3.set_image(&image_f3);
    
    
    //STATS EN SELECCION POK3

    let mut frame3 = Frame::new(450, 180, 100, 300, &format!("HP: {}",pokemon3.hp));
    let mut frame3 = Frame::new(450, 200, 100, 300, &format!("DEF: {}",pokemon3.defn));
    let mut frame3 = Frame::new(450, 220, 100, 300, &format!("ATK: {}",pokemon3.atk));
    let mut frame3 = Frame::new(450, 240, 100, 300, &format!("ASPD: {}",pokemon3.velocidad));
    let mut frame = Frame::new(450, 260, 100, 300, &format!("Tipo: {}",pokemon3.tipo));

    let mut but = Button::new(50, 240, 80, 40, "Vettel");
    let mut but2 = Button::new(250, 240, 80, 40, "Raikkonen");
    let mut but3 = Button::new(450, 240, 80, 40, "Albon");

    wind.show();


    but.set_callback(Box::new(move || frame.set_label("Meow!")));
    but2.set_callback(Box::new(move || newW(&mut frame_f3, &mut pokemon2)));

    readPokes();

    app.run()?;
    Ok(())
}

fn newW(mut fra : &mut Frame, fake: &mut Fakemon) {

    new_window(fake);
}

fn readPokes() {
    let paths = fs::read_dir("./res/fakemons/").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}

fn new_window(fake : &mut Fakemon) -> Result<(), Box<dyn Error>> {
    let condition = true;

    let mut wind2 = Window ::new(300, 300, 600, 400, "Batalla");

    let mut frame_back = Frame::new(0, -30, 600, 400, "");
    let mut image_back = SharedImage::load(&PathBuf::from("res/fondos/jungle.jpg"))?;
    image_back.scale(900, 500, true, true);
    frame_back.set_image(&image_back);

    let mut frame_you = Frame::new(50, 50, 100, 200, "");
    let mut image_you = SharedImage::load(&PathBuf::from("res/raikkonen.png"))?;
    image_you.scale(200, 200, true, true);
    frame_you.set_image(&image_you);

    let mut frame_text_box = Frame::new(40, 60, 530, 400, "");
    let mut image_text_box = SharedImage::load(&PathBuf::from("res/text_box.png"))?;
    image_text_box.scale(250, 250, true, true);
    frame_text_box.set_image(&image_text_box);

    let mut frame_enemy = Frame::new(50, 50, 900, 200, "");
    let mut image_enemy = SharedImage::load(&PathBuf::from("res/stroll.png"))?;
    image_enemy.scale(200, 200, true, true);
    frame_enemy.set_image(&image_enemy);

    // BOTONES DE FAKEMON
    let mut but_p = Button::new(50, 200, 80, 40, fake.habilidad1.nombre);
    let mut but_p2 = Button::new(50, 250, 80, 40, fake.habilidad2.nombre);
    let mut but_p3 = Button::new(50, 300, 80, 40, fake.habilidad3.nombre);

    let mut frame_comment = Frame::new(50, 50, 500, 400, "");

    let mut msg = "Raikkonen ha usado ".to_string() + fake.habilidad1.nombre;
    but_p.set_callback(Box::new(move || fakeAttack(&mut frame_comment, &msg)));
    //but_p2.set_callback(Box::new(move || frame_comment.set_label("Raikkonen ha usado rugido")));
    //but_p3.set_callback(Box::new(move || frame_comment.set_label("Raikkonen ha usado arañazo")));

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
    

    wind2.show();

    if condition {
        return Err(Box::new(MyError("Oops".into())));
    }

    Ok(())
}

fn fakeAttack(mut frame_comment : &mut Frame, attack : &String) {
    frame_comment.set_label(attack);
}