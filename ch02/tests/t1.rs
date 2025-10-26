#[cfg(test)]
mod tests {

    struct Rectangle {
        w: f64,
        h: f64,
    }

    impl Rectangle {
        fn area(&self) -> f64 {
            return self.w * self.h;
        }
    }

    const MY_PI: f64 = std::f64::consts::PI;
    struct Circle {
        r: f64,
    }
    impl Circle {
        fn area(&self) -> f64 {
            return MY_PI * self.r * self.r;
        }
    }

    struct Triangle {
        a: f64,
        b: f64,
        c: f64,
    }

    impl Triangle {
        fn area(&self) -> f64 {
            let s = (self.a + self.b + self.c) / 2.0;
            return (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt();
        }
    }

    enum Shape {
        Rec(Rectangle),
        Tri(Triangle),
        Cir(Circle),
    }

    impl Shape {
        fn area(&self) -> f64 {
            match self {
                Shape::Rec(r) => r.area(),
                Shape::Tri(t) => t.area(),
                Shape::Cir(c) => c.area(),
            }
        }
    }

    #[test]
    fn test_area() {
        let r = Rectangle { w: 5.0, h: 10.0 };
        let t = Triangle {
            a: 3.0,
            b: 4.0,
            c: 5.0,
        };
        let c = Circle { r: 5.0 };
        let shapes = vec![Shape::Rec(r), Shape::Tri(t), Shape::Cir(c)];
        for shape in shapes {
            println!("{}", shape.area());
        }
    }

    //  fn main() {
    //     test_area();
    //  }
}
