use web_sys::wasm_bindgen::JsCast;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ChessComGame {
    pub pgn: String,
}
impl TryFrom<ChessComGame> for rooky_core::RookyGame {
    type Error = web_sys::wasm_bindgen::JsValue;
    fn try_from(value: ChessComGame) -> Result<Self, Self::Error> {
        let pgn = value.pgn;
        let game = Self::try_from(pgn.as_bytes()).map_err(|e| {
            web_sys::wasm_bindgen::JsValue::from_str(&format!("Failed to parse PGN: {e}"))
        })?;
        Ok(game)
    }
}
impl TryFrom<&ChessComGame> for rooky_core::RookyGame {
    type Error = web_sys::wasm_bindgen::JsValue;
    fn try_from(value: &ChessComGame) -> Result<Self, Self::Error> {
        let game = Self::try_from(value.pgn.as_bytes()).map_err(|e| {
            web_sys::wasm_bindgen::JsValue::from_str(&format!("Failed to parse PGN: {e}"))
        })?;
        Ok(game)
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ChessComResponse {
    pub games: Vec<ChessComGame>,
}

pub struct ChessComClient {
    base_url: &'static str,
    window: web_sys::Window,
}

impl Default for ChessComClient {
    fn default() -> Self {
        Self {
            base_url: "https://api.chess.com/pub",
            window: web_sys::window().expect("No window"),
        }
    }
}

impl ChessComClient {
    #[allow(clippy::future_not_send)]
    async fn get(&self, path: &str) -> Result<web_sys::Response, web_sys::wasm_bindgen::JsValue> {
        let url = format!("{}/{}", self.base_url, path);
        let resp_value =
            wasm_bindgen_futures::JsFuture::from(self.window.fetch_with_str(&url)).await?;
        let resp: web_sys::Response = resp_value.dyn_into()?;
        Ok(resp)
    }
    /// Retrieves a stream of games from Chess.com for a given player, year, and month.
    ///
    /// # Errors
    ///
    /// If the request fails or if the response is not a 200 OK, an error is returned.
    #[allow(clippy::future_not_send)]
    pub async fn find_games(
        &self,
        username: &str,
        year: u32,
        month: u32,
    ) -> Result<
        ChessComGameStream<
            impl futures_util::Stream<
                Item = Result<web_sys::wasm_bindgen::JsValue, web_sys::wasm_bindgen::JsValue>,
            >,
        >,
        web_sys::wasm_bindgen::JsValue,
    > {
        let response = self
            .get(&format!("player/{username}/games/{year:04}/{month:02}/pgn",))
            .await;
        match response {
            Ok(resp) => {
                if resp.status() == 200 {
                    let raw_body = resp.body().ok_or("No body")?;
                    let body = wasm_streams::ReadableStream::from_raw(raw_body);
                    // Convert the JS ReadableStream to a Rust stream
                    Ok(ChessComGameStream::new(body.into_stream()))
                } else {
                    Err(web_sys::wasm_bindgen::JsValue::from_str(&format!(
                        "Error: {}",
                        resp.status()
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }
}

pub struct ChessComGameStream<S> {
    inner: S,
    buffer: Vec<u8>,
}

impl<S> ChessComGameStream<S> {
    pub const fn new(stream: S) -> Self {
        Self {
            inner: stream,
            buffer: Vec::new(),
        }
    }
}
impl<S> futures_util::Stream for ChessComGameStream<S>
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
    use futures_util::StreamExt;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    #[allow(clippy::future_not_send)]
    async fn test_get_pgn_games() {
        let client = ChessComClient::default();
        let mut response = client
            .find_games("Hikaru", 2025, 2)
            .await
            .expect("Failed to fetch games");
        let mut buffer = Vec::new();
        while let Some(game) = response.next().await {
            buffer.push(game);
        }
        assert!(!buffer.is_empty());
        wasm_bindgen_test::console_log!("Received {} games", buffer.len());
    }
}
