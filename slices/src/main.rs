fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
fn main() {
    let mut s = String::from("hello world");
    /*
     * 如果改为这个语句, 编译将不通过.
     * 因为`first`作为一个slice引用(只读不可变),
     * 它的读生命周期将持续到本作用域结束,
     * 但是下边的```s.clear()```中`s`将被作为可变引用
     * 被`clear`借走, 这个时候我们根据borrow原则,
     * 阻止这种行为发生. 因为我们无法保证在`clear`写
     * 操作中我们的只读引用还是否有效.
     *
     * let first = first_word(&s);
     */
    println!("{}", first_word(&s));
    s.clear();
}
