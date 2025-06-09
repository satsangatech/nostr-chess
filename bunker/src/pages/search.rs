use futures_util::StreamExt;
use nostr_minions::browser_api::IdbStoreManager;
use shady_minions::ui::{
    Button, ButtonType, ButtonVariant, Card, CardContent, CardDescription, CardHeader, CardTitle,
    Form, Input, InputType, Label, Popover, PopoverContent, PopoverPosition, PopoverTrigger,
    Select, SelectContent, SelectItem, SelectTrigger,
};
use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;

#[function_component(SearchPage)]
pub fn search_page() -> Html {
    let selected = use_state(|| None);
    let inner_html = match *selected {
        Some(SearchType::Lichess) => html! { <LichessSearchForm /> },
        Some(SearchType::ChessCom) => html! { <ChessComSearchForm /> },
        None => html! { <SearchPicker selected={selected.clone()} /> },
    };
    let close_search = {
        let selected = selected.clone();
        Callback::from(move |_| {
            selected.set(None);
        })
    };
    let close_button = if selected.is_some() {
        html! {
            <Button
                variant={ButtonVariant::Outline}
                r#type={ButtonType::Button}
                class="absolute top-10 left-4"
                onclick={close_search}
            >
                <lucide_yew::ArrowLeft class="size-8" />
            </Button>
        }
    } else {
        html! {}
    };
    html! {
        <div class="relative flex-1 p-4 overflow-y-auto h-full flex gap-8 flex justify-center items-center">
            {inner_html}
            {close_button}
        </div>
    }
}

#[derive(Copy, Clone, PartialEq)]
enum SearchType {
    Lichess,
    ChessCom,
}

#[derive(Properties, PartialEq)]
struct SearchPickerProps {
    pub selected: UseStateHandle<Option<SearchType>>,
}

#[function_component(SearchPicker)]
fn search_picker(props: &SearchPickerProps) -> Html {
    let selected = props.selected.clone();
    let language_ctx = crate::contexts::language::use_language_ctx();
    html! {
        <Card class="size-fit max-w-3xl mx-auto">
            <CardHeader>
                <CardTitle>{language_ctx.t("games_searching")}</CardTitle>
                <CardDescription class="text-sm text-white">
                    {language_ctx.t("games_searching_description")}
                </CardDescription>
            </CardHeader>
            <CardContent class="space-y-4">
                <h3 class="text-lg font-semibold">{language_ctx.t("game_site_search_question")}</h3>
                <div class="flex flex-col gap-4">
                    <Button
                        r#type={ButtonType::Button}
                        variant={ButtonVariant::Outline}
                        class="w-full"
                        onclick={
                            let selected = selected.clone();
                            Callback::from(move |_| {
                                selected.set(Some(SearchType::Lichess));
                            })
                        }
                    >
                        <img
                            src="https://upload.wikimedia.org/wikipedia/commons/4/47/Lichess_logo_2019.png"
                            alt={language_ctx.t("search_lichess_logo_alt")}
                            class="size-6 mr-2" />
                        {"Lichess"}
                    </Button>
                    <Button
                        r#type={ButtonType::Button}
                        variant={ButtonVariant::Outline}
                        class="w-full"
                        onclick={
                            let selected = selected.clone();
                            Callback::from(move |_| {
                                selected.set(Some(SearchType::ChessCom));
                            })
                        }
                    >
                        <img
                            src="https://images.chesscomfiles.com/uploads/v1/images_users/tiny_mce/PedroPinhata/phpkXK09k.png"
                            alt={language_ctx.t("search_chesscom_logo_alt")}
                            class="size-6 mr-2 object-contain" />
                        {"Chess.com"}
                    </Button>
                </div>
            </CardContent>
        </Card>
    }
}

#[function_component(LichessSearchForm)]
pub fn external_search_form() -> Html {
    let language_ctx = crate::contexts::language::use_language_ctx();
    let query_state = use_state(external::LichessGameQuery::default);
    let keypair = nostr_minions::key_manager::use_nostr_key();
    let game_ctx = crate::live_game::use_game_history();
    let onsubmit = {
        let query_state = query_state.clone();
        let game_ctx = game_ctx.dispatcher();
        Callback::from(move |_| {
            let query_state = (*query_state).clone();

            if let Some(key_ctx) = keypair.clone() {
                let game_ctx = game_ctx.clone();
                yew::platform::spawn_local(async move {
                    let Ok(mut resp) = external::LichessClient::default()
                        .stream_game_history(query_state)
                        .await
                    else {
                        web_sys::console::log_1(&"Failed to get request".into());
                        nostr_minions::widgets::toastify::ToastifyOptions::new_failure(
                            "Failed to fetch games. Please check your query parameters.",
                        )
                        .show();
                        return;
                    };
                    let mut count = 0;
                    while let Some(game) = resp.next().await {
                        count += 1;
                        let mut new_note: nostr_minions::nostro2::NostrNote = game.into();
                        if key_ctx.sign_note(&mut new_note).is_err() {
                            continue;
                        }
                        let entry = rooky_core::idb::RookyGameEntry {
                            id: new_note.id.clone().unwrap_or_default(),
                            note: new_note,
                            origin: rooky_core::idb::GameOrigin::Public,
                        };
                        if entry.clone().save_to_store().await.is_err() {
                            continue;
                        }
                        game_ctx.dispatch(crate::live_game::AnnotatedGameHistoryAction::AddGame(
                            entry.clone(),
                        ));
                    }
                    if count == 0 {
                        nostr_minions::widgets::toastify::ToastifyOptions::new_failure(
                            "No games found for the given query.",
                        )
                        .show();
                        return;
                    }
                    nostr_minions::widgets::toastify::ToastifyOptions::new_success(
                        "Games saved successfully!",
                    )
                    .show();
                });
            }
        })
    };
    let username_state = {
        let query_state = query_state.clone();
        Callback::from(move |username: String| {
            query_state.set(external::LichessGameQuery {
                username,
                ..(*query_state).clone()
            });
        })
    };

    let max_games_state = {
        let query_state = query_state.clone();
        Callback::from(move |max: String| {
            query_state.set(external::LichessGameQuery {
                max: Some(max.parse().unwrap_or_default()),
                ..(*query_state).clone()
            });
        })
    };
    html! {
        <Card class="size-fit max-w-3xl mx-auto">
            <CardHeader>
                <CardTitle>{language_ctx.t("search_lichess_game_query")}</CardTitle>
                <CardDescription class="text-sm text-white">
                    {language_ctx.t("search_lichess_form_description")}
                </CardDescription>
            </CardHeader>
            <CardContent>
            <Form {onsubmit} class="space-y-6">
                <div class="space-y-4">
                  <div class="grid gap-2">
                    <Label r#for="username" class="font-medium">
                        {language_ctx.t("search_username_label")}
                      <span class="text-red-500">{"*"}</span>
                    </Label>
                    <Input
                      id="username"
                      name="username"
                      placeholder={language_ctx.t("search_lichess_username_placeholder")}
                      value={query_state.username.clone()}
                      required={true}
                      onchange={username_state}
                    />
                  </div>

                  <div class="grid gap-2">
                    <Label r#for="max" class="font-medium">
                        {language_ctx.t("search_max_games_label")}
                        <span class="text-red-500">{"*"}</span>
                    </Label>
                    <Input
                        id="max"
                        name="max"
                        r#type={InputType::Number}
                        min="1"
                        required={true}
                        placeholder={language_ctx.t("search_max_games_placeholder")}
                        value={query_state.max.map_or("".to_string(), |v| v.to_string())}
                        onchange={max_games_state}
                        />
                  </div>

                  <div class="grid grid-cols-1 md:grid-cols-2 gap-6">

                    <div class="grid gap-2">
                      <Label r#for="color" class="font-medium">
                      {language_ctx.t("search_player_color")}
                      </Label>
                       <Select::<shakmaty::Color> id="color">
                         <SelectTrigger::<shakmaty::Color> >
                           //<::SelectValue::<shakmaty::Color> placeholder="::Select color" />
                         </SelectTrigger::<shakmaty::Color>>
                         <SelectContent::<shakmaty::Color>>
                            <SelectItem::<shakmaty::Color> value={shakmaty::Color::White} />
                            <SelectItem::<shakmaty::Color> value={shakmaty::Color::Black} />
                         </SelectContent::<shakmaty::Color>>
                       </Select::<shakmaty::Color>>
                    </div>

                    <div class="grid gap-2">
                      <Label r#for="perfType" class="font-medium">
                      {language_ctx.t("search_performance_type")}
                      </Label>
                      <Select::<external::LichessPerfType> id="perfType">
                        <SelectTrigger::<external::LichessPerfType> label="choose perf type"/>
                        <SelectContent::<external::LichessPerfType>>
                            <SelectItem::<external::LichessPerfType> value={external::LichessPerfType::Bullet} />
                            <SelectItem::<external::LichessPerfType> value={external::LichessPerfType::Blitz} />
                            <SelectItem::<external::LichessPerfType> value={external::LichessPerfType::Rapid} />
                            <SelectItem::<external::LichessPerfType> value={external::LichessPerfType::Classical} />
                        </SelectContent::<external::LichessPerfType>>
                      </Select::<external::LichessPerfType>>
                    </div>
                  </div>

                  <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div class="grid gap-2">
                      <Label r#for="since" class="font-medium">
                      {language_ctx.t("search_since_date")}
                      </Label>
                      <Popover>
                        <PopoverTrigger >
                          <Button
                            variant={ButtonVariant::Outline}
                            class={"w-full justify-start text-left font-normal"}
                          >
                            <lucide_yew::Calendar class="size-4" />
                          </Button>
                        </PopoverTrigger>
                        <PopoverContent class="w-auto p-0">
                            <Input
                                id="since"
                                name="since"
                                r#type={InputType::Date}
                                placeholder={language_ctx.t("search_date_format_placeholder")}
                                value={query_state.since.unwrap_or_default().to_string()}
                              />
                        </PopoverContent>
                      </Popover>
                    </div>

                    <div class="grid gap-2">
                      <Label r#for="until" class="font-medium">
                      {language_ctx.t("search_until_date")}
                      </Label>
                      <Popover>
                        <PopoverTrigger >
                          <Button
                            variant={ButtonVariant::Outline}
                            class={"w-full justify-start text-left font-normal"}
                          >
                            <lucide_yew::Calendar class="size-4" />
                          </Button>
                        </PopoverTrigger>
                        <PopoverContent position={PopoverPosition::Top} >
                            <Input
                                id="until"
                                name="until"
                                r#type={InputType::Date}
                                placeholder={language_ctx.t("search_date_format_placeholder")}
                                value={query_state.until.unwrap_or_default().to_string()}
                              />
                        </PopoverContent>
                      </Popover>
                    </div>
                  </div>
                </div>

                <div class="w-full flex gap-2">
                    <Button
                        r#type={ButtonType::Reset}
                        variant={ButtonVariant::Outline}
                        class="flex-1">
                        {language_ctx.t("common_clear")}
                    </Button>
                    <Button r#type={ButtonType::Submit} class="flex-1">
                        {language_ctx.t("search_generate_query")}
                    </Button>
                </div>
              </Form>
            </CardContent>
    </Card>
    }
}
#[function_component(ChessComSearchForm)]
pub fn external_search_form() -> Html {
    let language_ctx = crate::contexts::language::use_language_ctx();
    let keypair = nostr_minions::key_manager::use_nostr_key();
    let onsubmit = {
        Callback::from(move |e: web_sys::HtmlFormElement| {
            let name = e
                .get_with_name("username")
                .map(|x| x.unchecked_into::<web_sys::HtmlInputElement>().value())
                .expect("Failed to get username");

            let month_str = e
                .get_with_name("date")
                .map(|x| x.unchecked_into::<web_sys::HtmlInputElement>().value())
                .expect("Failed to get username");
            let (year, month) = month_str.split_once('-').unwrap_or_default();
            let year = year.parse::<u32>().unwrap_or_default();
            let month = month.parse::<u32>().unwrap_or_default();
            if let Some(keypair) = keypair.clone() {
                yew::platform::spawn_local(async move {
                    let Ok(mut resp) = external::ChessComClient::default()
                        .find_games(&name, year, month)
                        .await
                    else {
                        web_sys::console::log_1(&"Failed to get request".into());
                        return;
                    };
                    while let Some(game) = resp.next().await {
                        let mut new_note: nostr_minions::nostro2::NostrNote = game.into();
                        keypair
                            .sign_note(&mut new_note)
                            .expect("Failed to sign note");
                        let entry = rooky_core::idb::RookyGameEntry {
                            id: new_note.id.clone().unwrap_or_default(),
                            note: new_note,
                            origin: rooky_core::idb::GameOrigin::Public,
                        };
                        let Ok(_) = entry.save_to_store().await else {
                            web_sys::console::log_1(&"Failed to save note".into());
                            return;
                        };
                    }
                });
            }
        })
    };
    html! {
        <Card class="size-fit max-w-3xl mx-auto">
            <CardHeader>
              <CardTitle>{language_ctx.t("search_chesscom_game_query")}</CardTitle>
              <CardDescription>{language_ctx.t("search_chesscom_api_description")}</CardDescription>
            </CardHeader>
            <CardContent>
                <Form {onsubmit} class="space-y-6">
                    <div class="space-y-4">
                      <div class="grid gap-2">
                        <Label r#for="username" class="font-medium">
                            {language_ctx.t("search_username_label")}
                          <span class="text-red-500">{"*"}</span>
                        </Label>
                        <Input
                          id="username"
                          name="username"
                          placeholder={language_ctx.t("search_chesscom_username_placeholder")}
                          required={true}
                        />
                      </div>


                      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div class="grid gap-2">
                          <Label r#for="month-date" class="font-medium">
                            {language_ctx.t("search_date_label")}
                          </Label>
                          <Input
                              id="date"
                              name="date"
                              r#type={InputType::Month}
                              placeholder={language_ctx.t("search_month_format_placeholder")}
                            />
                        </div>
                      </div>
                    </div>

                    <div class="w-full flex gap-2">
                        <Button
                            r#type={ButtonType::Reset}
                            variant={ButtonVariant::Outline}
                            class="flex-1">
                            {language_ctx.t("common_clear")}
                        </Button>
                        <Button r#type={ButtonType::Submit} class="flex-1">
                            {language_ctx.t("search_generate_query")}
                        </Button>
                    </div>
                </Form>
            </CardContent>
        </Card>
    }
}
