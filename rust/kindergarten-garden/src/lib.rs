const STUDENTS: [&str;12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let diagram_idx = STUDENTS.iter().position(|&s| s == student).unwrap() * 2;
    diagram.lines()
        .flat_map(|line| 
            line.chars().skip(diagram_idx).take(2).map(|cup| match cup {
                'G' => "grass",
                'C' => "clover",
                'R' => "radishes",
                _ => "violets",
            })
        )
        .collect()
}
