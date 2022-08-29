mod some_module;

use error_unused_code::some_module::some_function;

fn main() {
    let input = "lorem ipsum";
    let output = some_function(input);
    println!("{}", output);
}
