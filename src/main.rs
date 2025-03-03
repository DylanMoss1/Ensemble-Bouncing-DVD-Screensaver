use nannou::image;
use nannou::image::{DynamicImage, GenericImageView};
use nannou::prelude::*;
use nannou::rand::{thread_rng, Rng};

const VELOCITY: f32 = 200.0;
const IMAGE_SIZE_SCALE: f32 = 2.0;

struct Model {
    image: DynamicImage,
    dvd_rect: Rect,
    dvd_vel: Vec2,
}

fn change_color(image: &DynamicImage) -> DynamicImage {
    image.huerotate(thread_rng().gen_range(100..150))
}

fn model(app: &App) -> Model {
    let primary_window_id = app
        .new_window()
        .event(window_event)
        .view(view)
        .fullscreen()
        .build()
        .unwrap();

    let primary_window = app.window(primary_window_id).unwrap();
    primary_window.set_cursor_visible(false);

    let img_data = include_bytes!("../assets/ensemble.png");
    let image = change_color(&image::load_from_memory(img_data).unwrap().thumbnail(
        app.window_rect().w() as u32 / 6,
        app.window_rect().h() as u32 / 6,
    ));
    let rect = Rect::from_x_y_w_h(
        0.0,
        0.0,
        (image.dimensions().0 as f32) * IMAGE_SIZE_SCALE,
        (image.dimensions().1 as f32) * IMAGE_SIZE_SCALE,
    );

    Model {
        image,
        dvd_rect: rect,
        dvd_vel: Vec2::new(VELOCITY, VELOCITY),
    }
}

fn window_event(app: &App, _: &mut Model, event: WindowEvent) {
    if app.time > 0.1 {
        match event {
            WindowEvent::MousePressed(..)
            | WindowEvent::KeyPressed(..)
            | WindowEvent::MouseWheel(..) => app.quit(),
            _ => (),
        }
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win = app.window_rect();
    let delta_time = app.duration.since_prev_update.secs() as f32;
    let dvd_vel = &mut model.dvd_vel;

    model.dvd_rect = model
        .dvd_rect
        .shift_x(dvd_vel.x * delta_time)
        .shift_y(dvd_vel.y * delta_time);

    if model.dvd_rect.left() <= win.left() || model.dvd_rect.right() >= win.right() {
        dvd_vel.x = -dvd_vel.x;
        model.image = change_color(&model.image);
    }
    if model.dvd_rect.bottom() <= win.bottom() || model.dvd_rect.top() >= win.top() {
        dvd_vel.y = -dvd_vel.y;
        model.image = change_color(&model.image);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let texture = wgpu::Texture::from_image(app, &model.image);

    draw.texture(&texture)
        .xy(model.dvd_rect.xy())
        .wh(model.dvd_rect.wh());

    frame.clear(BLACK);
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}
