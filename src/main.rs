use std::f64::consts::PI;

use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();
    //turtle.pen_up();
    //turtle.backward(280.0);
    //turtle.left(90.0);
    //turtle.pen_down();

    turtle.set_speed(25);
    leftHand(&mut turtle);
    body(&mut turtle);
    nose(&mut turtle);
    mouth(&mut turtle);
    rainbow_circle(&mut turtle);
    love_heart(&mut turtle);
    five_circles(&mut turtle);
    turtle.hide();
}

fn body(turtle: &mut Turtle){
    turtle.pen_up();
    turtle.go_to([-73.0, 230.0]);
    turtle.set_pen_color("light grey");
    turtle.set_pen_size(3.0);
    turtle.set_fill_color("white");
    turtle.begin_fill();
    turtle.pen_down();
    turtle.set_heading(20.0);
    arc(turtle,-250.0, 35.0, 60);
    // 左耳
    turtle.set_heading(50.0);
    arc(turtle,-42.0, 180.0, 60);
    // 左侧
    turtle.set_heading(-50.0);
    arc(turtle,-190.0, 30.0 ,60);
    arc(turtle,-320.0, 45.0 ,60);
    // 左腿
    arc(turtle,120.0, 30.0 ,60);
    arc(turtle,200.0, 12.0 ,60);
    arc(turtle,-18.0, 85.0 ,60);
    arc(turtle,-180.0, 23.0,60);
    arc(turtle,-20.0, 110.0,60);
    arc(turtle,15.0, 115.0,60);
    arc(turtle,100.0, 12.0,60);
    // 右腿;
    arc(turtle,15.0, 120.0,60);
    arc(turtle,-15.0, 110.0,60);
    arc(turtle,-150.0, 30.0,60);
    arc(turtle,-15.0, 70.0,60);
    arc(turtle,-150.0, 10.0,60);
    arc(turtle,200.0, 35.0,60);
    arc(turtle,-150.0, 20.0,60);
    // 右手
    turtle.set_heading(-120.0);
    arc(turtle,50.0, 30.0,60);
    arc(turtle,-35.0, 200.0,60);
    arc(turtle,-300.0, 23.0,60);
    // 右侧
    turtle.set_heading(86.0);
    arc(turtle,-300.0, 26.0,60);
    // 右耳
    turtle.set_heading(122.0);
    arc(turtle,-53.0, 160.0,60);
    turtle.end_fill();
    
    // 右耳内
    turtle.pen_up();
    turtle.go_to([-130.0, 180.0]);
    turtle.set_pen_color("black");
    turtle.set_pen_size(1.0);
    turtle.set_fill_color("black");
    turtle.begin_fill();
    turtle.pen_down();
    turtle.set_heading(120.0);
    arc(turtle,-28.0, 160.0,60);
    turtle.set_heading(210.0);
    arc(turtle,150.0, 20.0,60);
    turtle.end_fill();
    
    // 左耳内
    turtle.pen_up();
    turtle.go_to([90.0, 230.0]);
    turtle.set_heading(40.0);
    turtle.begin_fill();
    turtle.pen_down();
    arc(turtle,-30.0, 170.0,60);
    turtle.set_heading(125.0);
    arc(turtle,150.0, 23.0,60);
    turtle.end_fill();
    
    // 右手内
    turtle.pen_up();
    turtle.go_to([-180.0, -55.0]);
    turtle.set_fill_color("black");
    turtle.begin_fill();
    turtle.set_heading(-120.0);
    turtle.pen_down();
    arc(turtle,50.0, 30.0,60);
    arc(turtle,-27.0, 200.0,60);
    arc(turtle,-300.0, 20.0,60);
    turtle.set_heading(-90.0);
    arc(turtle,300.0, 14.0,60);
    turtle.end_fill();
    
    // 左腿内
    turtle.pen_up();
    turtle.go_to([108.0, -168.0]);
    turtle.set_fill_color("black");
    turtle.begin_fill();
    turtle.pen_down();
    turtle.set_heading(-115.0);
    arc(turtle,110.0, 15.0,60);
    arc(turtle,200.0, 10.0,60);
    arc(turtle,-18.0, 80.0,60);
    arc(turtle,-180.0, 13.0,60);
    arc(turtle,-20.0, 90.0,60);
    arc(turtle,15.0, 60.0,60);
    turtle.set_heading(42.0);
    arc(turtle,-200.0, 29.0,60);
    turtle.end_fill();
    // 右腿内
    turtle.pen_up();
    turtle.go_to([-38.0, -210.0]);
    turtle.set_fill_color("black");
    turtle.begin_fill();
    turtle.pen_down();
    turtle.set_heading(-155.0);
    arc(turtle,15.0, 100.0,60);
    arc(turtle,-10.0, 110.0,60);
    arc(turtle,-100.0, 30.0,60);
    arc(turtle,-15.0, 65.0,60);
    arc(turtle,-100.0, 10.0,60);
    arc(turtle,200.0, 15.0,60);
    turtle.set_heading(-14.0);
    arc(turtle,-200.0, 27.0,60);
    turtle.end_fill();
    
    // 右眼
    // 眼圈
    turtle.pen_up();
    turtle.go_to([-64.0, 120.0]);
    turtle.begin_fill();
    turtle.pen_down();
    turtle.set_heading(40.0);
    arc(turtle,-35.0, 152.0,60);
    arc(turtle,-100.0, 50.0,60);
    arc(turtle,-35.0, 130.0,60);
    arc(turtle,-100.0, 50.0,60);
    turtle.end_fill();
    // 眼珠
    turtle.pen_up();
    turtle.go_to([-47.0, 55.0]);
    turtle.set_fill_color("white");
    turtle.begin_fill();
    turtle.pen_down();
    turtle.set_heading(0.0);
    arc(turtle,25.0, 360.0 ,60);
    turtle.end_fill();
    turtle.pen_up();
    turtle.go_to([-45.0, 62.0]);
    turtle.set_pen_color("dark grey");
    turtle.set_fill_color("dark grey");
    turtle.begin_fill();
    turtle.pen_down();
    turtle.set_heading(0.0);
    arc(turtle,19.0, 360.0,60);
    turtle.end_fill();
    turtle.pen_up();
    turtle.go_to([-45.0, 68.0]);
    turtle.set_fill_color("black");
    turtle.begin_fill();
    turtle.pen_down();
    turtle.set_heading(0.0);
    arc(turtle,10.0, 360.0,60);
    turtle.end_fill();
    turtle.pen_up();
    turtle.go_to([-47.0, 86.0]);
    turtle.set_pen_color("white");
    turtle.set_fill_color("white");
    turtle.begin_fill();
    turtle.pen_down();
    turtle.set_heading(0.0);
    arc(turtle,5.0, 360.0,60);
    turtle.end_fill();
    
    // 左眼
    // 眼圈
    turtle.pen_up();
    turtle.go_to([51.0, 82.0]);
    turtle.set_fill_color("black");
    turtle.begin_fill();
    turtle.pen_down();
    turtle.set_heading(120.0);
    arc(turtle,-32.0, 152.0,60);
    arc(turtle,-100.0, 55.0,60);
    arc(turtle,-25.0, 120.0,60);
    arc(turtle,-120.0, 45.0,60);
    turtle.end_fill();
    // 眼珠
    turtle.pen_up();
    turtle.go_to([79.0, 60.0]);
    turtle.set_fill_color("white");
    turtle.begin_fill();
    turtle.pen_down();
    turtle.set_heading(0.0);
    arc(turtle,24.0, 360.0,60);
    turtle.end_fill();
    turtle.pen_up();
    turtle.go_to([79.0, 64.0]);
    turtle.set_pen_color("dark grey");
    turtle.set_fill_color("dark grey");
    turtle.begin_fill();
    turtle.pen_down();
    turtle.set_heading(0.0);
    arc(turtle,19.0, 360.0,60);
    turtle.end_fill();
    turtle.pen_up();
    turtle.go_to([79.0, 70.0]);
    turtle.set_fill_color("black");
    turtle.begin_fill();
    turtle.pen_down();
    turtle.set_heading(0.0);
    arc(turtle,10.0, 360.0,60);
    turtle.end_fill();
    turtle.pen_up();
    turtle.go_to([79.0, 88.0]);
    turtle.set_pen_color("white");
    turtle.set_fill_color("white");
    turtle.begin_fill();
    turtle.pen_down();
    turtle.set_heading(0.0);
    arc(turtle,5.0, 360.0,60);
    turtle.end_fill() 
}

fn nose(turtle: &mut Turtle){
    turtle.pen_up();
    turtle.go_to([37.0, 80.0]);
    turtle.set_fill_color("black");
    turtle.begin_fill();
    turtle.pen_down();
    arc(turtle,-8.0, 130.0 ,60);
    arc(turtle,-22.0, 100.0,60);
    arc(turtle,-8.0, 130.0,60);
    turtle.end_fill();

}
fn mouth(turtle: &mut Turtle){ 
    turtle.pen_up();
    turtle.go_to([-15.0, 48.0]);
    turtle.set_heading(-36.0);
    turtle.begin_fill();
    turtle.pen_down();
    arc(turtle,60.0, 70.0,60);
    turtle.set_heading(-132.0);
    arc(turtle,-45.0, 100.0,60);
    turtle.end_fill();
}

fn rainbow_circle(turtle: &mut Turtle){ 
    turtle.pen_up();
    turtle.go_to([-135.0, 120.0]);
    turtle.set_pen_size(5.0);
    turtle.set_pen_color("cyan");
    turtle.pen_down();
    turtle.set_heading(60.0);
    arc(turtle,-165.0, 150.0 ,60);
    arc(turtle,-130.0, 78.0 ,60);
    arc(turtle,-250.0, 30.0 ,60);
    arc(turtle,-138.0, 105.0 ,60);
    turtle.pen_up();
    turtle.go_to([-131.0, 116.0]);
    turtle.set_pen_color("slate blue");
    turtle.pen_down();
    turtle.set_heading(60.0);
    arc(turtle,-160.0, 144.0 ,60);
    arc(turtle,-120.0, 78.0 ,60);
    arc(turtle,-242.0, 30.0 ,60);
    arc(turtle,-135.0, 105.0 ,60);
    turtle.pen_up();
    turtle.go_to([-127.0, 112.0]);
    turtle.set_pen_color("orange red");
    turtle.pen_down();
    turtle.set_heading(60.0);
    arc(turtle,-155.0, 136.0,60);
    arc(turtle,-116.0, 86.0,60);
    arc(turtle,-220.0, 30.0,60);
    arc(turtle,-134.0, 103.0,60);
    turtle.pen_up();
    turtle.go_to([-123.0, 108.0]);
    turtle.set_pen_color("gold");
    turtle.pen_down();
    turtle.set_heading(60.0);
    arc(turtle,-150.0, 136.0,60);
    arc(turtle,-104.0, 86.0,60);
    arc(turtle,-220.0, 30.0,60);
    arc(turtle,-126.0, 102.0,60);
    turtle.pen_up();
    turtle.go_to([-120.0, 104.0]);
    turtle.set_pen_color("green yellow");
    turtle.pen_down();
    turtle.set_heading(60.0);
    arc(turtle,-145.0, 136.0,60);
    arc(turtle,-90.0, 83.0,60);
    arc(turtle,-220.0, 30.0,60);
    arc(turtle,-120.0, 100.0,60);
    turtle.pen_up();    
}


fn love_heart(turtle: &mut Turtle){ 
    turtle.pen_up();
    turtle.go_to([220.0, 115.0]);
    turtle.set_pen_color("brown");
    turtle.set_pen_size(1.0);
    turtle.set_fill_color("brown");
    turtle.begin_fill();
    turtle.pen_down();
    turtle.set_heading(36.0);
    arc(turtle,-8.0, 180.0,60);
    arc(turtle,-60.0, 24.0,60);
    turtle.set_heading(110.0);
    arc(turtle,-60.0, 24.0, 60);
    arc(turtle,-8.0, 180.0,60);
    turtle.end_fill();
}

fn five_circles(turtle: &mut Turtle){ 
    turtle.pen_up();
    turtle.go_to([-5.0, -170.0]);
    turtle.pen_down();
    turtle.set_pen_color("blue");
    circle(turtle, 6.0);
    turtle.pen_up();
    turtle.go_to([10.0, -170.0]);
    turtle.pen_down();
    turtle.set_pen_color("black");
    circle(turtle,6.0);
    turtle.pen_up();
    turtle.go_to([25.0, -170.0]);
    turtle.pen_down();
    turtle.set_pen_color("red");
    circle(turtle,6.0);
    turtle.pen_up();
    turtle.go_to([2.0, -175.0]);
    turtle.pen_down();
    turtle.set_pen_color("brown");
    circle(turtle,6.0);
    turtle.pen_up();
    turtle.go_to([16.0, -175.0]);
    turtle.pen_down();
    turtle.set_pen_color("green");
    circle(turtle,6.0);
    turtle.pen_up();
    
    turtle.set_pen_color("black");
    turtle.go_to([-16.0, -160.0]);
    //turtle.write("BEIJING 2022", font=('Arial', 10, 'bold italic'));
    //turtle.hideturtle();
    
}

fn leftHand(turtle: &mut Turtle) { 
    turtle.pen_up();
    turtle.go_to([177.0, 112.0]);
    turtle.set_pen_color("light grey");
    turtle.set_pen_size(3.0);
    turtle.set_fill_color("white");
    turtle.begin_fill();
    turtle.pen_down();
    turtle.set_heading(80.0);
    arc(turtle,-45.0, 200.0,60);
    arc(turtle,-300.0, 23.0,60);
    turtle.end_fill();

    turtle.pen_up();
    turtle.go_to([182.0, 95.0]);
    turtle.set_pen_color("black");
    turtle.set_pen_size(1.0);
    turtle.set_fill_color("black");
    turtle.begin_fill();
    turtle.set_heading(95.0);
    turtle.pen_down();
    arc(turtle,-37.0, 160.0,60);
    arc(turtle,-20.0, 50.0,60);
    arc(turtle,-200.0, 30.0,60);
    turtle.end_fill();
}

fn circle(turtle: &mut Turtle, radius: f64) {
    arc(turtle, radius, 360.0, 180);
}

fn arc(turtle: &mut Turtle, radius: f64, extent: f64, steps: u32){
    let circumference = 2.0 * PI * radius;
    let distance = circumference * extent / 360.0;
    let step = distance / steps as f64;
    let rotation = extent / steps as f64;

    for _ in 0..steps {
        turtle.forward(step);
        turtle.right(rotation);
    }
}