#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

fn square(x: f64) -> f64 {
    x * x
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Circle {
            center: Point(x, y),
            radius: radius,
        }
    }

    // Returning the diameter of a circle with the radius - 2r
    pub fn diameter(self) -> f64 {
        2.0 * self.radius
    }

    // Are of a circle with a radius - pi * r²
    pub fn area(self) -> f64 {
        let pi = std::f64::consts::PI;
        pi * square(self.radius)
    }

    // Checking intersection of 2 circles
    pub fn intersect(self, other: Circle) -> bool {
        // Getting the center coordinates for the 2 circles
        let point1: Point = self.center;
        let point2: Point = other.center;

        // Getting the radius of the 2 circles
        let r1 = self.radius;
        let r2 = other.radius;

        let d: f64 = point1.distance(point2);

        let dist1 = (r1 - r2).abs();
        let dist2 = (r1 + r2).abs();

        // Checking if the 2 circles intersect
        if dist1 <= d && d <= dist2 {
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(self, other: Point) -> f64 {
        let (x1, y1) = (self.0, self.1);
        let (x2, y2) = (other.0, other.1);

        (square(x2 - x1) + square(y2 - y1)).sqrt()
    }
}
