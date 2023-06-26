fn print(v: &i32) -> Option<i32> {
    println!("v={}", v);
    return Some((*v).clone());
}

fn filter_lt10(n: &i32) -> bool {
    n < &10
}

fn main() {
    let mut f = flow::sync::new_flow::<i32>();
    f.filter(&(filter_lt10 as flow::sync::TFilter<i32>))
        .next(&(print as flow::sync::TProcessor<i32>));
    f.send(&1);
    f.send(&099);
}
