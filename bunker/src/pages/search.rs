use futures_util::StreamExt;
use nostr_minions::browser_api::IdbStoreManager;
use shady_minions::ui::{
    Button, ButtonType, ButtonVariant, Card, CardContent, CardDescription, CardHeader, CardTitle,
    Checkbox, Form, Input, InputType, Label, Popover, PopoverContent, PopoverPosition,
    PopoverTrigger, Select, SelectContent, SelectItem, SelectTrigger,
};
use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;

#[function_component(SearchPage)]
pub fn search_page() -> Html {
    html! {
        <div class="flex-1 p-4 overflow-y-auto h-full flex gap-8">
            <LichessSearchForm />
            <ChessComSearchForm />
        </div>
    }
}

#[function_component(LichessSearchForm)]
pub fn external_search_form() -> Html {
    let query_state = use_state(external::LichessGameQuery::default);
    let keypair = nostr_minions::key_manager::use_nostr_key();
    let onsubmit = {
        let query_state = query_state.clone();
        Callback::from(move |_| {
            let query_state = (*query_state).clone();

            if let Some(key_ctx) = keypair.clone() {
                yew::platform::spawn_local(async move {
                    let Ok(mut resp) = external::LichessClient::default()
                        .stream_game_history(query_state)
                        .await
                    else {
                        web_sys::console::log_1(&"Failed to get request".into());
                        return;
                    };
                    while let Some(game) = resp.next().await {
                        web_sys::console::log_1(&format!("{:?}", game).into());
                        let mut new_note: nostr_minions::nostro2::note::NostrNote = game.into();
                        key_ctx
                            .sign_note(&mut new_note)
                            .expect("Failed to sign note");
                        let entry: rooky_core::idb::RookyGameEntry =
                            new_note.try_into().expect("couldnt create note");
                        let Ok(_) = entry.save_to_store().await else {
                            web_sys::console::log_1(&"Failed to save note".into());
                            return;
                        };
                    }
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
              <CardTitle>{"Lichess Game Query"}</CardTitle>
              <CardDescription>{"Fill out the form to create a LichessGameQuery struct"}</CardDescription>
            </CardHeader>
            <CardContent>
            <Form {onsubmit} class="space-y-6">
                <div class="space-y-4">
                  <div class="grid gap-2">
                    <Label r#for="username" class="font-medium">
                        {"Username "}
                      <span class="text-red-500">{"*"}</span>
                    </Label>
                    <Input
                      id="username"
                      name="username"
                      placeholder="Lichess username"
                      value={query_state.username.clone()}
                      required={true}
                      onchange={username_state}
                    />
                  </div>

                  <div class="grid gap-2">
                    <Label r#for="max" class="font-medium">
                    {"Max Games"}
                    </Label>
                    <Input
                      id="max"
                      name="max"
                      r#type={InputType::Number}
                      min="1"
                      placeholder="Maximum number of games"
                      value={query_state.max.map_or("".to_string(), |v| v.to_string())}
                      onchange={max_games_state}
                    />
                  </div>

                  <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div class="flex items-center space-x-2">
                      <Checkbox
                        id="rated"
                        checked={query_state.rated.unwrap_or_default()}
                        onchange={
                            let query_state = query_state.clone();
                            move |e: bool| {
                            query_state.set(external::LichessGameQuery {
                                rated: Some(e),
                                ..(*query_state).clone()
                            });
                        }}
                      />
                      <Label r#for="rated" class="font-medium">
                      {"Rated Games Only"}
                      </Label>
                    </div>

                    <div class="grid gap-2">
                      <Label r#for="perfType" class="font-medium">
                      {"Performance Type"}
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
                      {"Since Date"}
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
                                placeholder="YYYY-MM-DD"
                                value={query_state.since.unwrap_or_default().to_string()}
                              />
                        </PopoverContent>
                      </Popover>
                    </div>

                    <div class="grid gap-2">
                      <Label r#for="until" class="font-medium">
                      {"Until Date"}
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
                                placeholder="YYYY-MM-DD"
                                value={query_state.until.unwrap_or_default().to_string()}
                              />
                        </PopoverContent>
                      </Popover>
                    </div>
                  </div>

                  <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div class="grid gap-2">
                      <Label r#for="color" class="font-medium">
                      {"Player Color"}
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
                      <Label r#for="vs" class="font-medium">
                      {"Opponent Username"}
                      </Label>
                      <Input
                        id="vs"
                        name="vs"
                        placeholder="Opponent's username"
                      />
                    </div>
                  </div>

                  <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                    <div class="flex items-center space-x-2">
                      <Checkbox
                        id="finished"
                        checked={query_state.finished.unwrap_or_default()}
                        onchange={
                            let query_state = query_state.clone();
                            move |e: bool| {
                            query_state.set(external::LichessGameQuery {
                                finished: Some(e),
                                ..(*query_state).clone()
                            });
                        }}
                      />
                      <Label r#for="finished" class="font-medium">
                        {"Finished Games"}
                      </Label>
                    </div>

                    <div class="flex items-center space-x-2">
                      <Checkbox
                        id="ongoing"
                        checked={query_state.ongoing.unwrap_or_default()}
                        onchange={
                            let query_state = query_state.clone();
                            move |e: bool| {
                            query_state.set(external::LichessGameQuery {
                                ongoing: Some(e),
                                ..(*query_state).clone()
                            });
                        }}
                      />
                      <Label r#for="ongoing" class="font-medium">
                      {"Ongoing Games"}
                      </Label>
                    </div>

                    <div class="grid gap-2">
                      <Label r#for="sort" class="font-medium">
                      {"Sort Order"}
                      <span class="text-red-500">{"*"}</span>
                      </Label>
                      <Select::<external::LichessSort> id="sort">
                        <SelectTrigger::<external::LichessSort> label="choose sort order"/>
                        <SelectContent::<external::LichessSort>>
                            <SelectItem::<external::LichessSort> value={external::LichessSort::Ascending} />
                            <SelectItem::<external::LichessSort> value={external::LichessSort::Descending} />
                        </SelectContent::<external::LichessSort>>
                      </Select::<external::LichessSort>>
                    </div>
                  </div>
                </div>

                <Button r#type={ButtonType::Submit} class="w-full">
                {"Generate Query"}
                </Button>
                <Button
                    r#type={ButtonType::Reset}
                    variant={ButtonVariant::Outline}
                    class="w-full">
                {"Clear"}
                </Button>
                <Button
                    r#type={ButtonType::Reset}
                    variant={ButtonVariant::Destructive}
                    class="w-full">
                {"Clear"}
                </Button>
              </Form>
            </CardContent>
    </Card>
    }
}
#[function_component(ChessComSearchForm)]
pub fn external_search_form() -> Html {
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
                        let mut new_note: nostr_minions::nostro2::note::NostrNote = game.into();
                        keypair
                            .sign_note(&mut new_note)
                            .expect("Failed to sign note");
                        let entry: rooky_core::idb::RookyGameEntry =
                            new_note.try_into().expect("couldnt create note");
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
              <CardTitle>{"Chess.com Game Query"}</CardTitle>
              <CardDescription>{"Chess.com API can only retrieve a monthly archive of games."}</CardDescription>
            </CardHeader>
            <CardContent>
            <Form {onsubmit} class="space-y-6">
                <div class="space-y-4">
                  <div class="grid gap-2">
                    <Label r#for="username" class="font-medium">
                        {"Username "}
                      <span class="text-red-500">{"*"}</span>
                    </Label>
                    <Input
                      id="username"
                      name="username"
                      placeholder="Chess.com username"
                      required={true}
                    />
                  </div>


                  <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div class="grid gap-2">
                      <Label r#for="month-date" class="font-medium">
                        {"Date"}
                      </Label>
                      <Input
                          id="date"
                          name="date"
                          r#type={InputType::Month}
                          placeholder="YYYY-MM"
                        />
                    </div>
                  </div>
                </div>


                <Button r#type={ButtonType::Submit} class="w-full">
                    {"Generate Query"}
                </Button>
                <Button
                    r#type={ButtonType::Reset}
                    variant={ButtonVariant::Outline}
                    class="w-full">
                    {"Clear"}
                </Button>
                <Button
                    r#type={ButtonType::Reset}
                    variant={ButtonVariant::Destructive}
                    class="w-full">
                    {"Clear"}
                </Button>
              </Form>
            </CardContent>
        </Card>
    }
}
