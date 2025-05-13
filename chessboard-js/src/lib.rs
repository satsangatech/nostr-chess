use wasm_bindgen::prelude::*;
use yew::prelude::*;
mod configs;
pub use configs::*;
#[wasm_bindgen]
extern "C" {
    #[derive(Debug, PartialEq, Clone)]
    pub type ChessBoardJs;

    #[wasm_bindgen(js_name = Chessboard)]
    fn chessboard(id: &str, config: &JsValue) -> ChessBoardJs;
    #[wasm_bindgen(method)]
    fn position(this: &ChessBoardJs, position: &str, is_obj: bool);
}
impl ChessBoardJs {
    pub fn new(id: &str, config: Option<ChessboardConfig>) -> ChessBoardJs {
        let configs: JsValue = match config {
            Some(config) => config.into(),
            None => JsValue::NULL,
        };
        chessboard(id, &configs)
    }
    pub fn set_position(&self, position: &str) {
        self.position(position, true);
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ChessBoardJsProps {
    pub id: &'static str,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub config: Option<ChessboardConfig>,
    #[prop_or_default]
    pub on_board_created: Callback<ChessBoardJs>,
}

#[function_component(ChessBoard)]
pub fn chess_board(props: &ChessBoardJsProps) -> Html {
    let ChessBoardJsProps {
        id,
        class,
        config,
        on_board_created,
        ..
    } = props.clone();
    let id_ref = id;

    use_effect_with((), move |_| {
        let chessboard = ChessBoardJs::new(id_ref, config);
        on_board_created.emit(chessboard);
        || {}
    });
    html! {
        <div {class}>
            <div id={id}/>
        </div>
    }
}
