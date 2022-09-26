fn main() {
    let content = include_str!("../input.txt");
    let mut wrapping_paper = 0x0;
    let mut ribbon = 0x0;

    for x in content.lines() {
        if x.len() == 0x0 {
            break;
        }

        let dim: Vec<u32> = x.split("x")
                                    .map(|x| x.parse::<u32>().unwrap())
                                    .collect();

        let (l, w, h) = (dim[0], dim[1], dim[2]);
        let surface = [l*w, w*h, h*l];

        let volume = l*w*h;
        let perimeters = [2 * (l + w), 2 * (w + h), 2 * (h + l)];

        ribbon += volume + perimeters.iter().min().unwrap();
        wrapping_paper += 2*surface.iter().sum::<u32>() + surface.iter().min().unwrap();
    }

    println!("wrapping paper: {wrapping_paper}");
    println!("ribbon: {ribbon}");
}
