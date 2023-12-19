use std::collections::HashMap;

#[derive(Debug)]
pub struct Rule {
    id: String,
    transitions: Vec<Transition>,
}

#[derive(Debug)]
pub enum Transition {
    Gt {
        field: String,
        value: u32,
        next: String,
    },
    Lt {
        field: String,
        value: u32,
        next: String,
    },
    Default {
        next: String,
    },
}

#[derive(Debug)]
pub struct Part {
    fields: HashMap<String, u32>,
}

impl From<&str> for Part {
    fn from(s: &str) -> Self {
        let fields = s
            .trim()
            .strip_prefix("{")
            .and_then(|s| s.strip_suffix("}"))
            .and_then(|s| {
                s.split(",")
                    .map(|s| s.split("="))
                    .map(|mut s| {
                        let key = s.next().unwrap().to_string();
                        let value = s.next().unwrap().parse::<u32>().unwrap();
                        (key, value)
                    })
                    .collect::<HashMap<String, u32>>()
                    .into()
            })
            .unwrap_or_default();
        Self { fields }
    }
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        let id = s
            .trim()
            .chars()
            .take_while(|c| c.is_alphabetic())
            .collect::<String>();

        let transitions = s
            .trim()
            .chars()
            .skip_while(|c| c != &'{')
            .skip(1)
            .take_while(|c| c != &'}')
            .collect::<String>()
            .split(",")
            .map(|s| {
                if s.contains(">") {
                    let mut s = s.split(|c| c == '>' || c == ':');
                    let field = s.next().unwrap().to_string();
                    let value = s.next().unwrap().parse::<u32>().unwrap();
                    let next = s.next().unwrap().to_string();
                    Transition::Gt { field, value, next }
                } else if s.contains("<") {
                    let mut s = s.split(|c| c == '<' || c == ':');
                    let field = s.next().unwrap().to_string();
                    let value = s.next().unwrap().parse::<u32>().unwrap();
                    let next = s.next().unwrap().to_string();
                    Transition::Lt { field, value, next }
                } else {
                    let next = s.to_string();
                    Transition::Default { next }
                }
            })
            .collect::<Vec<Transition>>();

        Self { id, transitions }
    }
}

pub fn part1(input: &String) -> u32 {
    let (rules, parts) = input.split_once("\n\n").unwrap();
    let rules = rules.trim().lines().map(Rule::from).collect::<Vec<Rule>>();
    let parts = parts.trim().lines().map(Part::from).collect::<Vec<Part>>();

    let mut accepted = Vec::new();
    let mut rejected = Vec::new();

    for part in parts {
        let mut next = rules
            .iter()
            .find(|r| r.id == "in")
            .expect("No first rule found");
        let mut i = 0;
        'outer: loop {
            println!("({:?}) / {}: {:?}", part, i, next);
            for transition in &next.transitions {
                let next_label = match transition {
                    Transition::Gt { field, value, next } => {
                        if part.fields.get(field).unwrap() > &value {
                            next
                        } else {
                            continue;
                        }
                    }
                    Transition::Lt { field, value, next } => {
                        if part.fields.get(field).unwrap() < &value {
                            next
                        } else {
                            continue;
                        }
                    }
                    Transition::Default { next } => next,
                };

                println!(" -> {}", next_label);

                match next_label.as_str() {
                    "A" => {
                        accepted.push(part);
                        break 'outer;
                    }
                    "R" => {
                        rejected.push(part);
                        break 'outer;
                    }
                    _ => {
                        next = rules
                            .iter()
                            .find(|r| &r.id == next_label)
                            .expect("No rule found");
                        break;
                    }
                }
            }
            i += 1;
        }
    }

    accepted
        .iter()
        .flat_map(|p| vec![p.fields["x"], p.fields["m"], p.fields["a"], p.fields["s"]])
        .sum()
}

pub fn part2(input: &String) -> u64 {
    0
}

pub mod test {
    #[test]
    pub fn part1() {
        let r = super::part1(
            &"
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"
                .to_string(),
        );
        assert_eq!(r, 19114);
    }

    #[test]
    pub fn part2() {
        let r = super::part2(&"".to_string());
        assert_eq!(r, 167_409_079_868_000);
    }
}
