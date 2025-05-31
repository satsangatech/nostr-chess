use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Eq, Debug)]
pub enum MainRoute {
    #[at("/")]
    Home,
    #[at("/search")]
    Search,
    #[at("/games")]
    MyGames,
    #[at("/annotate")]
    NewGame,
    #[at("/detail/:id")]
    GameDetail { id: String },
    #[at("/settings")]
    Settings,
}
#[derive(Clone, Properties, PartialEq, Eq)]
pub struct MainPanelProps {
    pub route: MainRoute,
}

#[function_component(MainPanel)]
pub fn main_panel(props: &MainPanelProps) -> Html {
    let background = html! {
            <>
                <div class="h-full w-full "></div>
            </>
    };

    let showing_class =
        "absolute inset-0 opacity-100 overflow-hidden transition-all duration-300 ease-in-out scale-100";
    let hidden_class = "absolute inset-0 opacity-0 transition-all duration-300 ease-in-out scale-0";
    html! {
        <div class="relative size-full ">
            <div class="absolute inset-0 z-0">
                {background}
            </div>
            <div class=" z-10 flex flex-col size-full">
                <div class="relative flex-1">
                    <div class={if matches!(props.route, MainRoute::Home) { showing_class } else { hidden_class }}>
                        <crate::pages::HomePage />
                    </div>
                    <div class={if matches!(props.route, MainRoute::Search) { showing_class } else { hidden_class }}>
                        <crate::pages::SearchPage />
                    </div>
                    <div class={if matches!(props.route, MainRoute::MyGames) { showing_class } else { hidden_class }}>
                        <crate::pages::GamesPage />
                    </div>
                    <div class={if matches!(props.route, MainRoute::Settings) { showing_class } else { hidden_class }}>
                        <div class="flex gap-4 size-full items-center justify-evenly">
                            <crate::pages::RelayManagementPage />
                            <crate::pages::KeyRecoveryPage />
                        </div>
                    </div>
                    <div class={if matches!(props.route, MainRoute::NewGame) { showing_class } else { hidden_class }}>
                        <crate::pages::NewJsChessGame />
                    </div>
                    {if let MainRoute::GameDetail { id } = props.route.clone() {
                        html! {
                            <crate::pages::GameDetailPage {id} />
                        }
                    } else {
                        html! {}
                    }}
                </div>
            </div>
        </div>
    }
}

#[function_component(MainPages)]
pub fn resin_pages() -> Html {
    html! {
        <Switch<MainRoute> render = { move |switch: MainRoute| {
            html! { <MainPanel route={switch}/> }
        }}/>
    }
}
