use draw::*;

const SCREEN_SIZE: (u32, u32) = (5050, 5050);

// change these function to change functions graphed
fn f1(x:f32, y: f32) -> f32 {(  y - 20.0*(5.0*x/20.0).sin()  ) as f32}
fn f2(x:f32, y: f32) -> f32 {(  y - (x*x) - 5.0*x  ) as f32}
 
fn main() {
    let mut canvas: Canvas = Canvas::new(SCREEN_SIZE.0, SCREEN_SIZE.1);
    canvas = draw_grid(canvas);
    
    canvas = draw_background(canvas, RGB { r: 0, g: 0, b: 0 });
    
    canvas = draw_function(canvas, &f1, RGB { r: 255, g: 0, b: 0 });
    canvas = draw_function(canvas, &f2, RGB { r: 0, g: 255, b: 0 });

    
    render_video(canvas, "image/rendered_graph.svg")
}


fn draw_grid(mut canvas: Canvas) -> Canvas {
    for i in 0..101 {
        for j in 0..101 {
            let position: (f32, f32) = ((i*50) as f32, (j*50) as f32);
            canvas.display_list.add(draw_pixel(position.0, position.1));
            
        }
    }
    return canvas;
}


fn draw_function (mut canvas: Canvas, f: &dyn Fn(f32, f32) -> f32, color: RGB) -> Canvas {
    for i in -50..51{
        for j in -50..51 {
            if is_pixel_solution(i as f32, j as f32, f) {
                let position: (f32, f32) = (((i+50)*50) as f32, (((-j)+50)*50) as f32);
                canvas.display_list.add(fill_in_pixel(position.0, position.1, color));
            }
        }
    }
    return canvas;
}


fn render_video(canvas: Canvas, file_path:&str) {
    render::save(
        &canvas,
        file_path,
        SvgRenderer::new(),
    )
    .expect("Failed to save")
}


fn is_positive(number:f32) -> i32 {
    if number > 0.0 {
        return 1;
    }
    return -1;
}


fn is_pixel_solution(x: f32, y: f32, f: &dyn Fn(f32, f32) -> f32) -> bool {

    // this solution was directly ripped from https://www.youtube.com/watch?v=EvvWOaLgKVU&t=1288s
    // but translated to rust

    let z: i32 = is_positive(f(x, y)) + is_positive(f(x-1.0, y-1.0)) + is_positive(f(x-1.0, y)) + is_positive(f(x, y-1.0));
    if z < 4 && z > -4 {
        return true;
    }
    return false;
}


fn draw_pixel(xpos: f32, ypos: f32) -> Drawing {
    let rect: Drawing = Drawing::new()
        .with_shape(Shape::Rectangle {
        width: 50,
        height: 50,
        })
        .with_xy(xpos, ypos)
        .with_style(Style::stroked(5, RGB { r: 255, g: 255, b: 255 }));
    rect
}


fn draw_background(mut canvas: Canvas, color: RGB) -> Canvas {
    let background: Drawing = Drawing::new()
    .with_shape(Shape::Rectangle {
    width: SCREEN_SIZE.0,
    height: SCREEN_SIZE.1,
    })
    .with_style(Style::filled(color));

    canvas.display_list.add(background);
    return canvas;
}


fn fill_in_pixel(xpos: f32, ypos: f32, color: RGB) -> Drawing {
    let rect: Drawing = Drawing::new()
        .with_shape(Shape::Rectangle {
        width: 50,
        height: 50,
        })
        .with_xy(xpos, ypos)
        .with_style(Style::filled(color));
    rect
}
