pub fn read_entry() -> String {
    let mut message = std::string::String::new();
    let stdin_reader = std::io::stdin(); 
    let reader_res = stdin_reader.read_line(&mut message);

    if reader_res.is_err() {
        println!("error! {:?}", reader_res.err());
    }

    message.trim().to_string()
}

pub mod outer_mod {

    pub mod inner_mod {

        // fungsi say_hello berikut hanya bisa diakses dari dalam `outer_mod`.
        // pengaksesannya dari luar `outer_mod` menghasilkan error.
        pub(in super) fn say_hello() {
            println!("hello rust")
        }
    }

    pub fn do_something() {
        inner_mod::say_hello();
    }
}