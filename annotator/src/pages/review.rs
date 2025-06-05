use crate::components::{DirectMessageRookyGame, SaveTxtRookyGame, ShareRookyGame};
use yew::prelude::*;

#[function_component(ReviewPage)]
pub fn review_page() -> Html {
    let game_ctx = crate::live_game::use_annotated_game();
    let move_list = &game_ctx.pgn_game().moves;

    html! {
        <>
        <div class="fixed top-8 left-8 ">
            <yew_router::components::Link<crate::router::AnnotatorRoute> to={crate::router::AnnotatorRoute::Home}>
                <shady_minions::ui::Button
                    variant={shady_minions::ui::ButtonVariant::Outline}
                    size={shady_minions::ui::ButtonSize::Small}
                    >
                    <lucide_yew::ArrowLeft class="size-4" />
                </shady_minions::ui::Button>
            </yew_router::components::Link<crate::router::AnnotatorRoute>>
        </div>
        <div class="h-full text-white px-6 flex flex-col justify-between w-full">
            // Header
            <div>
            <div class="text-center mb-6">
                <h2 class="text-muted text-sm mb-2">{"Game Review"}</h2>
                <h1 class="text-xl font-semibold">
                    {match game_ctx.pgn_game().outcome {
                        shakmaty::Outcome::Draw => "Draw",
                        shakmaty::Outcome::Decisive { winner: shakmaty::Color::White } => "White Wins",
                        shakmaty::Outcome::Decisive { winner: shakmaty::Color::Black } => "Black Wins",
                    }}
                </h1>
            </div>

            // Moves List
            <div class="space-y-3 mb-8 max-h-64 overflow-y-auto">
                {
                    move_list.chunks(2).enumerate().map(|(index, chess_move_chunk)| {
                        let move_number = index + 1;
                        let white_move = chess_move_chunk.first().map(|m| m.to_string()).unwrap_or_default();
                        let black_move = chess_move_chunk.get(1).map(|m| m.to_string()).unwrap_or_default();
                        html! {
                            <div class="flex justify-center items-center p-2 bg-background border-muted rounded-md">
                                <span class="text-sm font-semibold mr-1">{move_number}</span>
                                <span class="text-sm text-white">{white_move}</span>
                                <span class="text-sm text-gray-300">{black_move}</span>
                            </div>
                        }

                    }).collect::<Html>()
                }
            </div>
            </div>

            // Bottom Section
            <div class="mt-auto">
              // Info Text
                <div class="text-center mb-6 px-4">
                    <p class="text-muted text-sm leading-relaxed">
                        {"Share your game, send it to a friend, or save it as a text file to save this game."}
                    </p>
                </div>

                <div class="flex gap-3 flex-col">
                    <ShareRookyGame game={game_ctx.pgn_game().clone()} />
                    <DirectMessageRookyGame game={game_ctx.pgn_game().clone()} />
                    <SaveTxtRookyGame game={game_ctx.pgn_game().clone()} />
                </div>
            </div>
        </div>
        </>
    }
}
