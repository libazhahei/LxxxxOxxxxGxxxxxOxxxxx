use unsvg::{Image, COLORS};
pub struct Turtle {
    pen: bool,
    x: f32,
    y: f32,
    color: i32,
    direction: i32,
    map: Option<Image>,
}

#[allow(dead_code)]
impl Turtle {
    pub fn new(x: f32, y: f32) -> Turtle {
        Turtle {
            pen: false,
            x,
            y,
            color: 7,
            direction: 0,
            map: None,
        }
    }
    pub fn new_with_img(image: Image) -> Turtle {
        let (width, height) = image.get_dimensions();
        let mut turtle = Turtle::new(width as f32 / 2.0, height as f32 / 2.0);
        turtle.attach_img(image);
        turtle
    }
    pub fn pen(&self) -> bool {
        // println!("PEN: -> {}", self.pen);
        self.pen
    }
    pub fn attach_img(&mut self, map: Image) {
        self.map = Some(map);
    }
    pub fn x(&self) -> f32 {
        // println!("XCOR: -> {}", self.x);
        self.x
    }
    pub fn y(&self) -> f32 {
        // println!("YCOR: -> {}", self.y);
        self.y
    }
    pub fn direction(&self) -> i32 {
        // println!("HEADING -> {}", self.direction);
        self.direction
    }
    pub fn color(&self) -> i32 {
        // println!("COLOR -> {}", self.color);
        self.color
    }
    pub fn pen_up(&mut self) {
        // println!("PENUP");
        self.pen = false;
    }
    pub fn pen_down(&mut self) {
        // println!("PENDOWN");
        self.pen = true;
    }
    pub fn set_color(&mut self, color: i32) {
        // println!("SETCOLOR: {}", color);
        self.color = color;
    }
    pub fn set_x(&mut self, x: f32) {
        // println!("SETX: {}", x);
        self.x = x;
    }
    pub fn set_y(&mut self, y: f32) {
        // println!("SETY: {}", y);
        self.y = y;
    }
    pub fn turn(&mut self, degrees: i32) {
        // println!("TURN: {}", degrees);
        self.direction += degrees;
    }
    pub fn set_direction(&mut self, degrees: i32) {
        // println!("SETHEADING: {}", degrees);
        self.direction = degrees;
    }
    pub fn move_forward(&mut self, distance: f32) {
        // println!("MOVE_FORWARD: {}", distance);
        self.move_(distance, 0);
    }

    pub fn move_back(&mut self, distance: f32) {
        // println!("MOVE_BACK: {}", distance);
        self.move_(distance, 180);
    }
    pub fn move_left(&mut self, distance: f32) {
        // println!("MOVE_LEFT: {}", distance);
        self.move_(distance, 270);
    }
    pub fn move_right(&mut self, distance: f32) {
        // println!("MOVE_RIGHT: {}", distance);
        self.move_(distance, 90);
    }

    fn move_(&mut self, distance: f32, direction_offset: i32) {
        let color = *COLORS.get(self.color as usize).expect("color is not valid");
        if self.pen {
            let map = self.map.as_mut().expect("there is no image was given");
            let (x, y) = map
                .draw_simple_line(
                    self.x,
                    self.y,
                    self.direction + direction_offset,
                    distance,
                    color,
                )
                .unwrap_or_else(|err| panic!("Error: {:?}", err));
            // println!("{} ->: {}, {} ->y: {}", self.x, x, self.y, y);
            self.x = x;
            self.y = y;
        } else {
            let mut map = Image::new(10, 10);
            let (x, y) = map
                .draw_simple_line(
                    self.x,
                    self.y,
                    self.direction + direction_offset,
                    distance,
                    color,
                )
                .unwrap_or_else(|err| panic!("Error: {:?}", err));
            // println!("{} ->: {}, {} ->y: {}", self.x, x, self.y, y);
            self.x = x;
            self.y = y;
        };
    }
    pub fn image(&self) -> &Image {
        self.map.as_ref().expect("there is no image was given")
    }
}
