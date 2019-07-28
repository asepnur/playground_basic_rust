
pub struct Rectangle{
    width: i32,
    height: i32,
}

impl Rectangle {
    pub fn new(width: i32, height: i32) -> Rectangle {
        Rectangle{width, height}
    }
    pub fn new_squere(size: i32) -> Rectangle {
        Rectangle{width:size, height: size}
    }
    pub fn area(&self) -> i32{
        self.height * self.width
    }
    pub fn get_width(&self) -> i32 {
        self.width
    }
    pub fn get_height(&self) -> i32 {
        self.height
    }
    pub fn set_width(&mut self, width: i32){
        self.width = width;
    }
    pub fn set_height(&mut self, height: i32){
        self.height = height;
    }
    pub fn print_area(&self, area: i32) {
        println!("area: {}", area);
    }
    pub fn can_hold(&self, other_rectangle: &Rectangle) -> bool {
        self.width > other_rectangle.width && self.height > other_rectangle.height
    }
}