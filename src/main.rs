mod some_module;

use error_unused_code::some_other_module::some_other_function;

fn main() {
    let input = "lorem ipsum";
    let output = some_other_function(input);
    println!("{}", output);
}
