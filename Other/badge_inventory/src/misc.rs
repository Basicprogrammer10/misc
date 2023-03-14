use std::collections::HashMap;

pub fn collapse_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn best<'a, T>(a: &'a str, b: &'a [T], transformer: fn(&T) -> String) -> Option<&'a T> {
    let mut best = 0.0;
    let mut best_str = None;

    for i in b {
        let sim = similarity(a, &transformer(i));
        if sim > best {
            best = sim;
            best_str = Some(i);
        }
    }

    best_str
}

pub fn similarity(str1: &str, str2: &str) -> f64 {
    let a = str1.replace(' ', "");
    let b = str2.replace(' ', "");
    // Check some simple cases
    if a == b {
        return 1.0;
    }
    if a.len() < 2 || b.len() < 2 {
        return 0.0;
    }
    let mut first_bigrams: HashMap<&str, i32> = HashMap::new();
    for i in 0..a.len() - 1 {
        let bigram = &a[i..i + 2];
        let count = first_bigrams.get(bigram).unwrap_or(&0) + 1;
        first_bigrams.insert(bigram, count);
    }
    let mut intersection_size = 0;
    for i in 0..b.len() - 1 {
        let bigram = &b[i..i + 2];
        let count = *first_bigrams.get(bigram).unwrap_or(&0);
        if count > 0 {
            first_bigrams.insert(bigram, count - 1);
            intersection_size += 1;
        }
    }
    (2.0 * intersection_size as f64) / (str1.len() + str2.len() - 2) as f64
}

pub fn t<T>(expr: bool, a: T, b: T) -> T {
    if expr {
        return a;
    }

    b
}
