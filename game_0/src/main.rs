use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {

}

fn view(app: &App, _model: &Model, frame: Frame){
    let draw = app.draw();
    
    let mut radius = 0.10;

    let points = (0..36000).map(|i| {
        let radian = deg_to_rad(i as f32);

        let x = radian.sin() * radius;
        let y = radian.cos() * radius;

        radius += 0.10;
        (pt2(x,y), WHITE)
    });

    draw.polyline()
        .weight(15.0)
        .points_colored(points);
    draw.to_frame(app, &frame).unwrap();
}

