use std::io::Write;

fn main() {

    let student_name = vec!["\nOluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh", "", ""];
    let matric_number = vec!["\nACC10211111", "ECO10110101", "CSC10328828", "EE11020202", "MEE10202001", "", ""];
    let department = vec!["\nAccounting", "Economics", "Computer", "Electrical", "Mechanical", "", ""];
    let level = vec!["\n300","100","200","200","100"];

    let mut details = std::fs::File::create("data.txt").expect("create failed");
    details.write_all(b"\n");

    details.write_all(b"Details of each stundets name");

    for i in 0..student_name.len() {

        details.write_all(student_name[i].as_bytes()).expect("Create failed");
        details.write_all(b"\n").expect("Create failed");
    }

    details.write_all(b"Details of each students matriculation number respectively");

    for m in 0..matric_number.len() {

        details.write_all(matric_number[m].as_bytes()).expect("Create failed");
        details.write_all(b"\n").expect("Create failed");
    }
    
    details.write_all(b"Details of each stundents departments respectively");

    for v in 0..department.len() {

        details.write_all(department[v].as_bytes()).expect("Create failed");
        details.write_all(b"\n").expect("Create failed");
    }
    
    details.write_all(b"Details of each stundets level respectively");

    for w in 0..level.len() {

        details.write_all(level[w].as_bytes()).expect("Create failed");
        details.write_all(b"\n").expect("Create failed");
    }

    println!("data has been written");
}