#![allow(dead_code)]

#[cfg(test)]
mod tests {
    #[test]

    fn print() {
        println!("{} days", 31);

        println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

        println!(
            "{subject} {verb} {object}",
            object = "the lazy dog",
            subject = "the quick brown fox",
            verb = "jumps over"
        );

        println!(
            "{} of {:b} people know binary, the other half doesn't",
            1, 2
        );

        println!("{number:>width$}", number = 1, width = 6);

        println!("{number:0>width$}", number = 1, width = 6);

        /*
        error
        println!("My name is {0}, {1} {0}", "Bond");

        #[allow(dead_code)]
        struct Structure(i32);

        println!("This struct `{}` won't print...", Structure(3));
        */
    }

    #[test]
    fn size() {
        let x = 1u8;
        let y = 2u32;
        let z = 3f32;

        // サフィックスを指定しないリテラル。型は使用方法に依存する。
        let i = 1;
        let f = 1.0;

        // `size_of_val` 関数は変数のサイズをバイトで返す。
        println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
        println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
        println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
        println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
        println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
    }

    struct Circle {
        radius: i32,
    }

    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Circle of radius {}", self.radius)
        }
    }
}

struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    // 長方形は座標空間上における左上隅と右下隅の位置によって指定できる
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn rect_area(&self) -> f32 {
        let Rectangle {
            top_left: Point { x: x1, y: y1 },
            bottom_right: Point { x: x2, y: y2 },
        } = self;

        ((x2 - x1) * (y2 - y1)).abs()
    }

    fn square(p: Point, size: f32) -> Rectangle {
        Rectangle {
            top_left: Point {
                x: p.x,
                y: p.y + size,
            },
            bottom_right: Point {
                x: p.x + size,
                y: p.y,
            },
        }
    }
}

enum Error {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

type Result<T> = std::result::Result<T, Error>;
