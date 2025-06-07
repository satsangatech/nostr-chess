mod expert;
pub mod modal;
pub mod user_profile_card_standalone;

// Re-export the UserProfileCard component
pub use user_profile_card_standalone::UserProfileCard;

mod rookie;
pub use expert::ExpertAnnotation;
pub use rookie::RookieAnnotation;
use yew::prelude::*;

use nostr_minions::browser_api::IdbStoreManager;
use shady_minions::ui::{Button, Form, Input, Popover, PopoverContent, PopoverTrigger};

#[derive(Properties, PartialEq, Clone)]
pub struct RookyGameProps {
    pub game: rooky_core::RookyGame,
}

#[function_component(ShareRookyGame)]
pub fn share_rooky_game(props: &RookyGameProps) -> Html {
    let relay_ctx = use_context::<nostr_minions::relay_pool::NostrRelayPoolStore>()
        .expect("Relay context not found");
    let language_ctx = crate::contexts::language::use_language_ctx();
    let Some(keypair) = nostr_minions::key_manager::use_nostr_key() else {
        return html! {};
    };
    let onclick = {
        let keypair = keypair.clone();
        let game = props.game.clone();
        let relay_ctx = relay_ctx.clone();
        Callback::from(move |_| {
            let mut game_note: nostr_minions::nostro2::NostrNote = game.clone().into();
            keypair
                .sign_note(&mut game_note)
                .expect("Failed to sign note");
            let game_entry = rooky_core::idb::RookyGameEntry {
                id: game_note.id.clone().unwrap_or_default(),
                note: game_note.clone(),
                origin: rooky_core::idb::GameOrigin::Annotated,
            };
            yew::platform::spawn_local(async move {
                game_entry
                    .save_to_store()
                    .await
                    .expect("Failed to save game");
            });
            relay_ctx.send(game_note);
        })
    };

    html! {
        <Button {onclick}>
            <lucide_yew::Share2
                class={classes!("size-5")} />
            <span class="ml-2">{ language_ctx.t("share_to_nostr") }</span>
        </Button>
    }
}
use nostr_minions::nostro2_signer::nostro2_nips::Nip17;
#[function_component(DirectMessageRookyGame)]
pub fn dm_rooky_game(props: &RookyGameProps) -> Html {
    let relay_ctx = use_context::<nostr_minions::relay_pool::NostrRelayPoolStore>()
        .expect("Relay context not found");
    let language_ctx = crate::contexts::language::use_language_ctx();
    let Some(keypair) = nostr_minions::key_manager::use_nostr_key() else {
        return html! {};
    };
    let onsubmit = {
        let keypair = keypair.clone();
        let game = props.game.clone();
        let relay_ctx = relay_ctx.clone();
        Callback::from(move |form: web_sys::HtmlFormElement| {
            let Some(recipient) = form
                .get_with_name("recipient")
                .map(web_sys::wasm_bindgen::JsCast::unchecked_into::<web_sys::HtmlInputElement>)
                .map(|input| input.value())
            else {
                web_sys::console::log_1(&"Recipient not found".into());
                return;
            };
            let note: nostr_minions::nostro2::NostrNote = game.clone().into();
            let note_entry = rooky_core::idb::RookyGameEntry {
                id: note.id.clone().unwrap_or_default(),
                note: note.clone(),
                origin: rooky_core::idb::GameOrigin::Annotated,
            };
            yew::platform::spawn_local(async move {
                note_entry
                    .save_to_store()
                    .await
                    .expect("Failed to save game");
            });
            let dm_game = keypair
                .private_dm(&game.to_pgn(), &recipient)
                .expect("Failed to sign note");
            relay_ctx.send(dm_game);
        })
    };

    html! {
        <Button>
        <Popover>
            <PopoverTrigger>
                <div class="flex items-center gap-2">
                    <lucide_yew::MessageSquareLock class={classes!("size-5")} />
                    <span class="ml-2">{ language_ctx.t("send_nostr_dm") }</span>
                </div>
            </PopoverTrigger>
            <PopoverContent>
                <Form {onsubmit} class="flex gap-2">
                    <Input
                        name="recipient"
                        r#type={shady_minions::ui::InputType::Text}
                        placeholder={ language_ctx.t("enter_recipient_nostr_id") }
                        class={classes!("w-full", "mb-2", "min-w-32")} />
                    <Button r#type={shady_minions::ui::ButtonType::Submit}>
                        <lucide_yew::MessageSquareLock class={classes!("size-5")} />
                    </Button>
                </Form>
                </PopoverContent>
        </Popover>
        </Button>
    }
}

#[function_component(SaveTxtRookyGame)]
pub fn save_txt_rooky_game(props: &RookyGameProps) -> Html {
    let language_ctx = crate::contexts::language::use_language_ctx();
    let game = props.game.clone();
    let mut note: nostr_minions::nostro2::NostrNote = game.clone().into();
    note.serialize_id().expect("Failed to serialize ID");
    let onclick = {
        let game = game.clone();
        let id = note.id.take().unwrap();
        Callback::from(move |_| {
            let note: nostr_minions::nostro2::NostrNote = game.clone().into();
            let note_entry = rooky_core::idb::RookyGameEntry {
                id: note.id.clone().unwrap_or_default(),
                note: note.clone(),
                origin: rooky_core::idb::GameOrigin::Annotated,
            };
            yew::platform::spawn_local(async move {
                note_entry
                    .save_to_store()
                    .await
                    .expect("Failed to save game");
            });
            let blob_parts = web_sys::js_sys::Array::new();
            blob_parts.push(&web_sys::wasm_bindgen::JsValue::from_str(&game.to_pgn()));
            let blob = web_sys::Blob::new_with_str_sequence(&blob_parts).unwrap();

            let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
            let a = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("a")
                .unwrap();
            a.set_attribute("href", &url).unwrap();
            a.set_attribute("download", &format!("game-{id}.pgn"))
                .unwrap();
            a.dispatch_event(&web_sys::MouseEvent::new("click").unwrap())
                .unwrap();
            web_sys::Url::revoke_object_url(&url).unwrap();
        })
    };

    html! {
        <Button {onclick}>
            <lucide_yew::Download class={classes!("size-5")} />
            <span class="ml-2">{ language_ctx.t("share_save_pgn") }</span>
        </Button>
    }
}
