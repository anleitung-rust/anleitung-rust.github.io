//! Aufgabe: Drei ineinanderliegende Quadrate
//!
//! Dieses Programm zeichnet drei Quadrate, die ineinander liegen, aber sich nicht berühren.

use turtle_lib::*;

#[turtle_main]
fn main() {
    // Erstes Quadrat (klein, in der Mitte)
    turtle.set_pen_color(RED);
    turtle.set_pen_width(2.0);

    for _ in 0..4 {
        turtle.forward(50.0);
        turtle.right(90.0);
    }

    // Bewege die Schildkröte zur Startposition des zweiten Quadrats
    turtle.pen_up();
    turtle.backward(25.0);
    turtle.left(90.0);
    turtle.forward(25.0);
    turtle.right(90.0);
    turtle.pen_down();

    // Zweites Quadrat (mittel)
    turtle.set_pen_color(BLUE);

    for _ in 0..4 {
        turtle.forward(100.0);
        turtle.right(90.0);
    }

    // Bewege die Schildkröte zur Startposition des dritten Quadrats
    turtle.pen_up();
    turtle.backward(25.0);
    turtle.left(90.0);
    turtle.forward(25.0);
    turtle.right(90.0);
    turtle.pen_down();

    // Drittes Quadrat (groß)
    turtle.set_pen_color(GREEN);

    for _ in 0..4 {
        turtle.forward(150.0);
        turtle.right(90.0);
    }
}
