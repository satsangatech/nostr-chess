use std::rc::Rc;

use shakmaty::Position;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnnotatedGame {
    has_loaded: bool,
    game_positions: Vec<shakmaty::Chess>,
    pgn_game: rooky_core::RookyGame,
}

impl AnnotatedGame {
    #[must_use]
    pub const fn finished_loading(&self) -> bool {
        self.has_loaded
    }
    #[must_use]
    pub fn last_game_position(&self) -> shakmaty::Chess {
        self.game_positions
            .last()
            .cloned()
            .unwrap_or(shakmaty::Chess::default())
    }
    #[must_use]
    pub fn pgn_game(&self) -> &rooky_core::RookyGame {
        &self.pgn_game
    }
    #[must_use]
    pub fn color_turn(&self) -> shakmaty::Color {
        self.game_positions
            .last()
            .map_or(shakmaty::Color::White, |pos| pos.turn())
    }
    #[must_use]
    pub fn legal_moves(&self) -> Vec<shakmaty::Move> {
        self.game_positions
            .last()
            .cloned()
            .unwrap_or_default()
            .legal_moves()
            .to_vec()
    }
}
pub enum AnnotatedGameAction {
    FinishedLoading,
    Reset,
    PlayMove(shakmaty::Move),
    TakeBack,
    AddOutcome(shakmaty::Outcome),
    AddWhiteName(String),
    AddBlackName(String),
    ChangeDate(chrono::NaiveDate),
    UpdateEventDetails {
        event: String,
        site: String,
        round: String,
    },
}

impl Reducible for AnnotatedGame {
    type Action = AnnotatedGameAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AnnotatedGameAction::FinishedLoading => Rc::new(Self {
                has_loaded: true,
                ..(*self).clone()
            }),
            AnnotatedGameAction::Reset => Rc::new(Self {
                game_positions: vec![shakmaty::Chess::default()],
                pgn_game: rooky_core::RookyGame::default(),
                ..(*self).clone()
            }),
            AnnotatedGameAction::AddOutcome(outcome) => {
                let mut pgn_game = self.pgn_game.clone();
                pgn_game = pgn_game.add_result(outcome);
                Rc::new(Self {
                    pgn_game,
                    ..(*self).clone()
                })
            }
            AnnotatedGameAction::TakeBack => {
                let mut game_positions = self.game_positions.clone();
                let _ = game_positions.pop();
                let mut pgn_game = self.pgn_game.clone();
                let _ = pgn_game.take_back();
                Rc::new(Self {
                    game_positions,
                    pgn_game,
                    ..(*self).clone()
                })
            }
            AnnotatedGameAction::PlayMove(mv) => {
                let game_position = self.game_positions.last().cloned().unwrap_or_default();
                let Ok(new_position) = game_position.clone().play(&mv) else {
                    return Rc::new(Self { ..(*self).clone() });
                };
                let mut pgn_game = self
                    .pgn_game
                    .clone()
                    .new_move(shakmaty::san::SanPlus::from_move(game_position, &mv));
                if new_position.is_checkmate() || new_position.is_stalemate() {
                    if let Some(outcome) = new_position.outcome() {
                        pgn_game = pgn_game.add_result(outcome);
                    };
                }
                let mut game_positions = self.game_positions.clone();
                game_positions.push(new_position);
                Rc::new(Self {
                    game_positions,
                    pgn_game,
                    ..(*self).clone()
                })
            }
            AnnotatedGameAction::AddWhiteName(name) => {
                let mut pgn_game = self.pgn_game.clone();
                pgn_game = pgn_game.add_white_name(name);
                Rc::new(Self {
                    pgn_game,
                    ..(*self).clone()
                })
            }
            AnnotatedGameAction::AddBlackName(name) => {
                let mut pgn_game = self.pgn_game.clone();
                pgn_game = pgn_game.add_black_name(name);
                Rc::new(Self {
                    pgn_game,
                    ..(*self).clone()
                })
            }
            AnnotatedGameAction::ChangeDate(date) => {
                let mut pgn_game = self.pgn_game.clone();
                pgn_game = pgn_game.add_date(date);
                Rc::new(Self {
                    pgn_game,
                    ..(*self).clone()
                })
            }
            AnnotatedGameAction::UpdateEventDetails { event, site, round } => {
                let mut pgn_game = self.pgn_game.clone();
                pgn_game = pgn_game.add_event(event).add_site(site).add_round(round);
                Rc::new(Self {
                    pgn_game,
                    ..(*self).clone()
                })
            }
        }
    }
}

pub type AnnotatedGameStore = UseReducerHandle<AnnotatedGame>;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct AnnotatedGameChildren {
    pub children: Children,
}

#[function_component(AnnotatedGameProvider)]
pub fn key_handler(props: &AnnotatedGameChildren) -> Html {
    let pgn_game = rooky_core::RookyGame::default().add_date(chrono::Local::now().date_naive());
    let ctx = use_reducer(|| AnnotatedGame {
        has_loaded: true,
        game_positions: vec![shakmaty::Chess::new()],
        pgn_game,
    });

    let navigator = yew_router::hooks::use_navigator().expect("Navigator not found");
    use_effect_with(ctx.clone(), move |game| {
        if let Some(outcome) = game.last_game_position().outcome() {
            game.dispatch(crate::live_game::AnnotatedGameAction::AddOutcome(outcome));
            navigator.push(&crate::router::AnnotatorRoute::Review);
        }
        || {}
    });

    html! {
        <ContextProvider<AnnotatedGameStore> context={ctx}>
            {props.children.clone()}
        </ContextProvider<AnnotatedGameStore>>
    }
}

#[hook]
pub fn use_annotated_game() -> AnnotatedGameStore {
    use_context::<AnnotatedGameStore>().expect("AnnotatedGameStore context")
}
