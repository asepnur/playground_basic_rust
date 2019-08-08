pub struct Circle{
   pub x: f64,
   pub y: f64,
   pub radius: f64, 
}

pub struct CircleBuilder{
    pub x: f64, 
    pub y: f64,
    pub radius: f64,
}
impl Circle{
    pub fn area(&self) -> f64{
        std::f64::consts::PI * (self.radius * self.radius)
    }
}
impl CircleBuilder{
    pub fn new() -> CircleBuilder{
       CircleBuilder{x:0.0, y:0.0, radius:0.0}
    }
    pub fn x(&mut self, coordinate: f64) -> &mut CircleBuilder{
        self.x = coordinate;
        self
    }
    pub fn y(&mut self, coordinate: f64) -> &mut CircleBuilder{
        self.y = coordinate;
        self
    }
    pub fn radius(&mut self, radius: f64) ->&mut CircleBuilder{
        self.radius = radius;
        self
    }
    pub fn finalize(&self) -> Circle{
        Circle{
            x: self.x,
            y: self.y,
            radius: self.radius,
        }
    }
}