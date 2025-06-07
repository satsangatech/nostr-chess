use shady_minions::ui::{Card, CardContent, CardHeader, CardTitle};
use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let language_ctx = crate::language::use_language_ctx();
    html! {
        <>
            <div class="relative flex flex-col size-full items-center justify-center text-white gap-8">
                <div class="absolute top-12 right-12">
                    <crate::components::NotificationIcon />
                </div>
                <div class="text-center space-y-1">
                    <h2 class="text-7xl font-black mb-4">{language_ctx.t("bunker_welcome_title")}</h2>
                    <p class="text-xl font-bold">{language_ctx.t("bunker_welcome_subtitle")}</p>
                </div>
                <div class="grid grid-cols-2 gap-4 min-w-3xl">
                    <DatabaseSummary />
                    <InboxSummary />
                    <NostrIdSummary />
                </div>
            </div>
        </>
    }
}

#[function_component(InboxSummary)]
pub fn inbox_summary() -> Html {
    let language_ctx = crate::language::use_language_ctx();
    let games_ctx = crate::live_game::use_game_history();
    let games = games_ctx.unread_games().len();

    html! {
        <div class="relative bg-white rounded-[2vw] px-12 py-2 h-36 text-black flex flex-col justify-center">
            <p class="text-6xl font-bold">{format!("{}", games)}</p>
            <p class="text-lg">{language_ctx.t("bunker_games_inbox")}</p>
            <div class="absolute -left-1 top-1/2 bg-secondary rounded-[1vw] h-24 w-4 -translate-y-12"></div>
        </div>
    }
}
#[function_component(DatabaseSummary)]
pub fn database_summary() -> Html {
    let language_ctx = crate::language::use_language_ctx();
    let games_ctx = crate::live_game::use_game_history();
    let games = games_ctx.rooky_game_entries().len();

    html! {
        <div class="relative bg-white rounded-[2vw] px-12 py-2 h-36 text-black flex flex-col justify-center">
            <p class="text-6xl font-bold">{format!("{}", games)}</p>
            <p class="text-lg">{language_ctx.t("bunker_games_database_total")}</p>
            <div class="absolute -left-1 top-1/2 bg-primary rounded-[1vw] h-24 w-4 -translate-y-12"></div>
        </div>
    }
}

#[function_component(NostrIdSummary)]
pub fn nostr_id_summary() -> Html {
    let language_ctx = crate::language::use_language_ctx();
    if nostr_minions::key_manager::use_nostr_key().is_none() {
        return html! {
            <Card class="bg-black border-white">
                <CardHeader>
                    <CardTitle>{language_ctx.t("profile_nostr_id")}</CardTitle>
                </CardHeader>
                <CardContent>
                    <p>{language_ctx.t("profile_no_nostr_keypair")}</p>
                </CardContent>
            </Card>
        };
    };
    html! {
        <div class="relative col-span-2">
            <Card>
                <crate::components::UserProfileCard />
            </Card>
        </div>
    }
}
