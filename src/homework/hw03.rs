fn envelope() {
    const W: u32 = 25;
    const H: u32 = 10;

    for y in 0 .. H {
        for x in 0 .. W {
            if y == 0 || y == H - 1 || x == 0 || x == W - 1 || x == y * (W - 1) / (H - 1) || x == (H - 1 - y) * (W - 1) / (H - 1) {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}