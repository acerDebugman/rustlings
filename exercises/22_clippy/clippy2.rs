fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: Fix the Clippy lint.
    // for x in option.into_iter() {
    // let mut iter = option.into_iter();
    // while let Some(x) = iter.next() {
    for x in option.into_iter() {
        res += x;
    }

    println!("{res}");
}
