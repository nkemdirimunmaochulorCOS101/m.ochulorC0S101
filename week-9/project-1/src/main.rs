use std::io::Write;

fn main() {

    let lager = vec!["\n33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star", "", ""];
    let stout = vec!["\nLegend", "Turbo King", "Williams", "", "",""];
    let non_alcoholic = vec!["\nMaltina", "Amstel Malta", "Malta Gold", "Fayrouz", "", ""];

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all(b"\n");

        file.write_all(b"Menu for lagers drinks available");

    for i in 0..lager.len() {

        file.write_all(lager[i].as_bytes()).expect("Create failed");
        file.write_all(b"\n").expect("Create failed");
    }

        file.write_all(b"Menu for stout drinks available");
    for k in 0..stout.len() {
        
        file.write_all(stout[k].as_bytes()).expect("Create failed");
        file.write_all(b"\n").expect("Create failed");
    }

        file.write_all(b"Menu for non alcholic drinks available");
    for g in 0..non_alcoholic.len() {
        
        file.write_all(non_alcoholic[g].as_bytes()).expect("Create failed");
        file.write_all(b"\n").expect("Create failed");
    }
    println!("Date has been printed")
    }
