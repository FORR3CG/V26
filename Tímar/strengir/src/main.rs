fn main() {
    let strengur = String::from("Tskóli");
    let strengur = "Tskóli".to_string();
    let streng_sneid = "Geir";
    let hallo_world = String::from("Hello World!");
    let hallo = &hallo_world[..5];
    let world = &hallo_world[6..];

    println!("{} - {}", hallo, world);
}
