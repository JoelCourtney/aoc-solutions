#[macro_export]
macro_rules! matches {
    ($opt:ident; $($path:path,)*) => {
        match ($opt.year, $opt.day) {
            $((<$path>::YEAR, <$path>::DAY) => {
                let file_name = format!("inputs/{}-{}", $opt.year, $opt.day);
                let text = std::fs::read_to_string(file_name)?;
                let input = <$path>::parse(text)?;

                if $opt.part == 1 {
                    println!("y{}, d{}, p{} => {:?}",
                        <$path>::YEAR,
                        <$path>::DAY,
                        $opt.part,
                        <$path>::default().part1(input)?
                    );
                } else {
                    println!("y{}, d{}, p{} => {:?}",
                        <$path>::YEAR,
                        <$path>::DAY,
                        $opt.part,
                        <$path>::default().part2(input)?
                    );
                }
            })*
            _ => {
                anyhow::bail!("Puzzle not found: {}, {}", $opt.year, $opt.day);
            }
        }
    }
}