use nostr_minions::browser_api::IdbStoreManager;
use shady_minions::ui::{Card, CardContent, CardHeader, CardTitle};
use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <>
            <div class="flex flex-wrap gap-4 p-4">
                <Card class="w-full">
                    <CardHeader>
                        <CardTitle>{"Welcome to Rooky!"}</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <p>{"This is a simple game database application."}</p>
                    </CardContent>
                </Card>
                <DatabaseSummary />
                <NostrIdSummary />
            </div>
        </>
    }
}

#[function_component(DatabaseSummary)]
pub fn database_summary() -> Html {
    let db = use_state(|| 0);
    {
        let db = db.clone();
        use_effect_with((), move |_| {
            let db = db.clone();
            yew::platform::spawn_local(async move {
                if let Ok(rooky_db) =
                    rooky_core::idb::RookyGameEntry::retrieve_all_from_store().await
                {
                    db.set(rooky_db.len());
                }
            });
            || {}
        });
    }

    html! {
        <Card class="w-full">
            <CardHeader>
                <CardTitle>{"Database Summary"}</CardTitle>
            </CardHeader>
            <CardContent>
                <p>{format!("Total games in database: {}", *db)}</p>
            </CardContent>
        </Card>
    }
}

#[function_component(NostrIdSummary)]
pub fn nostr_id_summary() -> Html {
    let Some(keypair) = nostr_minions::key_manager::use_nostr_key() else {
        return html! {
            <Card class="w-full">
                <CardHeader>
                    <CardTitle>{"Nostr ID Summary"}</CardTitle>
                </CardHeader>
                <CardContent>
                    <p>{"No Nostr keypair found."}</p>
                </CardContent>
            </Card>
        };
    };
    html! {
        <Card class="w-full">
            <CardHeader>
                <CardTitle>{"Nostr ID Summary"}</CardTitle>
            </CardHeader>
            <CardContent>
                <p>{format!("Nostr ID: {}", keypair.npub().expect("couldnt encode npub"))}</p>
            </CardContent>
        </Card>
    }
}
