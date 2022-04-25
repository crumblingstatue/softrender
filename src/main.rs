use macroquad::prelude::*;

fn conf() -> Conf {
    Conf {
        window_width: 1024,
        window_height: 1024,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut img = Image::gen_image_color(1920, 1080, BLACK);
    let tex = Texture2D::from_image(&img);
    loop {
        for y in 0..1080 {
            for x in 0..1920 {
                let pix = (x + y) & (x - y);
                let pix = pix as u8;
                img.set_pixel(x, y, Color::from_rgba(pix, pix, pix, 255));
            }
        }
        tex.update(&img);
        draw_texture(tex, 0., 0., WHITE);
        next_frame().await
    }
}
