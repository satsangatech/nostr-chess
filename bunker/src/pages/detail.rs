use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct GameDetailPageProps {
    pub id: String,
}

#[function_component(GameDetailPage)]
pub fn game_detail_page(props: &GameDetailPageProps) -> Html {
    let game_ctx = crate::live_game::use_game_history();
    let game = use_memo((game_ctx.synced, props.id.clone()), |(synced, id)| {
        synced.then(|| {
            game_ctx
                .rooky_game_entries()
                .iter()
                .find(|game| game.note.id.as_ref() == Some(id))
                .map(rooky_core::RookyGame::from)
        })
    });

    html! {
        <div class="pl-12 h-full flex flex-col justify-evenly">
            <h2 class="text-4xl text-white font-black">{"Game Detail"}</h2>
            <div class="flex justify-evenly gap-6">
                { if let Some(game) = (*game).clone().flatten() {
                    html! { <crate::JsChessGame {game} /> }
                } else {
                    html! { <p>{"Loading..."}</p> }
                }}
            </div>
        </div>
    }
}
