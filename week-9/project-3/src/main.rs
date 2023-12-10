use std::io::Write;

fn main() {

    let commisioners_name = vec!["Aigbogun Alamba Dauda", "Murtala Afeez Bendu", "Okorocha Cloiatus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye", "", ""];
    let ministry:[&str;7] = ["\nInternal Affairs", "Justice", "Defence", "Power & Steel", "Petroleum", "", ""];
    let geopolitical_zone = vec!["\nSouth West", "North East", "South South", "South West", "South East"];
    
    let mut details = std::fs::File::create("data.txt").expect("create failed");
    details.write_all(b"\n");

    details.write_all(b"Each commisioners name thats were convicted");

    for i in 0..commisioners_name.len() {

        details.write_all(commisioners_name[i].as_bytes()).expect("Create failed");
        details.write_all(b"\n").expect("Create failed");
    }

    details.write_all(b"The ministry of each commisioners respectively");

    for m in 0..ministry.len() {

        details.write_all(ministry[m].as_bytes()).expect("Create failed");
        details.write_all(b"\n").expect("Create failed");
    }
    
    details.write_all(b"The geographical zone of each commisioners");

    for v in 0..geopolitical_zone.len() {

        details.write_all(geopolitical_zone[v].as_bytes()).expect("Create failed");
        details.write_all(b"\n").expect("Create failed");
    }
    
    println!("data has been written");
}
