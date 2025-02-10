#[derive(Debug)]
struct Category;

#[derive(Debug)]
struct Functor {
    dom: Category,
    cod: Category,
}

fn main() {
    let zero = Category;
    let one = Category;
    let f = Functor { dom: zero, cod: one };
    println!("{:?}", f);
}
