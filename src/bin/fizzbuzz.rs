use flow::sync::{new_flow, segregate, send_many, TClassificator, TObserver};

type Number = u64;

fn main() {
    let mut flow = new_flow::<Number>();

    let [flow_fb, flow_f, flow_b, flow_n] = segregate::<Number, &str>(
        &mut flow,
        &(fizzbuzzer as TClassificator<Number, &str>),
        vec!["fb","f", "b", "n"],
    ) else {todo!("invalid number of flows")};

    flow_fb.peep(&(print_fizzbuzz as TObserver<Number>));
    flow_f.peep(&(print_fizz as TObserver<Number>));
    flow_b.peep(&(print_buzz as TObserver<Number>));
    flow_n.peep(&(print_n as TObserver<Number>));

    send_many(&flow, 1..=100);
}

fn fizzbuzzer(n: &Number) -> Vec<&'static str> {
    if n % 15 == 0 {
        return vec!["fb"];
    }
    if n % 3 == 0 {
        return vec!["f"];
    }
    if n % 5 == 0 {
        return vec!["b"];
    }
    return vec!["n"];
}

fn print_fizzbuzz(_: &Number) {
    println!("FizzBuzz")
}

fn print_fizz(_: &Number) {
    println!("Fizz");
}

fn print_buzz(_: &Number) {
    println!("Buzz");
}

fn print_n(n: &Number) {
    println!("{}", n);
}
