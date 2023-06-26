use flow::sync::{new_flow, send_many, TFilter, TObserver};

fn filter_gt300(n: &i32) -> bool {
    n > &300
}

fn observer_in(n: &i32) {
    println!("n={}", n)
}
fn observer_filtered(n: &i32) {
    println!("passed n={}", n)
}

fn main() {
    let mut f = new_flow::<i32>();
    f.peep(&(observer_in as TObserver<i32>))
        .filter(&(filter_gt300 as TFilter<i32>))
        .peep(&(observer_filtered as TObserver<i32>));

    f.send(&1);
    f.send(&099);
    send_many(&f, [1, 2, 3, 4, 5].into_iter());
    send_many(&f, 290..310);
}
