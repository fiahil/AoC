use std::collections::HashSet;
use std::fs::read_to_string;
use std::io::Result;
use std::path::Path;

fn display(input: &String) {
    println!("{} (len: {})", input, input.len());
}

fn cancel(c1: u8, c2: u8) -> bool {
    c1.eq_ignore_ascii_case(&c2) && c1 != c2
}

fn reduce(input: Vec<u8>) -> Vec<u8> {
    let mut col = input.clone();

    loop {
        let mut skiplist = HashSet::<usize>::new();

        for i in 0..(col.len() - 1) {
            if cancel(col[i], col[i + 1]) && !skiplist.contains(&i) {
                // println!(
                //     "> {} | {} ({},{})",
                //     col[i] as char,
                //     col[i + 1] as char,
                //     i,
                //     i + 1
                // );

                skiplist.insert(i);
                skiplist.insert(i + 1);
            }
        }

        if skiplist.len() == 0 {
            break;
        } else {
            col = col
                .iter()
                .enumerate()
                .filter_map(|(i, c)| {
                    if !skiplist.contains(&i) {
                        Some(*c)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
        }
    }

    col
}

fn main() -> Result<()> {
    let s = String::from(read_to_string(Path::new("data/input"))?.trim_end_matches('\n'));

    display(&s);

    // let b = s.into_bytes();

    // let ss = String::from_utf8(reduce(b).to_vec()).expect("Conversion");

    // display(&ss);

    for c in ('a' as u8)..=('z' as u8) {
        let ss = s
            .chars()
            .filter(|x| !(*x as u8).eq_ignore_ascii_case(&c))
            .collect::<String>();

        let len = reduce(ss.into_bytes()).len();

        println!("{} -> {}", c as char, len);
    }

    Ok(())
}
