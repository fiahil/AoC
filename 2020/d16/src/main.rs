mod input;

use std::{collections::HashMap, fmt, fs::File, ops::RangeInclusive, str::FromStr};

use anyhow::{Context, Error, Result};

#[derive(Clone, PartialEq, Eq, Hash)]
struct Field {
    caption: String,
    r1: RangeInclusive<u32>,
    r2: RangeInclusive<u32>,
}

impl Field {
    fn from(s: &str) -> Result<Vec<Self>, Error> {
        s.lines()
            .map(|l| {
                let mut l = l.split(": ");

                let caption = l.next().context("Missing caption")?.to_string();
                let mut rs = l.next().context("Missing ranges")?.split(" or ");
                let r1 = rs.next().context("Missing r1").and_then(|e| {
                    let ops: Vec<u32> =
                        e.split("-").map(|e| e.parse()).collect::<Result<_, _>>()?;
                    Ok(ops[0]..=ops[1])
                })?;
                let r2 = rs.next().context("Missing r2").and_then(|e| {
                    let ops: Vec<u32> =
                        e.split("-").map(|e| e.parse()).collect::<Result<_, _>>()?;
                    Ok(ops[0]..=ops[1])
                })?;

                Ok(Field { caption, r1, r2 })
            })
            .collect()
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.caption)
    }
}

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.caption)
    }
}

#[derive(Debug, Clone)]
struct Ticket {
    raw_fields: Vec<u32>,
}

impl FromStr for Ticket {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ticket {
            raw_fields: s
                .split(",")
                .map(|ee| ee.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug)]
struct Note {
    fields: Vec<Field>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl Note {
    fn scanning_error_rate(&self) -> u32 {
        self.nearby_tickets
            .iter()
            .flat_map(|t| {
                t.raw_fields
                    .iter()
                    .filter(|val| {
                        self.fields
                            .iter()
                            .inspect(|f| {
                                println!("# {} {:?} || {:?} <- {}", f.caption, f.r1, f.r2, val)
                            })
                            .all(|field| !(field.r1.contains(val) || field.r2.contains(val)))
                    })
                    .inspect(|e| println!("# INVALID {}", e))
            })
            .fold(0, |acc, e| acc + e)
    }

    fn match_fields(&self) {
        let tickets = self
            .nearby_tickets
            .iter()
            .filter(|t| {
                t.raw_fields
                    .iter()
                    .filter(|val| {
                        self.fields
                            .iter()
                            .any(|field| field.r1.contains(val) || field.r2.contains(val))
                    })
                    .count()
                    == t.raw_fields.len()
            })
            .inspect(|t| println!("> {:?}", t))
            .collect::<Vec<&Ticket>>();

        let mut constraints: HashMap<&Field, Vec<usize>> = HashMap::new();

        // Establish a list of constraints for each field
        for field in self.fields.iter() {
            for ticket in tickets.iter() {
                for (i, raw) in ticket.raw_fields.iter().enumerate() {
                    if field.r1.contains(raw) || field.r2.contains(raw) {
                        constraints.entry(field).or_default();
                    } else {
                        constraints.entry(field).or_default().push(i);
                    }
                }
            }
        }

        // println!("> Constraints: {:#?}", constraints);

        let mut constraints: Vec<_> = constraints.into_iter().collect();
        constraints.sort_by(|a, b| a.1.len().cmp(&b.1.len()).reverse());

        // println!("> Constraints: {:#?}", constraints);

        let ordered_fields = constraints
            .iter()
            .map(|(f, v)| {
                let indices = (0..self.fields.len())
                    .into_iter()
                    .filter(|i| !v.contains(i))
                    .inspect(|e| println!("> {} = {:?}", f, e))
                    .collect::<Vec<usize>>();

                (f, indices)
            })
            .collect::<Vec<_>>();

        println!("> ordered fields: {:#?}", ordered_fields);

        fn find_solution(
            result: Vec<Option<Field>>,
            source: Vec<(&&Field, Vec<usize>)>,
            i: usize,
        ) -> Option<Vec<Field>> {
            if i >= source.len() {
                println!("> i {}/{} result: {:?}", i, source.len(), result);
                if result.iter().all(|e| e.is_some()) {
                    return Some(result.iter().flatten().cloned().collect());
                } else {
                    return None;
                }
            }

            let (f, indices) = source[i].clone();
            let indices = indices
                .iter()
                .filter(|e| result[**e].is_none())
                .collect::<Vec<_>>();

            println!("> {} : {:?}", f, indices);
            println!(
                "> available {:?}",
                result
                    .iter()
                    .enumerate()
                    .filter_map(|(i, e)| if e.is_none() { Some(i) } else { None })
                    .collect::<Vec<_>>()
            );

            for indice in indices {
                let mut new_result = result.clone();

                new_result[*indice].replace(f.clone().clone());
                println!("> i {} indices {:?} result {:?}", i, indice, new_result);
                let solution = find_solution(new_result, source.clone(), i + 1);
                println!("> i {} solution {:?}", i, solution);

                if solution.is_some() {
                    return solution;
                }
            }

            None
        }

        let mut result = Vec::new();
        (0..self.fields.len())
            .into_iter()
            .for_each(|_| result.push(None));

        let solution = find_solution(result.clone(), ordered_fields.clone(), 0);

        println!("> solution: {:?}", solution);

        let r = tickets.iter().all(|ticket| {
            ticket
                .raw_fields
                .iter()
                .zip(solution.clone().unwrap().iter())
                // .inspect(|(v, f)| println!("{} {} -> {:?} || {:?}", f.caption, v, f.r1, f.r2))
                .all(|(v, f)| f.r1.contains(v) || f.r2.contains(v))
        });

        println!("> r {}", r);

        let product = self
            .your_ticket
            .raw_fields
            .iter()
            .zip(solution.clone().unwrap().iter())
            .inspect(|(v, f)| println!(">>> {}: {}", f.caption, v))
            .filter_map(|(v, f)| {
                if f.caption.starts_with("departure") {
                    Some(v)
                } else {
                    None
                }
            })
            .fold(1u64, |acc, v| acc * (*v as u64));

        println!("> {}", product);
    }
}

fn transform(p: String) -> Result<Note> {
    let mut sections = p.split("\n\n").inspect(|e| println!("> {}", e));

    Ok(Note {
        fields: Field::from(sections.next().context("Missing fields")?)?,
        your_ticket: Ticket::from_str(
            sections
                .next()
                .context("Missing your ticket")?
                .lines()
                .skip(1)
                .next()
                .unwrap(),
        )?,
        nearby_tickets: sections
            .next()
            .context("Missing nearby tickets")?
            .lines()
            .skip(1)
            .map(|e| Ticket::from_str(e))
            .collect::<Result<_, _>>()?,
    })
}

fn main() -> Result<()> {
    let note = input::input(File::open("data/input.txt")?, transform)?;

    println!("{:#?}", note);

    let rate = note.scanning_error_rate();

    println!("> Rate {}", rate);

    note.match_fields();

    Ok(())
}
