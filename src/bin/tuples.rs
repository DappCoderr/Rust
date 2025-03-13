#![allow(unused)]

fn main() {
    // defining the tuple
    let t: (bool, i32, char) = (true, 45, 'a');

    // destructuring the tuple
    let (a, b, c) = t;

    println!("Print the value of a: {}", t.0);
    println!("Print the value of b: {}", t.1);
    println!("Print the value of b: {}", t.2);

    // ignore with _

    let (_, b, _) = t;

    // empty tuple

    let t = ();

    // nested tuple
    let nested = ((1.23, 'a'), (true, 1u32, 'b'), ());

    println!("value of tuples: {}", nested.0 .0);
    println!("value of tuples: {}", nested.1 .0);

    println!("--------------------------------------------------------");

    let employ_info: (&str, u8) = ("Haardik", 27);
    let emp_name = employ_info.0;
    let emp_age = employ_info.1;

    let (employee_name, employee_age) = employ_info;

    println!("employee name {} and age {}", emp_name, emp_age);
    println!("employee name {} and age {}", employee_name, employee_age);
}
