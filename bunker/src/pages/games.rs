use nostr_minions::widgets::ag_grid::{create_column, AgGridTheme};
// use shady_minions::ui::Button;
use yew::prelude::*;

#[function_component(GamesPage)]
pub fn games_page() -> Html {
    let games = use_context::<crate::live_game::AnnotatedGameHistoryStore>()
        .expect("Failed to get game history context")
        .rooky_games();

    html! {
        <div class="p-4 h-full mb-4 overflow-y-auto">
            <h1 class="text-2xl font-bold mb-4">{"Games"}</h1>
            // <Button
            //     class="flex items-center gap-2"
            //     onclick={refresh}>
            //     <lucide_yew::RefreshCcwDot
            //         class="size-6 text-primary-foreground cursor-pointer"
            //     />
            //     {"Refresh"}
            // </Button>
            <GameList {games} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct GameListProps {
    pub games: Vec<rooky_core::idb::RookyGameEntry>,
}

use nostr_minions::widgets::ag_grid::AgGridComponent;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct GameRow {
    pub id: String,
    pub opening: String,
    pub event: String,
    pub date: String,
    pub white: String,
    pub black: String,
}

#[function_component(GameList)]
pub fn game_list(props: &GameListProps) -> Html {
    let navigator = yew_router::hooks::use_navigator().expect("Failed to get navigator");
    let columns = vec![
        create_column("date", "Date"),
        create_column("opening", "Opening"),
        create_column("white", "White"),
        create_column("black", "Black"),
        create_column("event", "Event"),
        create_column("id", "ID"),
    ];
    let data = props
        .games
        .iter()
        .map(|game_entry| {
            let note: &nostr_minions::nostro2::note::NostrNote = &game_entry.0;
            let game: rooky_core::RookyGame = game_entry.into();

            GameRow {
                id: note.id.clone().unwrap_or_default(),
                opening: game.opening().map(|o| o.name).unwrap_or_default(),
                event: game.event.to_string(),
                date: game.date.format("%Y-%m-%d").to_string(),
                white: game.white.to_string(),
                black: game.black.to_string(),
            }
        })
        .collect::<Vec<_>>();
    let on_row_clicked = {
        let navigator = navigator.clone();
        web_sys::wasm_bindgen::closure::Closure::wrap(Box::new(
            move |event: web_sys::wasm_bindgen::JsValue| {
                web_sys::console::log_1(&event);
                let data = web_sys::js_sys::Reflect::get(
                    &event,
                    &web_sys::wasm_bindgen::JsValue::from_str("data"),
                )
                .expect("Failed to get data");
                let id = web_sys::js_sys::Reflect::get(
                    &data,
                    &web_sys::wasm_bindgen::JsValue::from_str("id"),
                )
                .expect("Failed to get id");
                let id = id.as_string().expect("Failed to convert id to string");
                navigator.push(&crate::router::MainRoute::GameDetail { id });
            },
        )
            as Box<dyn Fn(web_sys::wasm_bindgen::JsValue)>)
    }
    .into_js_value();

    html! {
            <AgGridComponent<GameRow>
                {data}
                {columns}
                theme={AgGridTheme::Balham}
                class={classes!("h-full", "max-h-[65vh]")}
                {on_row_clicked}
            />
    }
}
