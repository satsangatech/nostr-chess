use shakmaty::Position;

/// This is the STR (Seven Tag Roster).
///
/// There is a set of tags defined for mandatory use for archival storage of PGN data.
/// The interpretation of these tags is fixed as is the order in which they appear.
/// Although the definition and use of additional tag names and semantics is permitted and encouraged when needed,
/// the STR is the common ground that all programs should follow for public data interchange.
///
///
/// We should define a Rooky Tag Roster (RTR) for the protocol to use.
/// This could include tags for black and white pubkeys
/// and other nostr-specific information.
/// We also include a set of moves for the game.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RookyGame {
    /// The Event tag value should be reasonably descriptive. Abbreviations are to be avoided unless absolutely necessary.
    /// A consistent event naming should be used to help facilitate database scanning. If the name of the event is unknown,
    /// a single question mark should appear as the tag value.
    ///
    /// # Examples:
    ///
    ///
    /// `[Event "FIDE World Championship"]`
    /// `[Event "Moscow City Championship"]`
    /// `[Event "ACM North American Computer Championship"]`
    /// `[Event "Casual Game"]`
    pub event: crate::pgn_standards::PgnEvent,
    /// The Site tag value should include city and region names along with a standard name for the country.
    /// The use of the IOC (International Olympic Committee) three letter names is suggested for those countries where such codes are available.
    /// If the site of the event is unknown, a single question mark should appear as the tag value.
    /// A comma may be used to separate a city from a region.
    /// No comma is needed to separate a city or region from the IOC country code.
    ///
    /// Examples:
    ///
    /// `[Site "New York City, NY USA"]`
    /// `[Site "St. Petersburg RUS"]`
    /// `[Site "Riga LAT"]`
    pub site: crate::pgn_standards::PgnSite,
    /// The Round tag The Round tag value gives the playing round for the game. In a match competition,
    /// this value is the number of the game played. If the use of a round number is inappropriate,
    /// then the field should be a single hyphen character. If the round is unknown,
    /// a single question mark should appear as the tag value.
    ///
    /// Some organizers employ unusual round designations and have multipart playing rounds and sometimes
    /// even have conditional rounds. In these cases, a multipart round identifier can be made from a sequence of
    /// integer round numbers separated by periods. The leftmost integer represents the most significant round and
    /// succeeding integers represent round numbers in descending hierarchical order.
    pub round: crate::pgn_standards::PgnRound,
    pub date: chrono::NaiveDate,
    pub white: String,
    pub black: String,
    pub outcome: shakmaty::Outcome,
    pub moves: Vec<shakmaty::san::SanPlus>,
}
impl TryFrom<nostr_minions::nostro2::NostrNote> for RookyGame {
    type Error = crate::errors::ChessError;

    fn try_from(value: nostr_minions::nostro2::NostrNote) -> Result<Self, Self::Error> {
        Self::try_from(value.content.as_bytes())
    }
}
impl From<RookyGame> for nostr_minions::nostro2::NostrNote {
    fn from(game: RookyGame) -> Self {
        Self {
            content: game.to_pgn(),
            kind: 1,
            created_at: game
                .date
                .and_hms_opt(0, 0, 0)
                .expect("Invalid date")
                .and_utc()
                .timestamp(),
            ..Default::default()
        }
    }
}
impl Default for RookyGame {
    fn default() -> Self {
        Self {
            event: crate::pgn_standards::PgnEvent::Casual,
            site: crate::pgn_standards::PgnSite::Unknown,
            round: crate::pgn_standards::PgnRound::Unknown,
            date: chrono::Local::now().date_naive(),
            white: String::new(),
            black: String::new(),
            outcome: shakmaty::Outcome::Draw,
            moves: Vec::new(),
        }
    }
}

impl RookyGame {
    #[must_use]
    pub fn add_event(mut self, event: String) -> Self {
        self.event = crate::pgn_standards::PgnEvent::Named(event);
        self
    }
    #[must_use]
    pub fn add_site(mut self, site: String) -> Self {
        self.site = crate::pgn_standards::PgnSite::Named(site);
        self
    }
    #[must_use]
    pub fn add_round(mut self, round: String) -> Self {
        self.round = crate::pgn_standards::PgnRound::Named(round);
        self
    }
    #[must_use]
    pub const fn add_date(mut self, date: chrono::NaiveDate) -> Self {
        self.date = date;
        self
    }
    #[must_use]
    pub fn add_white_name(mut self, name: String) -> Self {
        self.white = name;
        self
    }
    #[must_use]
    pub fn add_black_name(mut self, name: String) -> Self {
        self.black = name;
        self
    }
    #[must_use]
    pub const fn add_result(mut self, result: shakmaty::Outcome) -> Self {
        self.outcome = result;
        self
    }
    #[must_use]
    pub fn new_move(mut self, san_plus: shakmaty::san::SanPlus) -> Self {
        self.moves.push(san_plus);
        self
    }
    #[must_use]
    pub fn to_pgn(&self) -> String {
        use std::fmt::Write;
        let mut pgn = String::new();
        writeln!(pgn, "[Event \"{}\"]", self.event).unwrap();
        writeln!(pgn, "[Site \"{}\"]", self.site).unwrap();
        writeln!(pgn, "[Round \"{}\"]", self.round).unwrap();
        writeln!(pgn, "[Date \"{}\"]", self.date.format("%Y.%m.%d")).unwrap();
        writeln!(pgn, "[White \"{}\"]", self.white).unwrap();
        writeln!(pgn, "[Black \"{}\"]", self.black).unwrap();
        writeln!(pgn, "[Result \"{}\"]", self.outcome).unwrap();
        writeln!(pgn).unwrap();
        for (move_num, moves) in self.moves.chunks(2).enumerate() {
            let move_num = move_num + 1;
            let white_move = moves
                .first()
                .map(std::string::ToString::to_string)
                .unwrap_or_default();
            let Some(black_move) = moves.get(1).map(std::string::ToString::to_string) else {
                write!(pgn, "{move_num}. {white_move} ").unwrap();
                break;
            };
            write!(pgn, "{move_num}. {white_move} {black_move} ").unwrap();
        }
        writeln!(pgn, "{}", self.outcome).unwrap();
        pgn
    }
    #[must_use]
    pub fn game_positions(&self) -> Vec<shakmaty::Chess> {
        let mut positions = Vec::new();
        let mut position = shakmaty::Chess::default();
        positions.push(position.clone());
        self.moves.iter().fold(&mut positions, |pos, san_plus| {
            let Ok(new_move) = san_plus.san.to_move(&position) else {
                return pos;
            };
            let Ok(new_pos) = position.clone().play(&new_move) else {
                return pos;
            };
            position = new_pos;
            pos.push(position.clone());
            pos
        });
        positions
    }

    pub fn opening(&self) -> Option<crate::openings::EcoOpening> {
        crate::openings::ECO_OPENINGS.iter().find_map(|opening| {
            let opening_slice = opening.moves.as_slice();
            if self.moves.len() < opening_slice.len() {
                return None;
            }
            let game_slice = &self.moves[..opening_slice.len()];
            if game_slice == opening_slice {
                Some(opening.clone())
            } else {
                None
            }
        })
    }
}
impl TryFrom<&[u8]> for RookyGame {
    type Error = crate::errors::ChessError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = pgn_reader::BufferedReader::new(value);
        let mut game = Self::default();
        reader.read_game(&mut game)?;
        if game.moves.is_empty() {
            return Err(crate::errors::ChessError::NotFound("No moves found"));
        }
        Ok(game)
    }
}
impl TryFrom<web_sys::js_sys::Uint8Array> for RookyGame {
    type Error = crate::errors::ChessError;

    fn try_from(value: web_sys::js_sys::Uint8Array) -> Result<Self, Self::Error> {
        value.to_vec().as_slice().try_into()
    }
}
impl std::str::FromStr for RookyGame {
    type Err = crate::errors::ChessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.as_bytes().try_into()
    }
}
impl std::fmt::Display for RookyGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_pgn())
    }
}

impl pgn_reader::Visitor for RookyGame {
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
    #[allow(clippy::too_many_lines)]
    fn header(&mut self, key: &[u8], value: pgn_reader::RawHeader<'_>) {
        let Ok(key) = crate::headers::RookyHeader::try_from(key) else {
            return;
        };
        match key {
            crate::headers::RookyHeader::Date => {
                let date = value.0.iter().map(|b| *b as char).collect::<String>();
                self.date = chrono::NaiveDate::parse_from_str(&date, "%Y.%m.%d")
                    .ok()
                    .unwrap_or_default();
            }
            crate::headers::RookyHeader::White => {
                if let Ok(white) = std::str::from_utf8(value.0) {
                    self.white = white.to_string();
                }
            }
            crate::headers::RookyHeader::Black => {
                if let Ok(black) = std::str::from_utf8(value.0) {
                    self.black = black.to_string();
                }
            }
            crate::headers::RookyHeader::GameResult => {
                if let Ok(result) = std::str::from_utf8(value.0) {
                    if let Ok(outcome) = result.parse::<crate::pgn_standards::PgnResult>() {
                        match outcome {
                            crate::pgn_standards::PgnResult::Outcome(outcome) => {
                                self.outcome = outcome;
                            }
                            crate::pgn_standards::PgnResult::Unknown => {}
                        }
                    }
                }
            }
            crate::headers::RookyHeader::Event => {
                if let Ok(event) = std::str::from_utf8(value.0) {
                    self.event = crate::pgn_standards::PgnEvent::Named(event.to_string());
                }
            }
            crate::headers::RookyHeader::Site => {
                if let Ok(site) = std::str::from_utf8(value.0) {
                    self.site = crate::pgn_standards::PgnSite::Named(site.to_string());
                }
            }
            crate::headers::RookyHeader::Round => {
                if let Ok(round) = std::str::from_utf8(value.0) {
                    self.round = crate::pgn_standards::PgnRound::Named(round.to_string());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const PNG_EXAMPLE: &str = r#"[Event "FIDE World Championship"]
[Site "New York, NY USA"]
[Round "5.2"]
[Date "2023.10.15"]
[White "Carlsen, Magnus"]
[Black "Nakamura, Hikaru"]
[Result "1-0"]

1. e4 e5 2. Nf3 Nc6 3. Bb5 Nf6 4. O-O Be7 5. Re1 O-O 6. d3 b5 7. Bb3 d6 8. c3 Na5 9. Bc2 c5 10. Nbd2 Nc6 11. Nf1 Re8 12. Ng3 Bf8 13. h3 h6 14. d4 exd4 15. cxd4 cxd4 16. Nxd4 Nxd4 17. Qxd4 Be6 18. Bg5 Qb6 19. Qxb6 axb6 20. Bxf6 gxf6 21. Rad1 Bg7 22. Rxd6 Bxb2 23. Rd2 Bg7 24. Ne2 Be5 25. f4 Bc7 26. e5 fxe5 27. fxe5 Bxe5 28. Nf4 Bf6 29. Nd5 Bxd5 30. Rxd5 Re6 31. Rxb5 Rc8 32. Bb3 Rc1+ 33. Kh2 Rc2 34. Rxb6 Rxa2 35. Rb8+ Kh7 36. Bxf7 Ra1 37. Be8 Rg6 38. Bxg6+ fxg6 39. Rxg6 1-0
"#;

    #[test]
    fn test_read_pgn_game() {
        let game = PNG_EXAMPLE.parse::<RookyGame>().unwrap();
        assert_eq!(game.moves.chunks(2).len(), 39);
        assert_eq!(
            game.outcome,
            shakmaty::Outcome::Decisive {
                winner: shakmaty::Color::White
            }
        );
        assert_eq!(game.white, "Carlsen, Magnus".to_string());
        assert_eq!(game.black, "Nakamura, Hikaru".to_string());
        assert_eq!(
            game.event,
            crate::pgn_standards::PgnEvent::Named("FIDE World Championship".to_string())
        );
        assert_eq!(
            game.site,
            crate::pgn_standards::PgnSite::Named("New York, NY USA".to_string())
        );
        assert_eq!(
            game.round,
            crate::pgn_standards::PgnRound::Named("5.2".to_string())
        );
        assert_eq!(
            game.date,
            chrono::NaiveDate::from_ymd_opt(2023, 10, 15).unwrap()
        );
    }
    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_build_pgn_game() {
        let new_game = RookyGame::default()
            .add_date(chrono::NaiveDate::from_ymd_opt(2023, 10, 15).unwrap())
            .add_site("New York, NY USA".to_string())
            .add_white_name("Carlsen, Magnus".to_string())
            .add_black_name("Nakamura, Hikaru".to_string())
            .add_event("FIDE World Championship".to_string())
            .add_round("5.2".to_string())
            .add_result(shakmaty::Outcome::Decisive {
                winner: shakmaty::Color::White,
            })
            .new_move(shakmaty::san::SanPlus::from_ascii(b"e4").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"e5").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Nf3").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Nc6").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Bb5").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Nf6").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"O-O").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Be7").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Re1").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"O-O").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"d3").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"b5").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Bb3").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"d6").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"c3").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Na5").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Bc2").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"c5").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Nbd2").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Nc6").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Nf1").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Re8").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Ng3").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Bf8").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"h3").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"h6").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"d4").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"exd4").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"cxd4").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"cxd4").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Nxd4").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Nxd4").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Qxd4").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Be6").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Bg5").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Qb6").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Qxb6").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"axb6").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Bxf6").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"gxf6").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Rad1").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Bg7").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Rxd6").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Bxb2").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Rd2").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Bg7").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Ne2").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Be5").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"f4").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Bc7").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"e5").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"fxe5").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"fxe5").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Bxe5").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Nf4").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Bf6").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Nd5").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Bxd5").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Rxd5").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Re6").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Rxb5").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Rc8").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Bb3").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Rc1+").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Kh2").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Rc2").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Rxb6").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Rxa2").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Rb8+").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Kh7").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Bxf7").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Ra1").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Be8").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Rg6").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Bxg6+").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"fxg6").unwrap())
            .new_move(shakmaty::san::SanPlus::from_ascii(b"Rxg6").unwrap());

        assert_eq!(new_game.moves.chunks(2).len(), 39);
        let example_game = PNG_EXAMPLE.parse::<RookyGame>().unwrap();
        assert_eq!(new_game.moves, example_game.moves);
        assert_eq!(new_game.white, example_game.white);
        assert_eq!(new_game.black, example_game.black);
        assert_eq!(new_game.event, example_game.event);
        assert_eq!(new_game.outcome, example_game.outcome);
        assert_eq!(new_game.site, example_game.site);
        assert_eq!(new_game.round, example_game.round);
        assert_eq!(new_game.to_pgn(), PNG_EXAMPLE);
    }
}
