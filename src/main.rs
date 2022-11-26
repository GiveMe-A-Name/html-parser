use html_parser::parser::parse;

fn main() {
    let html = include_str!("../index.html");
    let node = parse(html);
    println!("{:?}", node);
}
