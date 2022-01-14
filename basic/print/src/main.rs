#![allow(unused)]
mod display;
mod format;
mod list;

use std::fmt;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_println() {
    println!("{} days", 31); //31 days
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob"); //Alice, this is Bob. Bob, this is Alice
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    ); //the quick brown fox jumps over the lazy dog
    println!("{} of {:b} people know binary, the other half don't", 1, 2); //1 of 10 people know binary, the other half don't
    println!("{number:>width$}", number = 1, width = 6); //"     1"
    println!("{number:>0width$}", number = 1, width = 6); //000001
    let pi = 3.141592;
    println!("{0:.2}, {0:.3}", pi); //3.14,3.142
}

#[test]
fn test_format() {
    format!("Hello"); // => "Hello"
    format!("Hello, {}!", "world"); // => "Hello, world!"
    format!("The number is {}", 1); // => "The number is 1"
    format!("{:?}", (3, 4)); // => "(3, 4)"
    format!("{value}", value = 4); // => "4"
    let people = "Rustaceans";
    format!("Hello {people}!", people = people); // => "Hello Rustaceans!"
    format!("{} {}", 1, 2); // => "1 2"
    format!("{:04}", 42); // => "0042" with leading zeros
    format!("{:#?}", (100, 200)); // => "(
                                  //       100,
                                  //       200,
                                  //     )"
}

#[test]
fn test_width() {
    //minimum width 最小宽度
    println!("Hello {:5}!", "x");
    println!("Hello {:1$}!", "x", 5);
    println!("Hello {1:0$}!", 5, "x");
    println!("Hello {:width$}!", "x", width = 5);
    // let width = 5;
    // println!("Hello {:width$}!", "x");

    assert_eq!(format!("Hello {:<5}!", "x"), "Hello x    !");
    assert_eq!(format!("Hello {:-<5}!", "x"), "Hello x----!");
    assert_eq!(format!("Hello {:^5}!", "x"), "Hello   x  !");
    assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!");
}

#[test]
fn test_math() {
    assert_eq!(format!("Hello {:+}!", 5), "Hello +5!");
    assert_eq!(format!("{:#x}!", 27), "0x1b!");
    assert_eq!(format!("Hello {:05}!", 5), "Hello 00005!");
    assert_eq!(format!("Hello {:05}!", -5), "Hello -0005!");
    assert_eq!(format!("{:#010x}!", 27), "0x0000001b!");
}

#[test]
fn test_float() {
    //Hello x is 0.01000

    // Hello {arg 0 ("x")} is {arg 1 (0.01) with precision specified inline (5)}
    println!("Hello {0} is {1:.5}", "x", 0.01);

    // Hello {arg 1 ("x")} is {arg 2 (0.01) with precision specified in arg 0 (5)}
    println!("Hello {1} is {2:.0$}", 5, "x", 0.01);

    // Hello {arg 0 ("x")} is {arg 2 (0.01) with precision specified in arg 1 (5)}
    println!("Hello {0} is {2:.1$}", "x", 5, 0.01);

    // Hello {next arg ("x")} is {second of next two args (0.01) with precision
    //                          specified in first of next two args (5)}
    println!("Hello {} is {:.*}", "x", 5, 0.01);

    // Hello {next arg ("x")} is {arg 2 (0.01) with precision
    //                          specified in its predecessor (5)}
    println!("Hello {} is {2:.*}", "x", 5, 0.01);

    // Hello {next arg ("x")} is {arg "number" (0.01) with precision specified
    //                          in arg "prec" (5)}
    println!("Hello {} is {number:.prec$}", "x", prec = 5, number = 0.01);

    println!(
        "{}, `{name:.*}` has 3 fractional digits",
        "Hello",
        3,
        name = 1234.56
    );
    println!(
        "{}, `{name:.*}` has 3 characters",
        "Hello",
        3,
        name = "1234.56"
    );
    println!(
        "{}, `{name:>8.*}` has 3 right-aligned characters",
        "Hello",
        3,
        name = "1234.56"
    );
}

#[derive(Debug)]
struct Vector2D {
    x: isize,
    y: isize,
}

impl fmt::Display for Vector2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Different traits allow different forms of output of a type. The meaning
// of this format is to print the magnitude of a vector.
impl fmt::Binary for Vector2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let magnitude = (self.x * self.x + self.y * self.y) as f64;
        let magnitude = magnitude.sqrt();

        // Respect the formatting flags by using the helper method
        // `pad_integral` on the Formatter object. See the method
        // documentation for details, and the function `pad` can be used
        // to pad strings.
        let decimals = f.precision().unwrap_or(3);
        let string = format!("{:.*}", decimals, magnitude);
        f.pad_integral(true, "", &string)
    }
}
#[test]
fn test_vector() {
    let myvector = Vector2D { x: 3, y: 4 };

    println!("{}", myvector); // => "(3, 4)"
    println!("{:?}", myvector); // => "Vector2D {x: 3, y:4}"
    println!("{:10.3b}", myvector); //"     5.000"
}
