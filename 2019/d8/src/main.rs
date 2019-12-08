use std::fs;

fn print_layers(image: &Vec<u8>, width: usize, height: usize) {
    let mut i: usize = 0;
    let mut layer = 0;
    let mut zero_count;
    let mut layers: Vec<Vec<u8>> = Vec::new();

    loop {
        if i >= image.len() {
            break;
        }

        println!("Layer {}", layer);
        let mut l = Vec::new();
        zero_count = 0;
        for _ in 0..height {
            for _ in 0..width {
                print!("{} ", image[i]);
                l.push(image[i]);
                if image[i] == 0 {
                    zero_count += 1;
                }

                i += 1;
            }
            print!("\n");
        }
        println!("Layer {} | Zero count {}", layer, zero_count);
        layers.push(l);
        layer += 1;
    }

    layers.reverse();
    i = 0;
    for _ in 0..height {
        for _ in 0..width {
            let mut color = ' ';
            for ii in 0..layers.len() {
                match layers[ii][i] {
                    0 => color = ' ',
                    1 => color = '#',
                    _ => (),
                }
            }
            print!("{}", color);
            i += 1;
        }
        print!("\n");
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let width = 25;
    let height = 6;

    print_layers(
        &file
            .trim()
            .as_bytes()
            .iter()
            .map(|c| *c - '0' as u8)
            .collect(),
        width,
        height,
    );
}
