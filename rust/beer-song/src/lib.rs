pub fn verse(n: u32) -> String {
    let mut s = String::new();

    if n == 0 {
        s.push_str("No more bottles of beer on the wall, no more bottles of beer.\n");
        s.push_str("Go to the store and buy some more, 99 bottles of beer on the wall.\n");
    } else if n == 1 {
        s.push_str(&format!(
            "{} bottle of beer on the wall, {} bottle of beer.\n",
            n, n
        ));
        s.push_str("Take it down and pass it around, no more bottles of beer on the wall.\n");
    } else {
        s.push_str(&format!(
            "{} bottles of beer on the wall, {} bottles of beer.\n",
            n, n
        ));
        if n == 2 {
            s.push_str(&format!(
                "Take one down and pass it around, {} bottle of beer on the wall.\n",
                n - 1
            ));
        } else {
            s.push_str(&format!(
                "Take one down and pass it around, {} bottles of beer on the wall.\n",
                n - 1
            ));
        }
    }
    s
}

pub fn sing(start: u32, end: u32) -> String {
    // unimplemented!("sing verses {} to {}, inclusive", start, end)
    let mut s = String::new();
    for i in end..=start {
        s.push_str(&verse(start - i + end));
        if i != start {
            s.push_str("\n");
        }
    }

    s
}
