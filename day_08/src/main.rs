fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Couldn't read file");
    let width = 25;
    let height = 6;
    let layerlen = width*height;

    // let contents = "123456789012".to_string();
    // let width = 3;
    // let height = 2;

    let layers = contents.chars()
        .map(|c| c.to_string().parse::<i32>())
        .filter(|o| o.is_ok()).map(|o| o.unwrap())
        .collect::<Vec<i32>>()
        .chunks(layerlen)
        .map(|chnk| chnk.to_vec())
        .collect::<Vec<Vec<i32>>>();

    let mut counts = layers.iter().map(|layer| (
        layer.iter().filter(|v| **v == 0).count(),
        layer.iter().filter(|v| **v == 1).count(),
        layer.iter().filter(|v| **v == 2).count()
    )).collect::<Vec<(usize, usize, usize)>>();

    counts.sort_by(|a, b| a.cmp(&b));

    let part1 = counts[0].1 * counts[0].2;
    println!("part1 = {}", part1);
 
    // Part 2
    for xy in 0..layerlen {
        if xy % width == 0 {
            println!("");
        }

        // Find pixel value
        let mut v = 2;
        for l in 0..layers.len() {
            if layers[l][xy] != 2 {
                v = layers[l][xy];
                break;
            }
        }

        print!("{}", if v == 0 { ' ' } else { '#' });
    }
}
