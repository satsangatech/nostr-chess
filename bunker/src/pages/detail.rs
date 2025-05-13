use nostr_minions::browser_api::IdbStoreManager;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct GameDetailPageProps {
    pub id: String,
}

#[function_component(GameDetailPage)]
pub fn game_detail_page(props: &GameDetailPageProps) -> Html {
    let id = props.id.clone();
    let game = use_state(|| None);
    {
        let game = game.clone();
        use_effect_with(id.clone(), move |id| {
            let id = id.clone();
            yew::platform::spawn_local(async move {
                let game_entry: rooky_core::idb::RookyGameEntry =
                    rooky_core::idb::RookyGameEntry::retrieve_from_store(
                        &web_sys::wasm_bindgen::JsValue::from_str(&id),
                    )
                    .await
                    .expect("Failed to retrieve game");
                let pgn_game = rooky_core::RookyGame::from(&game_entry);
                game.set(Some(pgn_game));
            });
            || {}
        });
    }

    html! {
        <div class="p-4">
            <h1 class="text-2xl font-bold mb-4">{"Game Detail"}</h1>
            { if let Some(game) = (*game).clone() {
                html! { <crate::JsChessGame game={game.clone()} /> }
            } else {
                html! { <p>{"Loading..."}</p> }
            }}
        </div>
    }
}
