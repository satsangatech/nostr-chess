#[derive(Debug, PartialEq, Clone, Eq, Default)]
pub enum PgnEvent {
    #[default]
    Casual,
    Unknown,
    Named(String),
}
impl std::str::FromStr for PgnEvent {
    type Err = crate::errors::ChessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() || s == "?" {
            return Ok(Self::Unknown);
        }
        if s.contains("Casual") {
            return Ok(Self::Casual);
        }
        Ok(Self::Named(s.to_string()))
    }
}
impl std::fmt::Display for PgnEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Casual => write!(f, "Casual Game"),
            Self::Unknown => write!(f, "?"),
            Self::Named(name) => write!(f, "{name}"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Default)]
pub enum PgnSite {
    #[default]
    Unknown,
    Named(String),
}
impl std::str::FromStr for PgnSite {
    type Err = crate::errors::ChessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() || s == "?" {
            return Ok(Self::Unknown);
        }
        Ok(Self::Named(s.to_string()))
    }
}
impl std::fmt::Display for PgnSite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "?"),
            Self::Named(name) => write!(f, "{name}"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Default)]
pub enum PgnRound {
    #[default]
    Unknown,
    Named(String),
}
impl std::str::FromStr for PgnRound {
    type Err = crate::errors::ChessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() || s == "-" {
            return Ok(Self::Unknown);
        }
        Ok(Self::Named(s.to_string()))
    }
}
impl std::fmt::Display for PgnRound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "-"),
            Self::Named(name) => write!(f, "{name}"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Default)]
pub enum PgnResult {
    #[default]
    Unknown,
    Outcome(shakmaty::Outcome),
}
impl std::str::FromStr for PgnResult {
    type Err = crate::errors::ChessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1-0" => Ok(Self::Outcome(shakmaty::Outcome::Decisive {
                winner: shakmaty::Color::White,
            })),
            "0-1" => Ok(Self::Outcome(shakmaty::Outcome::Decisive {
                winner: shakmaty::Color::Black,
            })),
            "1/2-1/2" => Ok(Self::Outcome(shakmaty::Outcome::Draw)),
            "*" => Ok(Self::Unknown),
            _ => Err(crate::errors::ChessError::InvalidPgn(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid result",
            ))),
        }
    }
}
impl std::fmt::Display for PgnResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "*"),
            Self::Outcome(outcome) => match outcome {
                shakmaty::Outcome::Decisive { winner } => {
                    if winner == &shakmaty::Color::White {
                        write!(f, "1-0")
                    } else {
                        write!(f, "0-1")
                    }
                }
                shakmaty::Outcome::Draw => write!(f, "1/2-1/2"),
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Default)]
pub enum PgnTermination {
    Abandoned,
    Adjudication,
    Death,
    Emergency,
    #[default]
    Normal,
    RulesInfraction,
    TimeForfeit,
    Unterminated,
}
impl std::str::FromStr for PgnTermination {
    type Err = crate::errors::ChessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Abandoned" => Ok(Self::Abandoned),
            "Adjudication" => Ok(Self::Adjudication),
            "Death" => Ok(Self::Death),
            "Emergency" => Ok(Self::Emergency),
            "Normal" => Ok(Self::Normal),
            "Rules infraction" => Ok(Self::RulesInfraction),
            "Time forfeit" => Ok(Self::TimeForfeit),
            "Unterminated" => Ok(Self::Unterminated),
            _ => Err(crate::errors::ChessError::InvalidPgn(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid termination reason",
            ))),
        }
    }
}
impl std::fmt::Display for PgnTermination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Abandoned => write!(f, "Abandoned"),
            Self::Adjudication => write!(f, "Adjudication"),
            Self::Death => write!(f, "Death"),
            Self::Emergency => write!(f, "Emergency"),
            Self::Normal => write!(f, "Normal"),
            Self::RulesInfraction => write!(f, "Rules infraction"),
            Self::TimeForfeit => write!(f, "Time forfeit"),
            Self::Unterminated => write!(f, "Unterminated"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum TimeControl {
    Unknown,
    NoTimeControl,
    MovesInTime { moves: u32, seconds: u32 },
    SuddenDeath { seconds: u32 },
    Incremental { base: u32, increment: u32 },
    Sandclock { seconds: u32 },
    Multiple(Vec<TimeControl>),
}
#[derive(Debug, PartialEq, Eq)]
pub enum TimeControlParseError {
    EmptyString,
    InvalidFormat(String),
    InvalidNumber(String),
    UnexpectedField(String),
}
impl TryFrom<&[u8]> for TimeControl {
    type Error = TimeControlParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(value)
            .map_err(|_| TimeControlParseError::InvalidFormat("Invalid UTF-8".to_string()))?;
        s.parse()
    }
}
impl std::str::FromStr for TimeControl {
    type Err = TimeControlParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(TimeControlParseError::EmptyString);
        }

        // Handle single-character cases first
        match s {
            "?" => return Ok(Self::Unknown),
            "-" => return Ok(Self::NoTimeControl),
            _ => (),
        }

        // Check for multiple fields separated by colons
        if s.contains(':') {
            let fields: Vec<&str> = s.split(':').collect();
            let mut controls = Vec::new();

            for field in fields {
                controls.push(field.parse()?);
            }

            return Ok(Self::Multiple(controls));
        }

        // Parse individual time control fields
        if s.contains('/') {
            // Moves in time format: "moves/seconds"
            let parts: Vec<&str> = s.split('/').collect();
            if parts.len() != 2 {
                return Err(TimeControlParseError::InvalidFormat(s.to_string()));
            }

            let moves = parts[0]
                .parse()
                .map_err(|_| TimeControlParseError::InvalidNumber(parts[0].to_string()))?;
            let seconds = parts[1]
                .parse()
                .map_err(|_| TimeControlParseError::InvalidNumber(parts[1].to_string()))?;

            return Ok(Self::MovesInTime { moves, seconds });
        } else if s.contains('+') {
            // Incremental format: "base+increment"
            let parts: Vec<&str> = s.split('+').collect();
            if parts.len() != 2 {
                return Err(TimeControlParseError::InvalidFormat(s.to_string()));
            }

            let base = parts[0]
                .parse()
                .map_err(|_| TimeControlParseError::InvalidNumber(parts[0].to_string()))?;
            let increment = parts[1]
                .parse()
                .map_err(|_| TimeControlParseError::InvalidNumber(parts[1].to_string()))?;

            return Ok(Self::Incremental { base, increment });
        } else if let Some(stripped) = s.strip_prefix("*") {
            // Sandclock format: "*seconds"
            let seconds = stripped
                .parse()
                .map_err(|_| TimeControlParseError::InvalidNumber(stripped.to_string()))?;

            return Ok(Self::Sandclock { seconds });
        } else
        // Try to parse as sudden death (simple number)
        if let Ok(seconds) = s.parse() {
            return Ok(Self::SuddenDeath { seconds });
        }

        Err(TimeControlParseError::InvalidFormat(s.to_string()))
    }
}

impl std::fmt::Display for TimeControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "?"),
            Self::NoTimeControl => write!(f, "-"),
            Self::MovesInTime { moves, seconds } => write!(f, "{moves}/{seconds}"),
            Self::SuddenDeath { seconds } => write!(f, "{seconds}"),
            Self::Incremental { base, increment } => write!(f, "{base}+{increment}"),
            Self::Sandclock { seconds } => write!(f, "*{seconds}"),
            Self::Multiple(controls) => {
                let parts: Vec<String> = controls
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect();
                write!(f, "{}", parts.join(":"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_unknown_time() {
        assert_eq!("?".parse(), Ok(TimeControl::Unknown));
    }

    #[test]
    fn test_parse_no_time_control() {
        assert_eq!("-".parse(), Ok(TimeControl::NoTimeControl));
    }

    #[test]
    fn test_parse_moves_in_time() {
        assert_eq!(
            "40/9000".parse(),
            Ok(TimeControl::MovesInTime {
                moves: 40,
                seconds: 9000
            })
        );
    }

    #[test]
    fn test_parse_sudden_death_time() {
        assert_eq!("300".parse(), Ok(TimeControl::SuddenDeath { seconds: 300 }));
    }

    #[test]
    fn test_parse_incremental_time() {
        assert_eq!(
            "4500+60".parse(),
            Ok(TimeControl::Incremental {
                base: 4500,
                increment: 60
            })
        );
    }

    #[test]
    fn test_parse_sandclock_time() {
        assert_eq!("*180".parse(), Ok(TimeControl::Sandclock { seconds: 180 }));
    }

    #[test]
    fn test_parse_multiple_time() {
        assert_eq!(
            "40/9000:300".parse(),
            Ok(TimeControl::Multiple(vec![
                TimeControl::MovesInTime {
                    moves: 40,
                    seconds: 9000
                },
                TimeControl::SuddenDeath { seconds: 300 }
            ]))
        );
    }

    #[test]
    fn test_display_time() {
        assert_eq!(TimeControl::Unknown.to_string(), "?");
        assert_eq!(TimeControl::NoTimeControl.to_string(), "-");
        assert_eq!(
            TimeControl::MovesInTime {
                moves: 40,
                seconds: 9000
            }
            .to_string(),
            "40/9000"
        );
        assert_eq!(
            TimeControl::Multiple(vec![
                TimeControl::MovesInTime {
                    moves: 40,
                    seconds: 9000
                },
                TimeControl::SuddenDeath { seconds: 300 }
            ])
            .to_string(),
            "40/9000:300"
        );
    }
    #[test]
    fn test_from_str() {
        assert_eq!(
            "USA".parse::<OlympicCountryCode>().unwrap(),
            OlympicCountryCode::UnitedStatesOfAmerica
        );
        assert_eq!(
            "ENG".parse::<OlympicCountryCode>().unwrap(),
            OlympicCountryCode::England
        );
        assert_eq!(
            "RUS".parse::<OlympicCountryCode>().unwrap(),
            OlympicCountryCode::Russia
        );
        assert_eq!(
            "JAP".parse::<OlympicCountryCode>().unwrap(),
            OlympicCountryCode::Japan
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(OlympicCountryCode::UnitedStatesOfAmerica.to_string(), "USA");
        assert_eq!(OlympicCountryCode::England.to_string(), "ENG");
        assert_eq!(OlympicCountryCode::Russia.to_string(), "RUS");
        assert_eq!(OlympicCountryCode::Japan.to_string(), "JAP");
    }

    #[test]
    fn test_unknown_code() {
        assert!("XYZ".parse::<OlympicCountryCode>().is_err());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OlympicCountryCode {
    Afghanistan,
    AboardAircraft,
    Albania,
    Algeria,
    Andorra,
    Angola,
    Antigua,
    Argentina,
    Armenia,
    Antarctica,
    Australia,
    Azerbaijan,
    Bangladesh,
    Bahrain,
    Bahamas,
    Belgium,
    Bermuda,
    BosniaHerzegovina,
    Belarus,
    Bulgaria,
    Belize,
    Bolivia,
    Barbados,
    Brazil,
    Brunei,
    Botswana,
    Canada,
    Chile,
    Columbia,
    CostaRica,
    Croatia,
    Czechoslovakia,
    Cuba,
    Cyprus,
    Denmark,
    DominicanRepublic,
    Ecuador,
    Egypt,
    England,
    Spain,
    Estonia,
    FaroeIslands,
    Fiji,
    Finland,
    France,
    Gambia,
    GuernseyJersey,
    Georgia,
    Germany,
    Ghana,
    Greece,
    Guatemala,
    Guyana,
    Haiti,
    HongKong,
    Honduras,
    Hungary,
    India,
    Ireland,
    Iran,
    Iraq,
    Iceland,
    Israel,
    Italy,
    IvoryCoast,
    Jamaica,
    Japan,
    Jordan,
    Yugoslavia,
    Kazakhstan,
    Kenya,
    Kyrgyzstan,
    Kuwait,
    Latvia,
    Lebanon,
    Libya,
    Liechtenstein,
    Lithuania,
    Luxembourg,
    Malaysia,
    Mauritania,
    Mexico,
    Mali,
    Malta,
    Monaco,
    Moldova,
    Mongolia,
    Mozambique,
    Morocco,
    Mauritius,
    Myanmar,
    Nicaragua,
    TheInternet,
    Nigeria,
    NetherlandsAntilles,
    Netherlands,
    Norway,
    NewZealand,
    Austria,
    Pakistan,
    Palestine,
    Panama,
    Paraguay,
    Peru,
    Philippines,
    PapuaNewGuinea,
    Poland,
    Portugal,
    PeoplesRepublicOfChina,
    PuertoRico,
    Qatar,
    Indonesia,
    Romania,
    Russia,
    SouthAfrica,
    ElSalvador,
    Scotland,
    AtSea,
    Senegal,
    Seychelles,
    Singapore,
    Slovenia,
    SanMarino,
    AboardSpacecraft,
    SriLanka,
    Sudan,
    Surinam,
    Sweden,
    Switzerland,
    Syria,
    Thailand,
    Turkmenistan,
    Turkey,
    TrinidadAndTobago,
    Tunisia,
    UnitedArabEmirates,
    Uganda,
    Ukraine,
    Unknown,
    Uruguay,
    UnitedStatesOfAmerica,
    Uzbekistan,
    Venezuela,
    BritishVirginIslands,
    Vietnam,
    USVirginIslands,
    Wales,
    Yemen,
    Zambia,
    Zimbabwe,
    Zaire,
}

impl std::str::FromStr for OlympicCountryCode {
    type Err = crate::errors::ChessError;

    #[allow(clippy::too_many_lines)]
    fn from_str(code: &str) -> Result<Self, Self::Err> {
        match code {
            "AFG" => Ok(Self::Afghanistan),
            "AIR" => Ok(Self::AboardAircraft),
            "ALB" => Ok(Self::Albania),
            "ALG" => Ok(Self::Algeria),
            "AND" => Ok(Self::Andorra),
            "ANG" => Ok(Self::Angola),
            "ANT" => Ok(Self::Antigua),
            "ARG" => Ok(Self::Argentina),
            "ARM" => Ok(Self::Armenia),
            "ATA" => Ok(Self::Antarctica),
            "AUS" => Ok(Self::Australia),
            "AZB" => Ok(Self::Azerbaijan),
            "BAN" => Ok(Self::Bangladesh),
            "BAR" => Ok(Self::Bahrain),
            "BHM" => Ok(Self::Bahamas),
            "BEL" => Ok(Self::Belgium),
            "BER" => Ok(Self::Bermuda),
            "BIH" => Ok(Self::BosniaHerzegovina),
            "BLA" => Ok(Self::Belarus),
            "BLG" => Ok(Self::Bulgaria),
            "BLZ" => Ok(Self::Belize),
            "BOL" => Ok(Self::Bolivia),
            "BRB" => Ok(Self::Barbados),
            "BRS" => Ok(Self::Brazil),
            "BRU" => Ok(Self::Brunei),
            "BSW" => Ok(Self::Botswana),
            "CAN" => Ok(Self::Canada),
            "CHI" => Ok(Self::Chile),
            "COL" => Ok(Self::Columbia),
            "CRA" => Ok(Self::CostaRica),
            "CRO" => Ok(Self::Croatia),
            "CSR" => Ok(Self::Czechoslovakia),
            "CUB" => Ok(Self::Cuba),
            "CYP" => Ok(Self::Cyprus),
            "DEN" => Ok(Self::Denmark),
            "DOM" => Ok(Self::DominicanRepublic),
            "ECU" => Ok(Self::Ecuador),
            "EGY" => Ok(Self::Egypt),
            "ENG" => Ok(Self::England),
            "ESP" => Ok(Self::Spain),
            "EST" => Ok(Self::Estonia),
            "FAI" => Ok(Self::FaroeIslands),
            "FIJ" => Ok(Self::Fiji),
            "FIN" => Ok(Self::Finland),
            "FRA" => Ok(Self::France),
            "GAM" => Ok(Self::Gambia),
            "GCI" => Ok(Self::GuernseyJersey),
            "GEO" => Ok(Self::Georgia),
            "GER" => Ok(Self::Germany),
            "GHA" => Ok(Self::Ghana),
            "GRC" => Ok(Self::Greece),
            "GUA" => Ok(Self::Guatemala),
            "GUY" => Ok(Self::Guyana),
            "HAI" => Ok(Self::Haiti),
            "HKG" => Ok(Self::HongKong),
            "HON" => Ok(Self::Honduras),
            "HUN" => Ok(Self::Hungary),
            "IND" => Ok(Self::India),
            "IRL" => Ok(Self::Ireland),
            "IRN" => Ok(Self::Iran),
            "IRQ" => Ok(Self::Iraq),
            "ISD" => Ok(Self::Iceland),
            "ISR" => Ok(Self::Israel),
            "ITA" => Ok(Self::Italy),
            "IVO" => Ok(Self::IvoryCoast),
            "JAM" => Ok(Self::Jamaica),
            "JAP" => Ok(Self::Japan),
            "JRD" => Ok(Self::Jordan),
            "JUG" => Ok(Self::Yugoslavia),
            "KAZ" => Ok(Self::Kazakhstan),
            "KEN" => Ok(Self::Kenya),
            "KIR" => Ok(Self::Kyrgyzstan),
            "KUW" => Ok(Self::Kuwait),
            "LAT" => Ok(Self::Latvia),
            "LEB" => Ok(Self::Lebanon),
            "LIB" => Ok(Self::Libya),
            "LIC" => Ok(Self::Liechtenstein),
            "LTU" => Ok(Self::Lithuania),
            "LUX" => Ok(Self::Luxembourg),
            "MAL" => Ok(Self::Malaysia),
            "MAU" => Ok(Self::Mauritania),
            "MEX" => Ok(Self::Mexico),
            "MLI" => Ok(Self::Mali),
            "MLT" => Ok(Self::Malta),
            "MNC" => Ok(Self::Monaco),
            "MOL" => Ok(Self::Moldova),
            "MON" => Ok(Self::Mongolia),
            "MOZ" => Ok(Self::Mozambique),
            "MRC" => Ok(Self::Morocco),
            "MRT" => Ok(Self::Mauritius),
            "MYN" => Ok(Self::Myanmar),
            "NCG" => Ok(Self::Nicaragua),
            "NET" => Ok(Self::TheInternet),
            "NIG" => Ok(Self::Nigeria),
            "NLA" => Ok(Self::NetherlandsAntilles),
            "NLD" => Ok(Self::Netherlands),
            "NOR" => Ok(Self::Norway),
            "NZD" => Ok(Self::NewZealand),
            "OST" => Ok(Self::Austria),
            "PAK" => Ok(Self::Pakistan),
            "PAL" => Ok(Self::Palestine),
            "PAN" => Ok(Self::Panama),
            "PAR" => Ok(Self::Paraguay),
            "PER" => Ok(Self::Peru),
            "PHI" => Ok(Self::Philippines),
            "PNG" => Ok(Self::PapuaNewGuinea),
            "POL" => Ok(Self::Poland),
            "POR" => Ok(Self::Portugal),
            "PRC" => Ok(Self::PeoplesRepublicOfChina),
            "PRO" => Ok(Self::PuertoRico),
            "QTR" => Ok(Self::Qatar),
            "RIN" => Ok(Self::Indonesia),
            "ROM" => Ok(Self::Romania),
            "RUS" => Ok(Self::Russia),
            "SAF" => Ok(Self::SouthAfrica),
            "SAL" => Ok(Self::ElSalvador),
            "SCO" => Ok(Self::Scotland),
            "SEA" => Ok(Self::AtSea),
            "SEN" => Ok(Self::Senegal),
            "SEY" => Ok(Self::Seychelles),
            "SIP" => Ok(Self::Singapore),
            "SLV" => Ok(Self::Slovenia),
            "SMA" => Ok(Self::SanMarino),
            "SPC" => Ok(Self::AboardSpacecraft),
            "SRI" => Ok(Self::SriLanka),
            "SUD" => Ok(Self::Sudan),
            "SUR" => Ok(Self::Surinam),
            "SVE" => Ok(Self::Sweden),
            "SWZ" => Ok(Self::Switzerland),
            "SYR" => Ok(Self::Syria),
            "TAI" => Ok(Self::Thailand),
            "TMT" => Ok(Self::Turkmenistan),
            "TRK" => Ok(Self::Turkey),
            "TTO" => Ok(Self::TrinidadAndTobago),
            "TUN" => Ok(Self::Tunisia),
            "UAE" => Ok(Self::UnitedArabEmirates),
            "UGA" => Ok(Self::Uganda),
            "UKR" => Ok(Self::Ukraine),
            "UNK" => Ok(Self::Unknown),
            "URU" => Ok(Self::Uruguay),
            "USA" => Ok(Self::UnitedStatesOfAmerica),
            "UZB" => Ok(Self::Uzbekistan),
            "VEN" => Ok(Self::Venezuela),
            "VGB" => Ok(Self::BritishVirginIslands),
            "VIE" => Ok(Self::Vietnam),
            "VUS" => Ok(Self::USVirginIslands),
            "WLS" => Ok(Self::Wales),
            "YEM" => Ok(Self::Yemen),
            "ZAM" => Ok(Self::Zambia),
            "ZIM" => Ok(Self::Zimbabwe),
            "ZRE" => Ok(Self::Zaire),
            _ => Err(crate::errors::ChessError::InvalidPgn(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unknown country code: {code}"),
            ))),
        }
    }
}

impl std::fmt::Display for OlympicCountryCode {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::Afghanistan => "AFG",
            Self::AboardAircraft => "AIR",
            Self::Albania => "ALB",
            Self::Algeria => "ALG",
            Self::Andorra => "AND",
            Self::Angola => "ANG",
            Self::Antigua => "ANT",
            Self::Argentina => "ARG",
            Self::Armenia => "ARM",
            Self::Antarctica => "ATA",
            Self::Australia => "AUS",
            Self::Azerbaijan => "AZB",
            Self::Bangladesh => "BAN",
            Self::Bahrain => "BAR",
            Self::Bahamas => "BHM",
            Self::Belgium => "BEL",
            Self::Bermuda => "BER",
            Self::BosniaHerzegovina => "BIH",
            Self::Belarus => "BLA",
            Self::Bulgaria => "BLG",
            Self::Belize => "BLZ",
            Self::Bolivia => "BOL",
            Self::Barbados => "BRB",
            Self::Brazil => "BRS",
            Self::Brunei => "BRU",
            Self::Botswana => "BSW",
            Self::Canada => "CAN",
            Self::Chile => "CHI",
            Self::Columbia => "COL",
            Self::CostaRica => "CRA",
            Self::Croatia => "CRO",
            Self::Czechoslovakia => "CSR",
            Self::Cuba => "CUB",
            Self::Cyprus => "CYP",
            Self::Denmark => "DEN",
            Self::DominicanRepublic => "DOM",
            Self::Ecuador => "ECU",
            Self::Egypt => "EGY",
            Self::England => "ENG",
            Self::Spain => "ESP",
            Self::Estonia => "EST",
            Self::FaroeIslands => "FAI",
            Self::Fiji => "FIJ",
            Self::Finland => "FIN",
            Self::France => "FRA",
            Self::Gambia => "GAM",
            Self::GuernseyJersey => "GCI",
            Self::Georgia => "GEO",
            Self::Germany => "GER",
            Self::Ghana => "GHA",
            Self::Greece => "GRC",
            Self::Guatemala => "GUA",
            Self::Guyana => "GUY",
            Self::Haiti => "HAI",
            Self::HongKong => "HKG",
            Self::Honduras => "HON",
            Self::Hungary => "HUN",
            Self::India => "IND",
            Self::Ireland => "IRL",
            Self::Iran => "IRN",
            Self::Iraq => "IRQ",
            Self::Iceland => "ISD",
            Self::Israel => "ISR",
            Self::Italy => "ITA",
            Self::IvoryCoast => "IVO",
            Self::Jamaica => "JAM",
            Self::Japan => "JAP",
            Self::Jordan => "JRD",
            Self::Yugoslavia => "JUG",
            Self::Kazakhstan => "KAZ",
            Self::Kenya => "KEN",
            Self::Kyrgyzstan => "KIR",
            Self::Kuwait => "KUW",
            Self::Latvia => "LAT",
            Self::Lebanon => "LEB",
            Self::Libya => "LIB",
            Self::Liechtenstein => "LIC",
            Self::Lithuania => "LTU",
            Self::Luxembourg => "LUX",
            Self::Malaysia => "MAL",
            Self::Mauritania => "MAU",
            Self::Mexico => "MEX",
            Self::Mali => "MLI",
            Self::Malta => "MLT",
            Self::Monaco => "MNC",
            Self::Moldova => "MOL",
            Self::Mongolia => "MON",
            Self::Mozambique => "MOZ",
            Self::Morocco => "MRC",
            Self::Mauritius => "MRT",
            Self::Myanmar => "MYN",
            Self::Nicaragua => "NCG",
            Self::TheInternet => "NET",
            Self::Nigeria => "NIG",
            Self::NetherlandsAntilles => "NLA",
            Self::Netherlands => "NLD",
            Self::Norway => "NOR",
            Self::NewZealand => "NZD",
            Self::Austria => "OST",
            Self::Pakistan => "PAK",
            Self::Palestine => "PAL",
            Self::Panama => "PAN",
            Self::Paraguay => "PAR",
            Self::Peru => "PER",
            Self::Philippines => "PHI",
            Self::PapuaNewGuinea => "PNG",
            Self::Poland => "POL",
            Self::Portugal => "POR",
            Self::PeoplesRepublicOfChina => "PRC",
            Self::PuertoRico => "PRO",
            Self::Qatar => "QTR",
            Self::Indonesia => "RIN",
            Self::Romania => "ROM",
            Self::Russia => "RUS",
            Self::SouthAfrica => "SAF",
            Self::ElSalvador => "SAL",
            Self::Scotland => "SCO",
            Self::AtSea => "SEA",
            Self::Senegal => "SEN",
            Self::Seychelles => "SEY",
            Self::Singapore => "SIP",
            Self::Slovenia => "SLV",
            Self::SanMarino => "SMA",
            Self::AboardSpacecraft => "SPC",
            Self::SriLanka => "SRI",
            Self::Sudan => "SUD",
            Self::Surinam => "SUR",
            Self::Sweden => "SVE",
            Self::Switzerland => "SWZ",
            Self::Syria => "SYR",
            Self::Thailand => "TAI",
            Self::Turkmenistan => "TMT",
            Self::Turkey => "TRK",
            Self::TrinidadAndTobago => "TTO",
            Self::Tunisia => "TUN",
            Self::UnitedArabEmirates => "UAE",
            Self::Uganda => "UGA",
            Self::Ukraine => "UKR",
            Self::Unknown => "UNK",
            Self::Uruguay => "URU",
            Self::UnitedStatesOfAmerica => "USA",
            Self::Uzbekistan => "UZB",
            Self::Venezuela => "VEN",
            Self::BritishVirginIslands => "VGB",
            Self::Vietnam => "VIE",
            Self::USVirginIslands => "VUS",
            Self::Wales => "WLS",
            Self::Yemen => "YEM",
            Self::Zambia => "ZAM",
            Self::Zimbabwe => "ZIM",
            Self::Zaire => "ZRE",
        };
        write!(f, "{code}")
    }
}
