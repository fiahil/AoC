use std::collections::BTreeMap;

fn hash(input: &str) -> u32 {
    let mut current = 0;
    for c in input.chars() {
        current += c as u32;
        current *= 17;
        current %= 256;
    }
    current
}

pub fn part1(input: &String) -> u32 {
    let mut sum = 0;
    for step in input.trim().split(',') {
        dbg!(&step);
        sum += dbg!(hash(step));
    }

    sum
}

#[derive(Debug)]
enum LensOp {
    Unset,
    Add(String),
    Del(String),
}

pub fn part2(input: &String) -> u32 {
    let mut map = BTreeMap::new();

    for step in input.trim().split(',') {
        dbg!(&step);
        let (lensop, buf) = step.chars().fold(
            (LensOp::Unset, String::new()),
            |(acc, mut buf), c| match c {
                '-' => (LensOp::Del(buf), String::new()),
                '=' => (LensOp::Add(buf), String::new()),
                _ => {
                    buf += &c.to_string();
                    (acc, buf)
                }
            },
        );

        let focal = buf.parse().unwrap_or(0);

        match lensop {
            LensOp::Add(key) => {
                let hash = hash(&key);
                map.entry(hash)
                    .and_modify(|e: &mut Vec<(String, i32)>| {
                        if e.iter().find(|(k, _)| k == &key).is_none() {
                            e.push((key.clone(), focal))
                        } else {
                            e.iter_mut().find(|(k, _)| k == &key).unwrap().1 = focal;
                        }
                    })
                    .or_insert(vec![(key.clone(), focal)]);
            }
            LensOp::Del(key) => {
                let hash = hash(&key);
                map.entry(hash).and_modify(|e: &mut Vec<(String, i32)>| {
                    if let Some(index) =
                        e.iter()
                            .enumerate()
                            .find_map(|(idx, (k, _))| if k == &key { Some(idx) } else { None })
                    {
                        e.remove(index);
                    }
                });
            }
            _ => panic!("Unset"),
        }

        // dbg!(&map);
    }

    map.into_iter()
        .map(|(b, v)| {
            v.into_iter()
                .enumerate()
                .map(|(i, e)| (b + 1) * (i as u32 + 1) * e.1 as u32)
                .sum::<u32>()
        })
        .sum()
}

pub mod test {
    #[test]
    pub fn test_hash() {
        let r = super::hash("HASH");
        assert_eq!(r, 52);
    }

    #[test]
    pub fn part1() {
        let r = super::part1(&"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string());
        assert_eq!(r, 1320);
    }

    #[test]
    pub fn part2() {
        let r = super::part2(&"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string());
        assert_eq!(r, 145);
    }
}
