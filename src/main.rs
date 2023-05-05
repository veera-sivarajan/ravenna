mod request;

fn main() -> std::io::Result<()> {
    let data = request::get("veera.app", "/index.html", 443).unwrap();
    println!("status: {:?}", data.status);
    println!("Header: {:#?}", data.header);
    print!("{data}");
    Ok(())
}
