pub fn hello_world_main() {
    extern "C" { fn print_hello_world(); }

    unsafe { print_hello_world(); }
}