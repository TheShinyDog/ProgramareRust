fn main() {
    for i in (1..100).rev() {
        let i_1 = i - 1;
        if i != 1 {
            println!(
                "{i} bottles of beer on the wall,\n{i} bottles of beer.\nTake one down, pass it around,\n{i_1} bottles of beer on the wall.\n"
            );
        }
        if i == 1 {
            println!(
                "{i} bottle of beer on the wall,\n{i} bottles of beer.\nTake one down, pass it around,\nNo bottles of beer on the wall.\n"
            );
        }
    }
}
