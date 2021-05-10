use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    // let mut new = BTreeMap::new();
    // h.iter().for_each(|x| {
    //     x.1.iter().for_each(|f| {
    //         new.insert(f.to_ascii_lowercase(), *x.0);
    //     });
    // });
    // new;

    h.iter()
        .flat_map(|(key, value)| value.iter().map(move |c| (c.to_ascii_lowercase(), *key)))
        .collect()
}
