use std::rc::Rc;

use nostr_minions::browser_api::IdbStoreManager;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AnnotatedGameHistory {
    loaded: bool,
    synced: bool,
    pgn_games: Vec<rooky_core::idb::RookyGameEntry>,
}

impl AnnotatedGameHistory {
    #[must_use]
    pub const fn loaded(&self) -> bool {
        self.loaded
    }
    #[must_use]
    pub fn rooky_games(&self) -> Vec<rooky_core::idb::RookyGameEntry> {
        self.pgn_games.clone()
    }
}
pub enum AnnotatedGameHistoryAction {
    Loaded,
    Synced,
    LoadGames(Vec<rooky_core::idb::RookyGameEntry>),
    AddGame(rooky_core::idb::RookyGameEntry),
}

impl Reducible for AnnotatedGameHistory {
    type Action = AnnotatedGameHistoryAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AnnotatedGameHistoryAction::Loaded => Rc::new(Self {
                loaded: true,
                ..(*self).clone()
            }),
            AnnotatedGameHistoryAction::Synced => Rc::new(Self {
                synced: true,
                ..(*self).clone()
            }),
            AnnotatedGameHistoryAction::LoadGames(games) => Rc::new(Self {
                loaded: true,
                pgn_games: games,
                ..(*self).clone()
            }),
            AnnotatedGameHistoryAction::AddGame(game) => {
                let mut pgn_games = self.pgn_games.clone();
                pgn_games.push(game);
                Rc::new(Self {
                    loaded: true,
                    pgn_games,
                    ..(*self).clone()
                })
            }
        }
    }
}

pub type AnnotatedGameHistoryStore = UseReducerHandle<AnnotatedGameHistory>;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct AnnotatedGameHistoryChildren {
    pub children: Children,
}

#[function_component(AnnotatedGameHistoryProvider)]
pub fn key_handler(props: &AnnotatedGameHistoryChildren) -> Html {
    let ctx = use_reducer_eq(AnnotatedGameHistory::default);
    let relay_ctx = use_context::<nostr_minions::relay_pool::NostrRelayPoolStore>()
        .expect("Relay context not found");
    let user_id = nostr_minions::key_manager::use_nostr_key();
    let last_sync = nostr_minions::use_last_sync_time();
    let sub_id = use_state(|| None);
    {
        let dispatcher = ctx.dispatcher();
        use_effect_with((), move |()| {
            let dispatcher = dispatcher.clone();
            yew::platform::spawn_local(async move {
                if let Ok(games) = rooky_core::idb::RookyGameEntry::retrieve_all_from_store().await
                {
                    web_sys::console::log_1(&"Loaded".into());
                    dispatcher.dispatch(AnnotatedGameHistoryAction::LoadGames(games));
                    return;
                }
                web_sys::console::log_1(&"Loaded Empty".into());
                dispatcher.dispatch(AnnotatedGameHistoryAction::Loaded);
            });
            || ()
        });
    }
    {
        let relay_ctx = relay_ctx.clone();
        let sub_id = sub_id.clone();
        use_effect_with((user_id.clone(), last_sync), move |(user_id, last_sync)| {
            if let (Some(pubkey), Some(last_sync)) = (
                user_id.as_ref().map(|u| u.public_key()),
                last_sync.map(|t| t as u64),
            ) {
                let mut inbox_filter = nostr_minions::nostro2::subscriptions::NostrSubscription {
                    kinds: vec![1059].into(),
                    since: Some(last_sync / 1000),
                    ..Default::default()
                };
                inbox_filter.add_tag("#p", &pubkey);
                if let nostr_minions::nostro2::relay_events::NostrClientEvent::Subscribe(_, id, _) =
                    relay_ctx.send(inbox_filter)
                {
                    web_sys::console::log_1(&format!("Subscribing to inbox {}", id).into());
                    sub_id.set(Some(id));
                }
            }
            || {}
        });
    }
    {
        let dispatcher = ctx.dispatcher();
        let set_id = sub_id.clone();
        use_effect_with(relay_ctx.relay_events.clone(), move |notes| {
            if let Some(nostr_minions::nostro2::relay_events::NostrRelayEvent::EndOfSubscription(
                _,
                sub_id,
            )) = notes.last()
            {
                if Some(sub_id) == set_id.as_ref() {
                    web_sys::console::log_1(&"Synced".into());
                    dispatcher.dispatch(AnnotatedGameHistoryAction::Loaded);
                }
            }
            || {}
        });
    }
    {
        let dispatcher = ctx.dispatcher();
        use_effect_with(relay_ctx.unique_notes.clone(), move |notes| {
            if let Some(note) = notes.last().cloned() {
                web_sys::console::log_1(&"Note".into());
                if let Some(user_id) = user_id.as_ref() {
                    if let Ok(dm_note) = user_id.extract_rumor(note) {
                        if let Ok(_pgn_game) = rooky_core::RookyGame::try_from(dm_note.clone()) {
                            web_sys::console::log_1(&"PGN Game".into());
                            let entry = rooky_core::idb::RookyGameEntry {
                                note: dm_note,
                                origin: rooky_core::idb::GameOrigin::Received,
                            };
                            yew::platform::spawn_local(async move {
                                if entry.clone().save_to_store().await.is_ok() {
                                    web_sys::console::log_1(&"Saved".into());
                                    nostr_minions::LastSyncTime::new_sync_time()
                                        .await
                                        .expect("Failed to save sync time");
                                    dispatcher.dispatch(AnnotatedGameHistoryAction::AddGame(entry));
                                } else {
                                    web_sys::console::log_1(&"Failed to save".into());
                                }
                            });
                        }
                    }
                }
            };
            || {}
        });
    }

    html! {
        <ContextProvider<AnnotatedGameHistoryStore> context={ctx}>
            {props.children.clone()}
        </ContextProvider<AnnotatedGameHistoryStore>>
    }
}
