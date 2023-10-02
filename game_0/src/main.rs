use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    rotation_angle: f32,
}

fn model(_app: &App) -> Model {
    Model {
        rotation_angle:0.0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.rotation_angle += 0.15;
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    draw.background().color(WHITE);

    let mut radius = 0.0;

    let points = (0..36000).map(|i| {
        let radian = deg_to_rad(i as f32);

        let x = radian.sin() * radius;
        let y = radian.cos() * radius;

        radius += 0.15;
        (pt2(x,y), BLACK)
    });

    draw.polyline()
        .weight(20.0)
        .rotate(-model.rotation_angle)
        .points_colored(points);
    draw.to_frame(app, &frame).unwrap();
}

