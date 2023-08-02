use cli_clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    let mut ctx = ClipboardContext::new().unwrap();
    let input = ctx.get_contents().unwrap();

    println!("{}", r2v_lib::rdata_to_rs(input));
}
