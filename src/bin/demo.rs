use flow::sync::{new_flow, send_all, TFilter, TProcessor};

fn print(v: &i32) -> Option<i32> {
    println!("v={}", v);
    return Some((*v).clone());
}

fn filter_gt300(n: &i32) -> bool {
    n > &300
}

fn main() {
    let mut f = new_flow::<i32>();
    let gt10 = f.filter(&(filter_gt300 as TFilter<i32>));
    gt10.next(&(print as TProcessor<i32>));

    f.send(&1);
    f.send(&099);
    send_all(&f, [1, 2, 3, 4, 5].into_iter());
    send_all(&f, 1..500);
}
