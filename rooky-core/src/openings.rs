pub static ECO_OPENINGS: std::sync::LazyLock<Vec<EcoOpening>> = std::sync::LazyLock::new(|| {
    const OPENING_TABLES: [&str; 5] = [
        include_str!("../../openings/a.tsv"),
        include_str!("../../openings/b.tsv"),
        include_str!("../../openings/c.tsv"),
        include_str!("../../openings/d.tsv"),
        include_str!("../../openings/e.tsv"),
    ];
    OPENING_TABLES
        .iter()
        .flat_map(|table| table.lines().skip(1)) // Skip header for each table
        .filter_map(|line| {
            let mut parts = line.split('\t');
            let mut eco = EcoOpening {
                code: parts.next()?.to_string(),
                ..Default::default()
            };
            let name_part = parts.next()?;
            match name_part.split_once(':') {
                Some((name, variation)) => {
                    eco.name = name.trim().to_string();
                    eco.variation = variation.trim().to_string();
                }
                None => {
                    eco.name = name_part.to_string();
                }
            }
            let game_part = parts.next()?;
            let mut reader = pgn_reader::BufferedReader::new_cursor(game_part.as_bytes());
            reader.read_game(&mut eco).unwrap()?;
            Some(eco)
        })
        .collect()
});
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct EcoOpening {
    pub code: String,
    pub name: String,
    pub variation: String,
    pub moves: Vec<shakmaty::san::SanPlus>,
}
impl pgn_reader::Visitor for EcoOpening {
    type Result = Vec<shakmaty::san::SanPlus>;

    fn begin_game(&mut self) {
        self.moves = Vec::new();
    }
    fn san(&mut self, san_plus: shakmaty::san::SanPlus) {
        self.moves.push(san_plus);
    }
    fn end_game(&mut self) -> Self::Result {
        self.moves.clone()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_eco_openings() {
        let openings = &super::ECO_OPENINGS;
        assert_eq!(openings.len(), 3516);
        let opening = openings.first().unwrap();
        assert_eq!(opening.code, "A00");
        assert_eq!(opening.name, "Amar Opening");
    }
    #[test]
    fn test_find_opening() {
        const TEST_OPENING: &str = "1. e4 g5 2. d4 h6 3. h4 g4";
        let mut test_eco = super::EcoOpening::default();
        let mut reader = pgn_reader::BufferedReader::new_cursor(TEST_OPENING.as_bytes());
        reader.read_game(&mut test_eco).unwrap();
        println!("{test_eco:?}");
        let opening = super::ECO_OPENINGS
            .iter()
            .find(|opening| opening.moves == test_eco.moves);
        println!("{:?}", test_eco.moves);
        println!("{opening:?}");
        assert_eq!(opening.unwrap().code, "B00");
        assert_eq!(opening.unwrap().name, "Borg Defense");
        assert_eq!(opening.unwrap().variation, "Troon Gambit");
    }
}
