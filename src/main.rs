use fltk::{app::*, button::*, frame::*, window::*, image::*};
use std::error::Error;
use std::path::PathBuf;
use std::collections::HashMap;

struct Fakemon<'a> {
    // The 'a defines a lifetime
    nombre: &'a str,
    codigo: u8,
    hp: u8,
    defn: u8,
    nivel: u8,
    tipo: &'a str,
}


fn main() -> Result<(), Box<dyn Error>> {

    let nombre = "Vettel";
    let codigo = 1;
    let hp = 1;
    let defn = 1;
    let nivel = 1;
    let tipo = "Fuego";
    let mut pokemon1 =  Fakemon { nombre, codigo, hp, defn, nivel, tipo };

    let app = App::default().set_scheme(AppScheme::Gleam);
    let mut wind = Window::new(200, 200, 600, 500, "Fakemon Battle Center");

    let mut frame = Frame::new(50, 50, 100, 300, "");
    let mut image = SharedImage::load(&PathBuf::from("res/vettel.png"))?;
    image.scale(200, 200, true, true);
    frame.set_image(&image);

    let mut frameF2 = Frame::new(250, 50, 100, 300, "");
    let mut imageF2 = SharedImage::load(&PathBuf::from("res/raikkonen.png"))?;
    imageF2.scale(200, 200, true, true);
    frameF2.set_image(&imageF2);

    let mut frameF3 = Frame::new(450, 50, 100, 300, "");
    let mut imageF3 = SharedImage::load(&PathBuf::from("res/albon.png"))?;
    imageF3.scale(200, 200, true, true);
    frameF3.set_image(&imageF3);

    let mut but = Button::new(50, 240, 80, 40, "Vettel");
    let mut but2 = Button::new(250, 240, 80, 40, "Raikkonen");
    let mut but3 = Button::new(450, 240, 80, 40, "Albon");



    wind.show();

    but.set_callback(Box::new(move || frame.set_label("Meow!")));
    but2.set_callback(Box::new(move || newWindow()));

    app.run()?;
    Ok(())
}


fn newWindow() {
    println!("WOW");
    let mut wind2 = Window::new(200, 200, 400, 300, "Hello from rust2");
    wind2.end();
    wind2.show();
}