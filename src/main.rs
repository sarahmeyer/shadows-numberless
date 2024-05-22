use core::num;

use nannou::prelude::*;
use nannou::rand::Rng;

const WIN_WIDTH: u32 = 1024;
const WIN_HEIGHT: u32 = 768;

const LIGHT_OFFSET_X: f32 = 20.0;
const LIGHT_OFFSET_Y: f32 = 30.0;

const PETAL_RADIUS: f32 = 20.0;

struct Model {
    ellipse_position: Point2,
    random_ellipse_positions: Vec<Point2>,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .mouse_moved(mouse_moved)
        .size(WIN_WIDTH, WIN_HEIGHT)
        .view(view)
        .build()
        .unwrap();

    Model {
        ellipse_position: Point2::new(0.0, 0.0),
        random_ellipse_positions: vec![],
    }
}

fn mouse_moved(_app: &App, model: &mut Model, pos: Point2) {
    model.ellipse_position = pos;
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut rng = nannou::rand::thread_rng();

    model.random_ellipse_positions.push(Point2::new(
        rng.gen_range(-((WIN_WIDTH as i32) / 2)..(WIN_WIDTH as i32) / 2) as f32,
        rng.gen_range(-((WIN_HEIGHT as i32) / 2)..(WIN_HEIGHT as i32) / 2) as f32,
    ));

    if model.random_ellipse_positions.len() > 200 {
        model.random_ellipse_positions = vec![]
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(LIGHTGREEN);

    let translucent_gray = nannou::color::rgba(0.1, 0.1, 0.1, 0.5);
    let mut rng = nannou::rand::thread_rng();

    for p in &model.random_ellipse_positions {
        draw.ellipse()
            .radius(PETAL_RADIUS)
            .color(translucent_gray)
            .x_y(
                p.x + (LIGHT_OFFSET_X + p.x / 10.),
                p.y + (LIGHT_OFFSET_Y + p.y / 10.),
            );
    }
    for p in &model.random_ellipse_positions {
        draw.ellipse()
            .radius(PETAL_RADIUS)
            .color(PINK)
            .x_y(p.x, p.y);
        let mut num_noise_points = 100;
        while num_noise_points > 0 {
            num_noise_points -= 1;

            // let x = rng.gen_range((p.x - (PETAL_RADIUS / 2.))..(p.x + (PETAL_RADIUS / 2.)));
            // let y = rng.gen_range((p.y - (PETAL_RADIUS / 2.))..(p.y + (PETAL_RADIUS / 2.)));
            let r = PETAL_RADIUS * rng.gen::<f32>().powf(0.5);
            let theta: f32 = rng.gen::<f32>() * 2.0 * PI;
            let x: f32 = p.x + r * theta.cos();
            let y = p.y + r * theta.sin();
            draw.ellipse().radius(1.0).color(MAGENTA).x_y(x, y);
        }
    }

    draw.ellipse().color(translucent_gray).x_y(
        model.ellipse_position.x + (LIGHT_OFFSET_X + model.ellipse_position.x / 10.),
        model.ellipse_position.y + (LIGHT_OFFSET_Y + model.ellipse_position.y / 10.),
    );
    draw.ellipse()
        .color(DARKGREEN)
        .x_y(model.ellipse_position.x, model.ellipse_position.y);

    draw.to_frame(app, &frame).unwrap();
}
