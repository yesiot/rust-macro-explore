extern crate my_macros;
use my_macros::show_streams;
use my_macros::reverse_name;


#[show_streams(attr1, attr2, attr3)]
fn inspect_me(var1 : u32, var2: &str) -> String {
    return var1.to_string() + var2;
}

#[reverse_name(test)]
fn rust_is_fun() {
    println!("Called by function");
}

fn main() {
    inspect_me(4, "TEST");
    nuf_si_tsur();
}
