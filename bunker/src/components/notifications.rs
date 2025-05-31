use yew::prelude::*;

#[function_component(NotificationIcon)]
pub fn notification_icon() -> Html {
    let game_ctx = crate::live_game::use_game_history();
    let has_unread = game_ctx.has_unread();
    html! {
        <shady_minions::ui::Popover>
            <shady_minions::ui::PopoverTrigger>
            <div class="relative flex items-center justify-center p-2">
            { if has_unread {
                html! {
                    <span class="absolute -top-1 -right-1 size-3 bg-red-500 rounded-full"></span>
                }
            } else {
                html! {}
            }}
            <lucide_yew::Bell class={classes!("size-8", "text-white", "hover:text-secondary")} />
            </div>
            </shady_minions::ui::PopoverTrigger>
            <shady_minions::ui::PopoverContent
                position={shady_minions::ui::PopoverPosition::Left}
                class="bg-background text-white p-2 rounded-lg shadow-lg">
                <div class="flex flex-col gap-2">
                    <h3 class="text-lg font-bold">{"Notifications"}</h3>
                    {if game_ctx.unread_games().is_empty() {
                        html! { <p class="text-muted">{"No new notifications."}</p> }
                    } else {
                        game_ctx.unread_games().iter().map(|game| {
                            let rooky_game = rooky_core::RookyGame::from(game.clone());
                            html! {
                                <yew_router::components::Link<crate::router::MainRoute>
                                    to={crate::router::MainRoute::GameDetail {
                                        id: game.note.id.clone().unwrap_or_default(),
                                    }}
                                    >
                                        <div class="rounded hover:bg-primary transition-colors">
                                            <p class="font-semibold">{format!("New game: {}", &game.note.id.clone().unwrap_or_default()[..8])}</p>
                                            <p class="text-sm text-gray-300">{format!("{} vs {}", rooky_game.white, rooky_game.black)}</p>
                                        </div>
                                </yew_router::components::Link<crate::router::MainRoute>>
                            }
                        }).collect::<Html>()
                    }}
                </div>
            </shady_minions::ui::PopoverContent>
        </shady_minions::ui::Popover>
    }
}
