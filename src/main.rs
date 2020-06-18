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
    Attack3
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
    dano: i32,
    cantidad: i32,
    bajaDefensa: i32,
    bajaAtaque: i32

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
    defn: i32,
    atk: i32,
    nivel: i32,
    velocidad: i32,
    experiencia: i32,
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
    defn: i32,
    atk: i32,
    nivel: i32,
    velocidad: i32,
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

struct Objeto<'a> {

    nombre: &'a str,
    precio: &'a u32,
    descripcion: &'a str,

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
        630,
        440,
        "Fakemon Battle Center",
    );


    let mut latigo = Habilidad {

        nombre:"Latigo",
        dano: 0,
        cantidad: 10,
        bajaDefensa: 1,
        bajaAtaque: 0

    };


    let mut grunido = Habilidad {

        nombre:"Gruñido",
        dano: 0,
        cantidad: 10,
        bajaDefensa: 0,
        bajaAtaque: 1

    };

    let mut burbuja = Habilidad {

        nombre:"Burbuja",
        dano: 10,
        cantidad: 15,
        bajaDefensa: 0,
        bajaAtaque: 0

    };

    let mut placaje = Habilidad {

        nombre:"Placaje",
        dano: 5,
        cantidad: 20,
        bajaDefensa: 0,
        bajaAtaque: 0

    };

    let mut ascuas = Habilidad {

        nombre:"Ascuas",
        dano: 10,
        cantidad: 20,
        bajaDefensa: 0,
        bajaAtaque: 0

    };

    let mut corte_hoja = Habilidad {

        nombre:"Corte hoja",
        dano: 15,
        cantidad: 15,
        bajaDefensa: 0,
        bajaAtaque: 0

    };

    // FAKEMON
    let mut fake =  Fakemon { 
        nombre : "Vettel",
        codigo : 1,
        hp : 180,
        defn : 3,
        atk : 4,
        nivel : 1,
        velocidad : 3,
        experiencia : 0,
        tipo : "fuego",
        imagen : "vettel.png",
        habilidad1 : latigo,
        habilidad2 : grunido,
        habilidad3 : placaje

    };

    // ENEMIGO
    let mut enemigo =  Enemigo { 

        nombre : "Stroll",
        hp : 200,
        defn : 3,
        atk : 4,
        nivel : 1,
        velocidad : 3,
        tipo : "fuego",
    };

    // OBJETOS

    let mut pocion = Objeto {

        nombre: "Pocion",
        precio: &500,
        descripcion: "Pocion comun que restaura 500 puntos de HP del fakemon.",
    
    };
    
    let mut super_pocion = Objeto {
    
        nombre: "Super Pocion",
        precio: &700,
        descripcion: "Pocion nivel 2 que restaura 800 puntos de HP del fakemon.",
    
    };
    
    let mut hiper_pocion = Objeto {
    
        nombre: "Hiper Pocion",
        precio: &1000,
        descripcion: "Pocion nivel 3 que restaura 1000 puntos de HP del fakemon.",
    
    };
    
    let mut fakeball = Objeto {
    
        nombre: "Fakeball",
        precio: &1500,
        descripcion: "Objeto memorable caracteristico de la saga. Se dice que quien lo adquiere obtiene poderes de programador.",
    
    };

    let mut revivir = Objeto {
    
        nombre: "Revivir",
        precio: &2000,
        descripcion: "Permite re animar al fakemon caído en batalla.",
    
    };

    let tab = Tabs::new(0, 0, 650 - 20, 420 - 20, "");
    
    let grp1 = Group::new(0, 35, 650 - 20, 420 - 20, "Principal");
    
    let _frame = Frame::new(270, 335, 90, 30, "Examples!");

    let mut frame_back = Frame::new(0, 30, 650, 400, "");
    let mut image_back = SharedImage::load(&PathBuf::from("res/fondos/title_screen.png"))?;
    image_back.scale(650, 530, true, true);
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

    // BOTONES PARA ELEGIR FAKEMON
    let mut but_f1 = Button::new(70, 300, 80, 40, "Vettel");
    let mut but_f2 = Button::new(270, 300, 80, 40, "Raikkonen");
    let mut but_f3 = Button::new(470, 300, 80, 40, "Albon");
    but_f1.set_color(Color::from_rgb(240, 29, 14));
    but_f2.set_color(Color::from_rgb(13, 198, 255));
    but_f3.set_color(Color::from_rgb(7, 242, 62));
    but_f1.set_frame(FrameType::RoundUpBox);
    but_f2.set_frame(FrameType::RoundUpBox);
    but_f3.set_frame(FrameType::RoundUpBox);
    
    grp1.end();
    
    let mut grp2 = Group::new(0, 35, 650 - 20, 420 - 20, "Batalla");

    let mut frame_back = Frame::new(-20, -30, 760, 490, "");
    let mut image_back = SharedImage::load(&PathBuf::from("res/fondos/jungle.jpg"))?;
    image_back.scale(760, 490, true, true);
    frame_back.set_image(&image_back);

    let mut frame_you = Frame::new(50, 50, 100, 200, "");
    let mut image_you = SharedImage::load(&PathBuf::from("res/".to_string() + &fake.imagen.to_string()))?;
    image_you.scale(200, 200, true, true);
    frame_you.set_image(&image_you);

    let mut frame_text_box = Frame::new(60, 70, 530, 400, "");
    let mut image_text_box = SharedImage::load(&PathBuf::from("res/text_box.png"))?;
    image_text_box.scale(250, 250, true, true);
    frame_text_box.set_image(&image_text_box);

    // NOMBRES
    let mut frame_text_box = Frame::new(60, 20, 530, 300, "");
    let mut image_text_box = SharedImage::load(&PathBuf::from("res/iconos/namebar.png"))?;
    image_text_box.scale(250, 250, true, true);
    frame_text_box.set_image(&image_text_box);

    // JUGADOR //
    // INDICADOR DE VIDA
    let mut frame_fake_health = Frame::new(10, 170, 90, 90, "");
    let mut image_fake_health = SharedImage::load(&PathBuf::from("res/iconos/healthbar.png"))?;
    image_fake_health.scale(90, 90, true, true);
    frame_fake_health.set_image(&image_fake_health);

    let mut frame_fake_health_text = Frame::new(20, 160, 90, 90, "");
    frame_fake_health_text.set_label(&format!("{}",fake.hp));

    // INDICADOR DE DEFENSA
    let mut frame_fake_def = Frame::new(110, 170, 90, 90, "");
    let mut image_fake_def = SharedImage::load(&PathBuf::from("res/iconos/shieldsword.png"))?;
    image_fake_def.scale(90, 90, true, true);
    frame_fake_def.set_image(&image_fake_def);

    let mut frame_fake_def_text = Frame::new(100, 160, 90, 90, "");
    frame_fake_def_text.set_label(&format!("{}",fake.defn));

    let mut frame_fake_atk_text = Frame::new(130, 160, 90, 90, "");
    frame_fake_atk_text.set_label(&format!("{}",fake.atk));

    // ENEMIGO //
    let mut randomEnemy = readPokes("str");

    let mut frame_enemy = Frame::new(420, 50, 200, 200, "");
    let mut image_enemy = SharedImage::load(&PathBuf::from("res/fakemons/".to_string() + &randomEnemy.to_string()))?;
    image_enemy.scale(200, 200, true, true);
    frame_enemy.set_image(&image_enemy);

    // INDICADOR DE VIDA
    let mut frame_enemy_health = Frame::new(30, 110, 900, 200, "");
    let mut image_enemy_health = SharedImage::load(&PathBuf::from("res/iconos/healthbar.png"))?;
    image_enemy_health.scale(90, 90, true, true);
    frame_enemy_health.set_image(&image_enemy_health);

    let mut frame_enemy_health_text = Frame::new(40, 100, 900, 200, "");
    frame_enemy_health_text.set_label(&format!("{}",enemigo.hp));

    // INDICADOR DE DEFENSA
    let mut frame_enemy_def = Frame::new(130, 110, 900, 200, "");
    let mut image_enemy_def = SharedImage::load(&PathBuf::from("res/iconos/shieldsword.png"))?;
    image_enemy_def.scale(90, 90, true, true);
    frame_enemy_def.set_image(&image_enemy_def);

    let mut frame_enemy_def_text = Frame::new(120, 100, 900, 200, "");
    frame_enemy_def_text.set_label(&format!("{}",enemigo.defn));

    let mut frame_enemy_atk_text = Frame::new(150, 100, 900, 200, "");
    frame_enemy_atk_text.set_label(&format!("{}",enemigo.atk));

    // BOTONES DE FAKEMON
    let mut but_p = Button::new(60, 230, 80, 40, fake.habilidad1.nombre);
    let mut but_p2 = Button::new(60, 280, 80, 40, fake.habilidad2.nombre);
    let mut but_p3 = Button::new(60, 330, 80, 40, fake.habilidad3.nombre);
    but_p.set_color(Color::from_rgb(245, 235, 15));
    but_p2.set_color(Color::from_rgb(245, 235, 15));
    but_p3.set_color(Color::from_rgb(245, 235, 15));
    but_p.set_frame(FrameType::RoundUpBox);
    but_p2.set_frame(FrameType::RoundUpBox);
    but_p3.set_frame(FrameType::RoundUpBox);

    wind.end();
    wind.show();

    let (s, r) = app::channel::<Message>();

    but_p.emit(s, Message::Increment);
    but_p2.emit(s, Message::Decrement);
    but_p3.emit(s, Message::Attack3);

    grp2.end();

    // TAB DE TIENDA ////////////////////////////////////////////////////////////////////
    let grp3 = Group::new(0, 35, 650 - 30, 400 - 25, "Tienda");

    let mut frame_back = Frame::new(-20, 0, 660, 400, "");
    let mut image_back = SharedImage::load(&PathBuf::from("res/fondos/tienda.png"))?;
    image_back.scale(660, 400, true, true);
    frame_back.set_image(&image_back);

    //BOTONES DE LA TIENDA

    let mut _pocion = Button::new(30, 180, 140, 40, "Comprar");
    let mut d_pocion = Frame::new(50, -5, 100, 300, &format!("{}",pocion.nombre));
    let mut p_pocion = Frame::new(50, 15, 100, 300, &format!("Precio: {}",pocion.precio));
    _pocion.set_color(Color::from_rgb(245, 235, 15));
    _pocion.set_frame(FrameType::RoundUpBox);

    let mut _super_pocion = Button::new(250, 180, 140, 40, "Comprar");
    let mut d_super_pocion = Frame::new(270, -1, 100, 300, &format!("{}",super_pocion.nombre));
    let mut p_super_pocion = Frame::new(270, 17, 100, 300, &format!("Precio: {}",super_pocion.precio));
    _super_pocion.set_color(Color::from_rgb(245, 235, 15));
    _super_pocion.set_frame(FrameType::RoundUpBox);

    let mut _hiper_pocion = Button::new(470, 180, 140, 40, "Comprar");
    let mut d_hiper_pocion = Frame::new(490, -1, 100, 300, &format!("{}",hiper_pocion.nombre));
    let mut p_hiper_pocion = Frame::new(490, 17, 100, 300, &format!("Precio: {}",hiper_pocion.precio));
    _hiper_pocion.set_color(Color::from_rgb(245, 235, 15));
    _hiper_pocion.set_frame(FrameType::RoundUpBox);

    let mut _fakebola = Button::new(110, 345, 140, 40, "Comprar");
    let mut d_fakebola = Frame::new(130, 185, 100, 300, &format!("{}",fakeball.nombre));
    let mut p_fakebola = Frame::new(130, 165, 100, 300, &format!("Precio: {}",fakeball.precio));
    _fakebola.set_color(Color::from_rgb(245, 235, 15));
    _fakebola.set_frame(FrameType::RoundUpBox);

    let mut _revivir = Button::new(390, 342, 140, 40, "Comprar");
    let mut d_revivir = Frame::new(410, 180, 100, 300, &format!("{}",revivir.nombre));
    let mut p_revivir = Frame::new(410, 160, 100, 300, &format!("Precio: {}",revivir.precio));
    _revivir.set_color(Color::from_rgb(245, 235, 15));
    _revivir.set_frame(FrameType::RoundUpBox);

    image_back.scale(650, 400, true, true);
    frame_back.set_image(&image_back);

    grp3.end();
    
    tab.end();

    // STATS DE ENTRENADOR
    // DINERO
    let mut frameLogo = Frame::new(40, 410, 30, 30, "");
    let mut imageLogo = SharedImage::load(&PathBuf::from("res/iconos/coin.png"))?;
    imageLogo.scale(30, 30, true, true);
    frameLogo.set_image(&imageLogo);

    // MEDALLAS
    let mut frameLogo = Frame::new(130, 410, 30, 30, "");
    let mut imageLogo = SharedImage::load(&PathBuf::from("res/iconos/blue_liquid.png"))?;
    imageLogo.scale(30, 30, true, true);
    frameLogo.set_image(&imageLogo);

    wind.make_resizable(false);
    wind.end();
    wind.show();

    while app.wait()? {
        let label: i32 = 0;//frame.label().parse()?;
        
        match r.recv() {
            Some(Message::Increment) => attack(
                &mut enemigo, 
                &mut fake, 
                &mut frame_enemy_health_text, 
                &mut frame_enemy_def_text, 
                &mut frame_enemy_atk_text, 
                &mut frame_fake_health_text, 
                &mut frame_fake_def_text, 
                &mut frame_fake_atk_text, 
                1),
            Some(Message::Decrement) => attack(
                &mut enemigo, 
                &mut fake, 
                &mut frame_enemy_health_text, 
                &mut frame_enemy_def_text, 
                &mut frame_enemy_atk_text, 
                &mut frame_fake_health_text, 
                &mut frame_fake_def_text, 
                &mut frame_fake_atk_text, 
                2),
            Some(Message::Attack3) => attack(
                &mut enemigo, 
                &mut fake, 
                &mut frame_enemy_health_text, 
                &mut frame_enemy_def_text, 
                &mut frame_enemy_atk_text, 
                &mut frame_fake_health_text, 
                &mut frame_fake_def_text, 
                &mut frame_fake_atk_text, 
                3),
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


fn attack(
    enemigo: &mut Enemigo, 
    fakemon: &mut Fakemon,
    frame_enemy_health_text: &mut Frame, 
    frame_enemy_defn_text: &mut Frame, 
    frame_enemy_atk_text: &mut Frame, 
    frame_fake_health_text: &mut Frame, 
    frame_fake_defn_text: &mut Frame, 
    frame_fake_atk_text: &mut Frame, 
    num: i32) 

{
    
    let mut newHabilidad = &fakemon.habilidad1;

    if num == 1 {
        newHabilidad = &fakemon.habilidad1;
    }
    else if num == 2 {
        newHabilidad = &fakemon.habilidad2;
    }
    else {
        newHabilidad = &fakemon.habilidad3;
    }

    if newHabilidad.dano > 0 {
        let mut dano : i32 = (newHabilidad.dano * fakemon.atk) - enemigo.defn;
        let mut nuevoHpEnemigo : i32 = enemigo.hp - dano;
        frame_enemy_health_text.set_label(&format!("{}",nuevoHpEnemigo));
        enemigo.hp = nuevoHpEnemigo;
    }

    if newHabilidad.bajaDefensa > 0 {
        let mut nuevoDefEnemigo : i32 = enemigo.defn - newHabilidad.bajaDefensa;
        frame_enemy_defn_text.set_label(&format!("{}",nuevoDefEnemigo));
        enemigo.defn = nuevoDefEnemigo;
    }

    if newHabilidad.bajaAtaque > 0 {
        let mut nuevoAtkEnemigo : i32 = enemigo.atk - newHabilidad.bajaAtaque;
        frame_enemy_atk_text.set_label(&format!("{}",nuevoAtkEnemigo));
        enemigo.atk = nuevoAtkEnemigo;
    }
    

}

fn changeEnemy(frame_enemy: &mut Frame, frame_enemy_health_text: &mut Frame, enemigo: &mut Enemigo) {
    change(frame_enemy, frame_enemy_health_text, enemigo);
}

fn change(frame_enemy: &mut Frame, frame_enemy_health_text: &mut Frame, enemigo: &mut Enemigo) -> Result<(), Box<dyn Error>> {

    let mut randomEnemy = readPokes("str");
    frame_enemy_health_text.set_label(&format!("{}",enemigo.hp));

    let mut image_enemy = SharedImage::load(&PathBuf::from("res/fakemons/".to_string() + &randomEnemy.to_string()))?;
    image_enemy.scale(200, 200, true, true);
    frame_enemy.set_image(&image_enemy);

    Ok(())
}