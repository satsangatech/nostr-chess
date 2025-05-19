use wasm_bindgen::JsValue;
use web_sys::js_sys;
#[derive(Debug, PartialEq, Default, Clone)]
pub enum ChessboardPosition {
    #[default]
    Start,
    Empty,
    Fen(String),
}
impl From<ChessboardPosition> for JsValue {
    fn from(val: ChessboardPosition) -> Self {
        JsValue::from_str(match val {
            ChessboardPosition::Start => "start",
            ChessboardPosition::Empty => "empty",
            ChessboardPosition::Fen(fen) => Box::leak(fen.into_boxed_str()),
        })
    }
}
#[derive(Debug, PartialEq, Default, Clone)]
pub enum ChessboardOrientation {
    #[default]
    White,
    Black,
}
impl From<ChessboardOrientation> for JsValue {
    fn from(val: ChessboardOrientation) -> Self {
        JsValue::from_str(match val {
            ChessboardOrientation::White => "white",
            ChessboardOrientation::Black => "black",
        })
    }
}
#[derive(Debug, PartialEq, Default, Clone)]
pub enum DropOffBoard {
    #[default]
    Snapback,
    Trash,
}
impl From<DropOffBoard> for JsValue {
    fn from(val: DropOffBoard) -> Self {
        JsValue::from_str(match val {
            DropOffBoard::Snapback => "snapback",
            DropOffBoard::Trash => "trash",
        })
    }
}
#[derive(Debug, PartialEq, Default, Clone)]
pub enum AnimationSpeed {
    Slow,
    #[default]
    Fast,
    Custom(u32),
}
impl From<AnimationSpeed> for JsValue {
    fn from(val: AnimationSpeed) -> Self {
        match val {
            AnimationSpeed::Slow => JsValue::from_str("slow"),
            AnimationSpeed::Fast => JsValue::from_str("fast"),
            AnimationSpeed::Custom(speed) => JsValue::from_f64(speed as f64),
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ChessboardConfig {
    pub position: ChessboardPosition,
    pub orientation: ChessboardOrientation,
    pub show_notation: bool,
    pub spare_pieces: bool,
    pub draggable: bool,
    pub drop_off_board: DropOffBoard,
    pub appear_speed: AnimationSpeed,
    pub move_speed: AnimationSpeed,
    pub snapback_speed: AnimationSpeed,
    pub snap_speed: AnimationSpeed,
    pub trash_speed: AnimationSpeed,
    pub piece_theme: &'static str,
    pub on_drag_start: Option<js_sys::Function>,
    pub on_drop: Option<js_sys::Function>,
    pub on_snap_end: Option<js_sys::Function>,
}
impl ChessboardConfig {
    pub fn on_drag_start(
        &mut self,
        on_drag_start: impl Fn(
                wasm_bindgen::JsValue,
                wasm_bindgen::JsValue,
                wasm_bindgen::JsValue,
                wasm_bindgen::JsValue,
            ) -> bool
            + 'static,
    ) {
        let closure = wasm_bindgen::prelude::Closure::wrap(Box::new(on_drag_start)
            as Box<
                dyn Fn(
                    wasm_bindgen::JsValue,
                    wasm_bindgen::JsValue,
                    wasm_bindgen::JsValue,
                    wasm_bindgen::JsValue,
                ) -> bool,
            >);
        self.on_drag_start = Some(closure.into_js_value().into());
    }
    pub fn on_drop(
        &mut self,
        on_drop: impl Fn(wasm_bindgen::JsValue, wasm_bindgen::JsValue) -> wasm_bindgen::JsValue
            + 'static,
    ) {
        let closure = wasm_bindgen::prelude::Closure::wrap(Box::new(on_drop)
            as Box<dyn Fn(wasm_bindgen::JsValue, wasm_bindgen::JsValue) -> wasm_bindgen::JsValue>);
        self.on_drop = Some(closure.into_js_value().into());
    }
    pub fn on_snap_end(&mut self, on_snap_end: impl Fn(web_sys::DragEvent) + 'static) {
        let closure = wasm_bindgen::prelude::Closure::wrap(
            Box::new(on_snap_end) as Box<dyn Fn(web_sys::DragEvent)>
        );
        self.on_snap_end = Some(closure.into_js_value().into());
    }
}
impl Default for ChessboardConfig {
    fn default() -> Self {
        Self {
            position: ChessboardPosition::Start,
            orientation: ChessboardOrientation::White,
            show_notation: true,
            spare_pieces: false,
            draggable: true,
            drop_off_board: DropOffBoard::Snapback,
            appear_speed: AnimationSpeed::Fast,
            move_speed: AnimationSpeed::Fast,
            snapback_speed: AnimationSpeed::Fast,
            snap_speed: AnimationSpeed::Fast,
            trash_speed: AnimationSpeed::Fast,
            piece_theme: "/public/img/pieces/{piece}.svg",
            on_drag_start: None,
            on_drop: None,
            on_snap_end: None,
        }
    }
}
impl From<ChessboardConfig> for JsValue {
    fn from(val: ChessboardConfig) -> Self {
        let config = js_sys::Object::new();
        js_sys::Reflect::set(
            &config,
            &JsValue::from_str("position"),
            &val.position.into(),
        )
        .expect("failed to set position");
        js_sys::Reflect::set(
            &config,
            &JsValue::from_str("orientation"),
            &val.orientation.into(),
        )
        .expect("failed to set orientation");
        js_sys::Reflect::set(
            &config,
            &JsValue::from_str("showNotation"),
            &JsValue::from_bool(val.show_notation),
        )
        .expect("failed to set showNotation");
        js_sys::Reflect::set(
            &config,
            &JsValue::from_str("sparePieces"),
            &JsValue::from_bool(val.spare_pieces),
        )
        .expect("failed to set sparePieces");
        js_sys::Reflect::set(
            &config,
            &JsValue::from_str("draggable"),
            &JsValue::from_bool(val.draggable),
        )
        .expect("failed to set draggable");
        js_sys::Reflect::set(
            &config,
            &JsValue::from_str("dropOffBoard"),
            &val.drop_off_board.into(),
        )
        .expect("failed to set dropOffBoard");
        js_sys::Reflect::set(
            &config,
            &JsValue::from_str("appearSpeed"),
            &val.appear_speed.into(),
        )
        .expect("failed to set appearSpeed");
        js_sys::Reflect::set(
            &config,
            &JsValue::from_str("moveSpeed"),
            &val.move_speed.into(),
        )
        .expect("failed to set moveSpeed");
        js_sys::Reflect::set(
            &config,
            &JsValue::from_str("snapbackSpeed"),
            &val.snapback_speed.into(),
        )
        .expect("failed to set snapbackSpeed");
        js_sys::Reflect::set(
            &config,
            &JsValue::from_str("snapSpeed"),
            &val.snap_speed.into(),
        )
        .expect("failed to set snapSpeed");
        js_sys::Reflect::set(
            &config,
            &JsValue::from_str("trashSpeed"),
            &val.trash_speed.into(),
        )
        .expect("failed to set trashSpeed");
        js_sys::Reflect::set(
            &config,
            &JsValue::from_str("pieceTheme"),
            &JsValue::from_str(val.piece_theme),
        )
        .expect("failed to set pieceTheme");
        if let Some(on_drag_start) = val.on_drag_start {
            js_sys::Reflect::set(
                &config,
                &JsValue::from_str("onDragStart"),
                &on_drag_start.into(),
            )
            .expect("failed to set onDragStart");
        }
        if let Some(on_drop) = val.on_drop {
            js_sys::Reflect::set(&config, &JsValue::from_str("onDrop"), &on_drop.into())
                .expect("failed to set onDrop");
        }
        if let Some(on_snap_end) = val.on_snap_end {
            js_sys::Reflect::set(
                &config,
                &JsValue::from_str("onSnapEnd"),
                &on_snap_end.into(),
            )
            .expect("failed to set onSnapEnd");
        }
        config.into()
    }
}
