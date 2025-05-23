use nostr_minions::browser_api::IdbStoreManager;
use shady_minions::ui::Button;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
enum SortGamesBy {
    Date,
    Opening,
    White,
    Black,
    Outcome,
    Event,
}

#[function_component(GamesPage)]
pub fn games_page() -> Html {
    let filter_state = use_state(|| None::<rooky_core::idb::GameOrigin>);
    let page = use_state(|| 0);
    let total_pages = use_state(|| 0);
    html! {
        <div class="pl-12 h-full flex flex-col justify-evenly py-12 gap-6">
            <h2 class="text-4xl text-white font-black">{"Games"}</h2>
            <div class="flex flex-col justify-evenly gap-6">
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
    let games = use_state(Vec::new);
    let total_setter = props.total_pages.setter();
    let sort_state = use_state(|| SortGamesBy::Date);

    {
        let games = games.clone();
        use_effect_with(
            (filter, props.page.clone(), sort_state.clone()),
            move |(filter, page, sort)| {
                let filter = filter.clone();
                let page = page.clone();
                let sort = sort.clone();
                yew::platform::spawn_local(async move {
                    let mut unfiltered_games =
                        rooky_core::idb::RookyGameEntry::retrieve_all_from_store()
                            .await
                            .unwrap_or_default();
                    let total_pages = (unfiltered_games.len() as f64 / 10.0).ceil() as usize;
                    total_setter.set(total_pages);
                    let start = *page * 10;
                    let end = start + 10;
                    if start >= unfiltered_games.len() {
                        return;
                    }
                    if end > unfiltered_games.len() {
                        unfiltered_games = unfiltered_games[start..].to_vec();
                    } else {
                        unfiltered_games = unfiltered_games[start..end].to_vec();
                    }
                    if let Some(filter) = filter.as_ref() {
                        unfiltered_games.retain(|game| &game.origin == filter);
                    }
                    unfiltered_games.sort_by(|a, b| {
                        let a_game: rooky_core::RookyGame = a.into();
                        let b_game: rooky_core::RookyGame = b.into();
                        match *sort {
                            SortGamesBy::Date => a_game.date.cmp(&b_game.date),
                            SortGamesBy::Opening => a_game
                                .opening()
                                .map(|o| o.name)
                                .unwrap_or_default()
                                .cmp(&b_game.opening().map(|o| o.name).unwrap_or_default()),
                            SortGamesBy::White => a_game.white.cmp(&b_game.white),
                            SortGamesBy::Black => a_game.black.cmp(&b_game.black),
                            SortGamesBy::Outcome => {
                                a_game.outcome.to_string().cmp(&b_game.outcome.to_string())
                            }
                            SortGamesBy::Event => {
                                a_game.event.to_string().cmp(&b_game.event.to_string())
                            }
                        }
                    });
                    games.set(unfiltered_games);
                });
                || {}
            },
        );
    }

    html! {
        <div class="flex flex-col gap-4 flex-1 w-full">
            <div class="grid grid-cols-7 gap-4 bg-zinc-800 rounded-lg w-full px-6 py-3">
                <Button
                    onclick={
                        let sort = sort_state.clone();
                        Callback::from(move |_| {
                            sort.set(SortGamesBy::Date)
                        })
                    }>
                <h3 class="text-2xl text-white font-black">{"Date"}</h3>
                </Button>
                <Button
                    onclick={
                        let sort = sort_state.clone();
                        Callback::from(move |_| {
                            sort.set(SortGamesBy::Opening)
                        })
                    }>
                    <h3 class="text-2xl text-white font-black">{"Opening"}</h3>
                </Button>
                <Button
                    onclick={
                        let sort = sort_state.clone();
                        Callback::from(move |_| {
                            sort.set(SortGamesBy::White)
                        })
                    }>
                    <h3 class="text-2xl text-white font-black">{"White"}</h3>
                </Button>
                <Button
                    onclick={
                        let sort = sort_state.clone();
                        Callback::from(move |_| {
                            sort.set(SortGamesBy::Black)
                        })
                    }>
                    <h3 class="text-2xl text-white font-black">{"Black"}</h3>
                </Button>
                <Button
                    onclick={
                        let sort = sort_state.clone();
                        Callback::from(move |_| {
                            sort.set(SortGamesBy::Outcome)
                        })
                    }>
                    <h3 class="text-2xl text-white font-black">{"Result"}</h3>
                </Button>
                <Button
                    onclick={
                        let sort = sort_state.clone();
                        Callback::from(move |_| {
                            sort.set(SortGamesBy::Event)
                        })
                    }>
                    <h3 class="text-2xl text-white font-black">{"Event"}</h3>
                </Button>
                <h3 class="text-2xl text-white font-black">{"ID"}</h3>
            </div>
            { for (*games).iter().map(|game| {
                let pgn_game = rooky_core::RookyGame::from(game);
                html! {
                    <div class="grid grid-cols-7 gap-4 bg-white rounded-lg w-full px-6 py-3 overflow-hidden">
                        <h3 class="text-lg text-black font-light">{pgn_game.date.format("%Y-%m-%d").to_string()}</h3>
                        <h3 class="text-lg text-black font-light">{pgn_game.opening().map(|o| o.name).unwrap_or_default()}</h3>
                        <h3 class="text-lg text-black font-light">{pgn_game.white.clone()}</h3>
                        <h3 class="text-lg text-black font-light">{pgn_game.black.clone()}</h3>
                        <h3 class="text-lg text-black font-light">{pgn_game.outcome.to_string()}</h3>
                        <h3 class="text-lg text-black font-light">{pgn_game.event.to_string()}</h3>
                        <h3 class="text-lg text-black font-light truncate">{game.note.id.clone()}</h3>
                    </div>
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
    html! {
        <div class="flex flex-row justify-between">
            <div class="flex flex-row gap-4">
                <Button
                    onclick={
                        let filter = filter.clone();
                        Callback::from(move |_| {
                        filter.set(None);
                    })}>
                        {"All"}
                </Button>
                <Button
                    onclick={
                        let filter = filter.clone();
                        Callback::from(move |_| {
                        filter.set(Some(rooky_core::idb::GameOrigin::Annotated));
                    })}>
                        {"Annotated"}
                </Button>
                <Button
                    onclick={
                        let filter = filter.clone();
                        Callback::from(move |_| {
                        filter.set(Some(rooky_core::idb::GameOrigin::Received));
                    })}>
                        {"Received"}
                </Button>
                <Button
                    onclick={
                        let filter = filter.clone();
                        Callback::from(move |_| {
                        filter.set(Some(rooky_core::idb::GameOrigin::Public));
                    })}>
                        {"Public"}
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
