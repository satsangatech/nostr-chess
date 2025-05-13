#[derive(Debug)]
pub enum RookyHeader {
    /// Headers to match the PGN STR
    Event,
    Site,
    Date,
    Round,
    White,
    Black,
    GameResult,
    // TODO 
    // Defin RTR headers
}
impl TryFrom<&[u8]> for RookyHeader {
    type Error = &'static str;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value {
            b"Event" => Ok(Self::Event),
            b"Site" => Ok(Self::Site),
            b"Date" => Ok(Self::Date),
            b"Round" => Ok(Self::Round),
            b"White" => Ok(Self::White),
            b"Black" => Ok(Self::Black),
            b"Result" => Ok(Self::GameResult),
            _ => Err("Invalid header"),
        }
    }
}
