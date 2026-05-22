fn main() {
    'outer: for x in 1..5 {
        println!("x: {x}");
        let mut i = 0;
        while i < x {
            println!("x: {x}, i: {i}");
            i += 1;
            if i == 3 {
                break 'outer;
            }
        }
    }

    let mut point = (1, 2);
    let mut x_coord = &mut point.0;
    *x_coord = 20;

    x_coord = &mut point.1;
    *x_coord = 10;
    println!("point: {point:?}");
}
