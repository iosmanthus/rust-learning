extern crate group;
use group::MultiplicativeGroup;

fn main() {
    let g = MultiplicativeGroup::new(11);
    println!("11: {:?}", g.iter().filter(|&&x| g.is_generator(x)).count());
    let g = MultiplicativeGroup::new(13);
    println!(
        "13: {:?}",
        g.iter().filter(|&&x| g.is_generator(x)).collect::<Vec<_>>()
    );
}
