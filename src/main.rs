use macroquad::prelude::*;
mod field;
use self::field::Field;


const BG_COLOR_DARK: Color = Color::new(0.09, 0.1, 0.11, 1.0);
const BG_COLOR_LIGHT: Color = Color::new(0.12, 0.125, 0.14, 1.0);
const GRID_SIZE: usize = 8;



fn draw_background(tilesize: f32) {
    clear_background(BG_COLOR_DARK);
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            if (x + y) % 2 == 0 {
                draw_rectangle(x as f32 * tilesize, y as f32 * tilesize, tilesize, tilesize, BG_COLOR_LIGHT);
            }
        }
    }
}


#[macroquad::main("Snake")]
async fn main() {
    let font = load_ttf_font("resources/Monaco.ttf").await.unwrap();

    let mut screen_size: f32;
    let mut tile_size: f32;
    let mut field = Field::new(GRID_SIZE);
    loop {
        screen_size = screen_width().min(screen_height());
        tile_size = screen_size / GRID_SIZE as f32;

        draw_background(tile_size);
        field.update();
        field.draw(tile_size);

        draw_text_ex(
            &format!("Score: {}", field.snake.score),
            10.0, 30.0,
            TextParams{font: font, font_size: 24u16, color: Color::new(1.0, 1.0, 1.0, 0.4), ..Default::default()}
        );

        next_frame().await
    }
}
