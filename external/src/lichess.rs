use web_sys::wasm_bindgen::JsCast;
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum LichessPerfType {
    UltraBullet,
    Bullet,
    #[default]
    Blitz,
    Rapid,
    Classical,
    Correspondence,
    Crazyhouse,
    Chess960,
    KingOfTheHill,
    ThreeCheck,
    Antichess,
    Atomic,
    Horde,
    RacingKings,
}
impl std::fmt::Display for LichessPerfType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::UltraBullet => "ultraBullet",
                Self::Bullet => "bullet",
                Self::Blitz => "blitz",
                Self::Rapid => "rapid",
                Self::Classical => "classical",
                Self::Correspondence => "correspondence",
                Self::Crazyhouse => "crazyhouse",
                Self::Chess960 => "chess960",
                Self::KingOfTheHill => "kingOfTheHill",
                Self::ThreeCheck => "threeCheck",
                Self::Antichess => "antichess",
                Self::Atomic => "atomic",
                Self::Horde => "horde",
                Self::RacingKings => "racingKings",
            }
        )
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum LichessSort {
    Ascending,
    #[default]
    Descending,
}
impl std::fmt::Display for LichessSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ascending => "ascending",
                Self::Descending => "descending",
            }
        )
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum ChessColor {
    #[default]
    White,
    Black,
}
impl std::fmt::Display for ChessColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::White => "white",
                Self::Black => "black",
            }
        )
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct LichessGameQuery {
    pub username: String,
    pub max: Option<u32>,
    pub rated: Option<bool>,
    pub perf_type: Option<LichessPerfType>,
    pub since: Option<u32>,
    pub until: Option<u32>,
    pub color: Option<ChessColor>,
    pub vs: Option<String>,
    pub finished: Option<bool>,
    pub ongoing: Option<bool>,
    pub sort: LichessSort,
}
impl std::fmt::Display for LichessGameQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut min_url = format!("games/user/{}", self.username);
        if let Some(max) = self.max {
            min_url.push_str(&format!("?max={max}"));
        };
        if let Some(rated) = self.rated {
            min_url.push_str(&format!("&rated={rated}"));
        };
        if let Some(perf_type) = self.perf_type {
            min_url.push_str(&format!("&perfType={perf_type}"));
        };
        if let Some(since) = self.since {
            min_url.push_str(&format!("&since={since}"));
        };
        if let Some(until) = self.until {
            min_url.push_str(&format!("&until={until}"));
        };
        if let Some(color) = self.color {
            min_url.push_str(&format!("&color={color}"));
        };
        if let Some(vs) = &self.vs {
            min_url.push_str(&format!("&vs={vs}"));
        };
        if let Some(finished) = self.finished {
            min_url.push_str(&format!("&finished={finished}"));
        };
        if let Some(ongoing) = self.ongoing {
            min_url.push_str(&format!("&ongoing={ongoing}"));
        };
        min_url.push_str(&format!("&sort={}", self.sort));
        write!(f, "{min_url}")
    }
}

pub struct LichessClient {
    url: &'static str,
    window: web_sys::Window,
}
impl Default for LichessClient {
    fn default() -> Self {
        Self {
            url: "https://lichess.org/api",
            window: web_sys::window().expect("No window"),
        }
    }
}
impl LichessClient {
    #[allow(clippy::future_not_send)]
    async fn get_request(
        &self,
        path: &str,
    ) -> Result<web_sys::Response, web_sys::wasm_bindgen::JsValue> {
        let url = format!("{}/{path}", self.url);
        let resp_value =
            wasm_bindgen_futures::JsFuture::from(self.window.fetch_with_str(&url)).await?;
        let resp: web_sys::Response = resp_value.dyn_into()?;
        Ok(resp)
    }
    /// Stream lichess game history  
    ///
    /// This function will return a stream of lichess games.
    /// The stream will be closed when the lichess server closes the connection.
    ///
    /// # Errors
    ///
    /// This function will return an error if the lichess server returns an error.
    #[allow(clippy::future_not_send)]
    pub async fn stream_game_history(
        &self,
        query: LichessGameQuery,
    ) -> Result<
        LichessGameStream<
            impl futures_util::Stream<
                Item = Result<web_sys::wasm_bindgen::JsValue, web_sys::wasm_bindgen::JsValue>,
            >,
        >,
        // (),
        web_sys::wasm_bindgen::JsValue,
    > {
        let url = query.to_string();
        let resp = self.get_request(&url).await?;
        let raw_body = resp.body().ok_or("No body")?;

        let body = wasm_streams::ReadableStream::from_raw(raw_body);
        // Convert the JS ReadableStream to a Rust stream
        Ok(LichessGameStream::new(body.into_stream()))
    }
}

pub struct LichessGameStream<S> {
    inner: S,
    buffer: Vec<u8>,
}

impl<S> LichessGameStream<S> {
    pub const fn new(stream: S) -> Self {
        Self {
            inner: stream,
            buffer: Vec::new(),
        }
    }
}
impl<S> futures_util::Stream for LichessGameStream<S>
where
    S: futures_util::Stream<
            Item = Result<web_sys::wasm_bindgen::JsValue, web_sys::wasm_bindgen::JsValue>,
        > + Unpin,
{
    type Item = rooky_core::RookyGame;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        use std::task::Poll;
        let this = self.get_mut();

        loop {
            // Try to extract a complete game first
            if let Some(pos) = this.buffer.windows(3).position(|w| w == b"\n\n\n") {
                let (game_bytes, rest) = this.buffer.split_at(pos + 3);
                if let Ok(game) = rooky_core::RookyGame::try_from(game_bytes) {
                    this.buffer = rest.to_vec();
                    return Poll::Ready(Some(game));
                }
                this.buffer = rest.to_vec();
                continue;
            }

            // No complete game in buffer yet: try to poll underlying stream
            match std::pin::Pin::new(&mut this.inner).poll_next(cx) {
                Poll::Ready(Some(Ok(chunk))) => {
                    let new_bytes = web_sys::js_sys::Uint8Array::new(&chunk).to_vec();
                    this.buffer.extend_from_slice(&new_bytes);
                }
                Poll::Ready(Some(Err(_))) => continue, // skip error chunks
                Poll::Ready(None) => {
                    // Stream ended
                    return Poll::Ready(None);
                }
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    //   #[wasm_bindgen_test]
    async fn _test_lichess_client() {
        let client = LichessClient::default();
        let query = LichessGameQuery {
            username: "dadroatan".to_string(),
            max: Some(5),
            ..Default::default()
        };
        let stream = client.stream_game_history(query).await;
        assert!(stream.is_ok());
        let mut stream = stream.unwrap();
        let mut received = 0;
        let mut buffer = Vec::new();
        use futures_util::StreamExt;
        while let Some(game) = stream.next().await {
            wasm_bindgen_test::console_log!("Received game: {:?}", game);
            received += 1;
            buffer.push(game);
        }
        assert!(received == 5);
    }
}
