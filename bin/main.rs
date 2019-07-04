use ast_gen::generate;

fn main() {
    let imgs = generate(15000, 3).smoothen_all(1).combine_colored(Some([10, 0, 50])).save_colored("colored.png");
}