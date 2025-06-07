use shady_minions::ui::Button;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
enum SortGamesBy {
    Date(bool),
    Opening(bool),
    White(bool),
    Black(bool),
    Outcome(bool),
    Event(bool),
}

#[function_component(GamesPage)]
pub fn games_page() -> Html {
    let filter_state = use_state(|| None::<rooky_core::idb::GameOrigin>);
    let page = use_state(|| 0);
    let total_pages = use_state(|| 0);
    let language_ctx = crate::contexts::language::use_language_ctx();
    html! {
        <div class="h-full flex flex-col justify-evenly p-12 gap-6">
            <h2 class="text-4xl text-white font-black">{language_ctx.t("common_games")}</h2>
            <div class="flex flex-col justify-evenly gap-6 flex-1">
                <FilterSelector filter={filter_state.clone()} page={page.clone()} total_pages={total_pages.clone()} />
                <GamesList filter={filter_state.clone()} page={page.clone()} total_pages={total_pages.clone()} />
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct GamesFilterProps {
    pub filter: UseStateHandle<Option<rooky_core::idb::GameOrigin>>,
    pub page: UseStateHandle<usize>,
    pub total_pages: UseStateHandle<usize>,
}

#[function_component(GamesList)]
pub fn games_list(props: &GamesFilterProps) -> Html {
    let filter = props.filter.clone();
    let game_ctx = crate::contexts::live_game::use_game_history();
    let language_ctx = crate::contexts::language::use_language_ctx();
    let sort_state = use_state(|| SortGamesBy::Date(true));
    let total_setter = props.total_pages.setter();
    let games = {
        let mut unfiltered_games = game_ctx.rooky_game_entries();
        let filter = filter.clone();
        let page = props.page.clone();
        let sort = sort_state.clone();
        let total_pages = (unfiltered_games.len() as f64 / 5.0).ceil() as usize;
        total_setter.set(total_pages);
        let start = *page * 5;
        let end = start + 5;
        if start >= unfiltered_games.len() {
            vec![]
        } else {
            if let Some(filter) = filter.as_ref() {
                unfiltered_games = unfiltered_games
                    .into_iter()
                    .filter(|game| game.origin == *filter)
                    .collect::<Vec<_>>();
            }
            unfiltered_games.sort_by(|a, b| {
                let a_game: rooky_core::RookyGame = a.into();
                let b_game: rooky_core::RookyGame = b.into();
                match *sort {
                    SortGamesBy::Date(rev) => {
                        if rev {
                            b_game.date.cmp(&a_game.date)
                        } else {
                            a_game.date.cmp(&b_game.date)
                        }
                    }
                    SortGamesBy::Opening(rev) => {
                        if rev {
                            b_game
                                .opening()
                                .map(|o| o.name)
                                .unwrap_or_default()
                                .cmp(&a_game.opening().map(|o| o.name).unwrap_or_default())
                        } else {
                            a_game
                                .opening()
                                .map(|o| o.name)
                                .unwrap_or_default()
                                .cmp(&b_game.opening().map(|o| o.name).unwrap_or_default())
                        }
                    }
                    SortGamesBy::White(rev) => {
                        if rev {
                            b_game.white.cmp(&a_game.white)
                        } else {
                            a_game.white.cmp(&b_game.white)
                        }
                    }
                    SortGamesBy::Black(rev) => {
                        if rev {
                            b_game.black.cmp(&a_game.black)
                        } else {
                            a_game.black.cmp(&b_game.black)
                        }
                    }
                    SortGamesBy::Outcome(rev) => {
                        if rev {
                            b_game.outcome.to_string().cmp(&a_game.outcome.to_string())
                        } else {
                            a_game.outcome.to_string().cmp(&b_game.outcome.to_string())
                        }
                    }
                    SortGamesBy::Event(rev) => {
                        if rev {
                            b_game.event.to_string().cmp(&a_game.event.to_string())
                        } else {
                            a_game.event.to_string().cmp(&b_game.event.to_string())
                        }
                    }
                }
            });
            let len = unfiltered_games.len();
            let start = start.min(len);
            let end = end.clamp(start, len);
            unfiltered_games = unfiltered_games[start..end].to_vec();
            unfiltered_games
        }
    };

    html! {
        <div class="flex flex-col gap-4 flex-1 w-full">
            <div class="grid grid-cols-7 gap-4 bg-zinc-800 rounded-lg w-full px-6 py-3">
                <div class="flex gap-2 items-center ">
                    <h3 class="text-xl text-white font-black">{language_ctx.t("game_details_date")}</h3>
                    <Button
                        variant={shady_minions::ui::ButtonVariant::Outline}
                        size={shady_minions::ui::ButtonSize::Small}
                        onclick={
                            let sort = sort_state.clone();
                            Callback::from(move |_| {
                                if let SortGamesBy::Date(rev) = *sort {
                                    sort.set(SortGamesBy::Date(!rev));
                                } else {
                                    sort.set(SortGamesBy::Date(true));
                                }
                            })}>
                            <lucide_yew::ArrowUpDown class="size-4 text-white" />
                    </Button>

                </div>
                <div class="flex gap-2 items-center">
                    <h3 class="text-xl text-white font-black">{language_ctx.t("common_opening")}</h3>
                    <Button
                        variant={shady_minions::ui::ButtonVariant::Outline}
                        size={shady_minions::ui::ButtonSize::Small}
                        onclick={
                            let sort = sort_state.clone();
                            Callback::from(move |_| {
                                if let SortGamesBy::Opening(rev) = *sort {
                                    sort.set(SortGamesBy::Opening(!rev));
                                } else {
                                    sort.set(SortGamesBy::Opening(true));
                                }
                            })}>
                        <lucide_yew::ArrowUpDown class="size-4 text-white" />
                    </Button>
                </div>
                <div class="flex gap-2 items-center">
                    <h3 class="text-xl text-white font-black">{language_ctx.t("common_white")}</h3>
                    <Button
                        variant={shady_minions::ui::ButtonVariant::Outline}
                        size={shady_minions::ui::ButtonSize::Small}
                        onclick={
                            let sort = sort_state.clone();
                            Callback::from(move |_| {
                                if let SortGamesBy::White(rev) = *sort {
                                    sort.set(SortGamesBy::White(!rev));
                                } else {
                                    sort.set(SortGamesBy::White(true));
                                }
                            })}>
                        <lucide_yew::ArrowUpDown class="size-4 text-white" />
                    </Button>
                </div>
                <div class="flex gap-2 items-center">
                    <h3 class="text-xl text-white font-black">{language_ctx.t("common_black")}</h3>
                    <Button
                        variant={shady_minions::ui::ButtonVariant::Outline}
                        size={shady_minions::ui::ButtonSize::Small}
                        onclick={
                            let sort = sort_state.clone();
                            Callback::from(move |_| {
                                if let SortGamesBy::Black(rev) = *sort {
                                    sort.set(SortGamesBy::Black(!rev));
                                } else {
                                    sort.set(SortGamesBy::Black(true));
                                }
                            })}>
                        <lucide_yew::ArrowUpDown class="size-4 text-white" />
                    </Button>
                </div>
                <div class="flex gap-2 items-center">
                    <h3 class="text-xl text-white font-black">{language_ctx.t("game_details_result")}</h3>
                    <Button
                        variant={shady_minions::ui::ButtonVariant::Outline}
                        size={shady_minions::ui::ButtonSize::Small}
                        onclick={
                            let sort = sort_state.clone();
                            Callback::from(move |_| {
                                if let SortGamesBy::Outcome(rev) = *sort {
                                    sort.set(SortGamesBy::Outcome(!rev));
                                } else {
                                    sort.set(SortGamesBy::Outcome(true));
                                }
                            })}>
                        <lucide_yew::ArrowUpDown class="size-4 text-white" />
                    </Button>
                </div>
                <div class="flex gap-2 items-center">
                    <h3 class="text-xl text-white font-black">{language_ctx.t("game_details_event")}</h3>
                    <Button
                        variant={shady_minions::ui::ButtonVariant::Outline}
                        size={shady_minions::ui::ButtonSize::Small}
                        onclick={
                            let sort = sort_state.clone();
                            Callback::from(move |_| {
                                if let SortGamesBy::Event(rev) = *sort {
                                    sort.set(SortGamesBy::Event(!rev));
                                } else {
                                    sort.set(SortGamesBy::Event(true));
                                }
                            })}>
                        <lucide_yew::ArrowUpDown class="size-4 text-white" />
                    </Button>
                </div>
                <h3 class="text-xl text-white font-black align-center">{language_ctx.t("common_id_title")}</h3>
            </div>
            { for (*games).iter().map(|game| {
                let pgn_game = rooky_core::RookyGame::from(game);
                html! {
                    <yew_router::components::Link<crate::router::MainRoute>
                        to={crate::router::MainRoute::GameDetail {
                            id: game.note.id.clone().unwrap_or_default(),
                        }}
                        >
                    <div class="grid grid-cols-7  gap-4 bg-white rounded-lg w-full px-6 py-3 overflow-hidden h-fit hover:bg-muted">
                        <h3 class="text-lg text-black font-light">{pgn_game.date.format("%Y-%m-%d").to_string()}</h3>
                        <h3 class="text-lg text-black font-light">{pgn_game.opening().map(|o| o.name).unwrap_or_default()}</h3>
                        <h3 class="text-lg text-black font-light">{pgn_game.white.clone()}</h3>
                        <h3 class="text-lg text-black font-light">{pgn_game.black.clone()}</h3>
                        <h3 class="text-lg text-black font-light">{pgn_game.outcome.to_string()}</h3>
                        <h3 class="text-lg text-black font-light">{pgn_game.event.to_string()}</h3>
                        <h3 class="text-lg text-black font-light truncate">{game.note.id.clone()}</h3>
                    </div>
                    </yew_router::components::Link<crate::router::MainRoute>>
                }
            }) }
        </div>
    }
}

#[function_component(FilterSelector)]
pub fn filter_selector(props: &GamesFilterProps) -> Html {
    let filter = props.filter.clone();
    let page = props.page.clone();
    let total_pages = props.total_pages.clone();
    let language_ctx = crate::contexts::language::use_language_ctx();
    html! {
        <div class="flex flex-row justify-between">
            <div class="flex flex-row gap-4">
                <Button
                    onclick={
                        let filter = filter.clone();
                        Callback::from(move |_| {
                        filter.set(None);
                    })}>
                        {language_ctx.t("common_all")}
                </Button>
                <Button
                    onclick={
                        let filter = filter.clone();
                        Callback::from(move |_| {
                        filter.set(Some(rooky_core::idb::GameOrigin::Annotated));
                    })}>
                        {language_ctx.t("common_annotated")}
                </Button>
                <Button
                    onclick={
                        let filter = filter.clone();
                        Callback::from(move |_| {
                        filter.set(Some(rooky_core::idb::GameOrigin::Received));
                    })}>
                        {language_ctx.t("common_received")}
                </Button>
                <Button
                    onclick={
                        let filter = filter.clone();
                        Callback::from(move |_| {
                        filter.set(Some(rooky_core::idb::GameOrigin::Public));
                    })}>
                        {language_ctx.t("common_public")}
                </Button>
            </div>
            <div class="flex flex-row gap-4">
                <Button
                    variant={if *page == 0 {
                        shady_minions::ui::ButtonVariant::Disabled
                    } else {
                        shady_minions::ui::ButtonVariant::Normal
                    }}
                    onclick={
                        let page = page.clone();
                        Callback::from(move |_| {
                        if *page > 0 {
                            page.set(*page - 1);
                        }
                    })}>
                        <lucide_yew::ChevronLeft class="w-6 h-6" />
                </Button>
                <Button
                    variant={if *page == *total_pages {
                        shady_minions::ui::ButtonVariant::Disabled
                    } else {
                        shady_minions::ui::ButtonVariant::Normal
                    }}
                    onclick={
                        let page = page.clone();
                        Callback::from(move |_| {
                        if *page < *total_pages {
                            page.set(*page + 1);
                        }
                    })}>
                        <lucide_yew::ChevronRight class="size-6" />
                </Button>
            </div>

        </div>
    }
}
