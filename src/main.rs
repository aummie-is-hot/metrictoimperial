/*
By: <Aum markandey>
Date: 2026-09-22
Program Details: <metric to imperial conversion calculator with a simple GUI for bobs company>
*/

mod ui;
mod utils;

//use crate::ui::grid::draw_grid;
use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::ui::text_button::TextButton;
use macroquad::prelude::*;
use crate::utils::preload_image::TextureManager;
use crate::utils::preload_image::LoadingScreenOptions; // If you want to customize the loading screen
use crate::utils::preload_image::GifLoadingScreenInfo;


use crate::utils::scale::use_virtual_resolution; // If you want to add animated GIFs to loading screen
use crate::ui::text_input::TextInput;
/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "Metric to Imperial Converter".to_string(),
        window_width: 1700,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let texture_manager = TextureManager::new();
    texture_manager.preload_with_loading_screen(&["assets/edward.png"], None, None).await;
    
   
    let img_edward = StillImage::from_preload(
        texture_manager.get_preload("assets/edward.png").unwrap(),
        1700.0,
        768.0,
        0.0,
        0.0,
        true,
        1.0,
    );
let mut input_unit  = TextInput::new(500.0, 300.0, 150.0, 40.0, 25.0);

let mut total: f64 = 0.0;
input_unit.set_allowed_chars("0123456789");


    let mut btn_exit = TextButton::new(1450.0, 650.0, 200.0, 60.0, "Exit", WHITE, RED, 30);
    let mut lbl_text = Label::new("First type the value of the units you want to convert \nthen click each button to change which unit your converting from \nthen click a second button to pick which unit you want to convert to", 10.0, 100.0, 30);
    let mut btn_meters = TextButton::new(50.0, 350.0, 200.0, 60.0, "Meters", WHITE, RED, 30);
    let mut btn_centimeters = TextButton::new(50.0, 450.0, 200.0, 60.0, "Centimeters", WHITE, RED, 30);
    let mut btn_feet = TextButton::new(50.0, 550.0, 200.0, 60.0, "Feet", WHITE, RED, 30);
    let mut btn_inches = TextButton::new(50.0, 650.0, 200.0, 60.0, "Inches", WHITE, RED, 30);
    let mut btn_calc_total = TextButton::new(50.0, 200.0, 200.0, 60.0, "Calculate Total", WHITE, RED, 30);
    

    btn_calc_total.with_text_color(BLACK); // Sets the normal text color
    btn_calc_total.with_hover_text_color(WHITE);
  
    btn_exit.with_text_color(BLACK); // Sets the normal text color
    btn_exit.with_hover_text_color(WHITE);
    btn_centimeters.with_text_color(BLACK); // Sets the normal text color
    btn_centimeters.with_hover_text_color(WHITE);
    btn_feet.with_text_color(BLACK); // Sets the normal text color
    btn_feet.with_hover_text_color(WHITE);
    btn_inches.with_text_color(BLACK); // Sets the normal text color
    btn_inches.with_hover_text_color(WHITE);
    btn_meters.with_text_color(BLACK); // Sets the normal text color
    btn_meters.with_hover_text_color(WHITE);
    
let mut edward: bool = false;
    loop {
        clear_background(WHITE);
        //draw_grid(50.0, BROWN);
        use_virtual_resolution(1700.0, 768.0);
        if btn_calc_total.click() {
            
            lbl_text.set_text("Calculating...");
            
            let unit_text = input_unit.get_text();
            let input_unit = unit_text.trim().parse::<f64>();
            
            
           
            
            lbl_text.set_text(format!("Total: ${:.2}", total));
           // btn_calc_change.enabled = true;
    
   
        };
        if input_unit.get_text().is_empty(){
            btn_inches.enabled = false;
    btn_calc_total.enabled = false;
    btn_feet.enabled =false;
    btn_centimeters.enabled = false;
    btn_meters.enabled = false;
        } else {
            btn_inches.enabled = true;
    btn_calc_total.enabled = true;
    btn_feet.enabled =true;
    btn_centimeters.enabled = true;
    btn_meters.enabled = true;
        }
        
        if btn_exit.click() {
            break;
        };
        
       if btn_meters.click() {
           
        };
        if btn_centimeters.click() {
           
        };
        if btn_feet.click() {
            
        };
        if btn_inches.click() {
            
        };
        
        input_unit.draw();
        
       
       
        
        
       
        
         if edward == true {
            img_edward.draw();
            lbl_text.set_text("i dont know him him");
            lbl_text.with_colors(WHITE, Some(DARKGRAY));
        }
        lbl_text.draw();
        next_frame().await;
       
    }
}
