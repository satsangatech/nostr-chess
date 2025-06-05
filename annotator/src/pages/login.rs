use nostr_minions::browser_api::IdbStoreManager;
use nostr_minions::nostro2::NostrSigner;
use shady_minions::ui::{
    Button, Card, CardContent, CardDescription, CardHeader, CardTitle, Form, Input, Modal, Tabs,
    TabsContent, TabsList, TabsTrigger,
};
use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;
#[function_component(NostrLogin)]
pub fn login_form() -> Html {
    let open_modal = use_state(|| false);
    let login_modal = use_state(|| false);
    let language_ctx = crate::contexts::language::use_language_ctx();

    let onclick = {
        let modal = open_modal.clone();
        Callback::from(move |_| {
            modal.set(!(*modal));
        })
    };
    let login_onclick = {
        let modal = login_modal.clone();
        Callback::from(move |_| {
            modal.set(!(*modal));
        })
    };
    html! {
        <div class="flex flex-col items-center justify-center min-h-screen px-6">
        <img
            class={classes!("mb-4",  "mx-auto" , "size-48")}
            src="public/assets/img/splashscreen.svg"
            alt="Login Logo" />
        <Card class={classes!()}>
            <CardHeader>
                <CardTitle>{ language_ctx.t("login_title") }</CardTitle>
                <CardDescription>{ language_ctx.t("login_subtitle") }</CardDescription>
            </CardHeader>
            <CardContent>
                <Tabs default_value="register" class={classes!("w-full")}>
                    <TabsList class={classes!("justify-stretch", "w-full", "flex")}>
                        <TabsTrigger value="register">{ language_ctx.t("login_new_identity") }</TabsTrigger>
                        <TabsTrigger value="login">{ language_ctx.t("login_recover") }</TabsTrigger>
                    </TabsList>
                    <TabsContent value="register">
                        <p class={classes!("text-sm", "text-muted-foreground")}>
                            { language_ctx.t("login_no_key_message") }
                        </p>
                        <Button
                            {onclick}
                            r#type={shady_minions::ui::ButtonType::Button}
                            class={classes!("mt-4", "flex-1")}>
                            { language_ctx.t("login_generate_key") }
                        </Button>
                    </TabsContent>
                    <TabsContent value="login">
                        <p class={classes!("text-sm", "text-muted-foreground")}>
                            { language_ctx.t("login_recover_message") }
                        </p>
                        <Button
                            onclick={login_onclick}
                            r#type={shady_minions::ui::ButtonType::Button}
                            class={classes!("mt-4", "flex-1")}>
                            { language_ctx.t("login_input_key") }
                        </Button>
                    </TabsContent>
                </Tabs>
            </CardContent>
        </Card>
        <Modal is_open={open_modal} >
            <NewKeyForm />
        </Modal>
        <Modal is_open={login_modal} >
            <LoginForm />
        </Modal>
        </div>
    }
}

#[function_component(LoginForm)]
pub fn new_key_form() -> Html {
    let key_ctx = use_context::<nostr_minions::key_manager::NostrIdStore>()
        .expect("KeyManagerProvider not found");
    let language_ctx = crate::contexts::language::use_language_ctx();

    let mnemonic_submit = {
        let key_ctx = key_ctx.dispatcher();
        Callback::from(move |form: web_sys::HtmlFormElement| {
            let mut mnemonic = vec![];
            for i in 1..=24 {
                let Some(word) = form
                    .get_with_name(&format!("word-{i}"))
                    .map(|input| input.unchecked_into::<web_sys::HtmlInputElement>().value())
                else {
                    web_sys::console::log_1(&"Error: Input not found".into());
                    return;
                };
                mnemonic.push(word);
            }
            let mnemonic = mnemonic.join(" ");
            let new_key = nostr_minions::nostro2_signer::keypair::NostrKeypair::parse_mnemonic(
                &mnemonic,
                nostr_minions::nostro2_signer::Language::English,
                true,
            )
            .expect("Failed to create new key");
            let key_ctx = key_ctx.clone();
            yew::platform::spawn_local(async move {
                let pubkey = new_key.public_key();
                let new_id =
                    nostr_minions::key_manager::UserIdentity::from_new_keys(new_key.clone())
                        .await
                        .expect("Failed to create new key");
                new_id
                    .clone()
                    .save_to_store()
                    .await
                    .expect("Failed to save new key");
                key_ctx.dispatch(nostr_minions::key_manager::NostrIdAction::LoadIdentity(
                    pubkey, new_id,
                ));
            });
        })
    };

    let nsec_submit = {
        let key_ctx = key_ctx.dispatcher();
        Callback::from(move |form: web_sys::HtmlFormElement| {
            let Some(input) = form
                .get_with_name("hex-key")
                .map(|input| input.unchecked_into::<web_sys::HtmlInputElement>().value())
            else {
                web_sys::console::log_1(&"Error: Input not found".into());
                return;
            };
            let Ok(mut new_key) =
                input.parse::<nostr_minions::nostro2_signer::keypair::NostrKeypair>()
            else {
                // TODO
                web_sys::console::log_1(&"Error: Invalid NSEC key".into());
                return;
            };
            new_key.set_extractable(true);
            let key_ctx = key_ctx.clone();
            yew::platform::spawn_local(async move {
                let pubkey = new_key.public_key();
                let new_id =
                    nostr_minions::key_manager::UserIdentity::from_new_keys(new_key.clone())
                        .await
                        .expect("Failed to create new key");
                new_id
                    .clone()
                    .save_to_store()
                    .await
                    .expect("Failed to save new key");
                key_ctx.dispatch(nostr_minions::key_manager::NostrIdAction::LoadIdentity(
                    pubkey, new_id,
                ));
            });
        })
    };

    html! {
        <Card>
            <CardHeader>
                <CardTitle>{ language_ctx.t("login_signin") }</CardTitle>
                <CardDescription class={classes!("flex-1")}>
                    { language_ctx.t("login_data_stored_message") }
                </CardDescription>
            </CardHeader>
            <CardContent class={classes!("space-y-4")}>
                <Tabs default_value="mnemonic" class={classes!("w-full")}>
                    <TabsList class={classes!("justify-stretch", "w-full", "flex")}>
                        <TabsTrigger value="mnemonic">{ language_ctx.t("login_secret_phrase") }</TabsTrigger>
                        <TabsTrigger value="hex-key">{ language_ctx.t("login_password") }</TabsTrigger>
                    </TabsList>
                    <TabsContent value="mnemonic" class={classes!("space-y-4")}>
                        <Form onsubmit={mnemonic_submit}>
                            <p class={classes!("font-bold", "text-muted-foreground", "select-none", "pointer-events-none")}>
                                { language_ctx.t("login_keep_order") }
                            </p>
                            <div class={classes!("font-bold", "text-sm", "grid", "grid-cols-3", "gap-2")}>
                                { (1..24).map(|i| {
                                    html! {
                                        <div class={classes!("text-center", "text-sm", "font-bold", "flex", "gap-1")}>
                                            <Input
                                                id={format!("word-{i}")}
                                                placeholder={format!("Word {i}")}
                                                required={true}
                                                r#type={shady_minions::ui::InputType::Text}
                                                class={classes!("text-sm", "font-bold", "text-center")}/>
                                        </div>
                                    }
                                }).collect::<Html>() }
                            </div>
                            <Button
                                r#type={shady_minions::ui::ButtonType::Submit}
                                class={classes!("mt-4", "mr-4")}>
                                { language_ctx.t("common_save") }
                            </Button>
                        </Form>
                    </TabsContent>
                    <TabsContent value="hex-key" class={classes!("space-y-4")}>
                        <Form onsubmit={nsec_submit}>
                        <p class={classes!("font-bold", "text-muted-foreground", "select-none", "pointer-events-none")}>
                            { language_ctx.t("login_nsec_prefix") }
                        </p>
                        <Input
                            id="hex-key"
                            placeholder="nsec_1234567890abcdef"
                            required={true}
                            r#type={shady_minions::ui::InputType::Password}
                            class={classes!("text-sm", "font-bold", "text-center")}/>
                        <Button
                            r#type={shady_minions::ui::ButtonType::Submit}
                            class={classes!("mt-4", "mr-4")}>
                            { language_ctx.t("common_save") }
                        </Button>
                        </Form>
                    </TabsContent>
                </Tabs>
            </CardContent>
        </Card>
    }
}

#[function_component(NewKeyForm)]
pub fn new_key_form() -> Html {
    let key_ctx = use_context::<nostr_minions::key_manager::NostrIdStore>()
        .expect("KeyManagerProvider not found");
    let language_ctx = crate::contexts::language::use_language_ctx();
    let new_key =
        use_state(|| nostr_minions::nostro2_signer::keypair::NostrKeypair::generate(true));
    let mnemonic = new_key
        .mnemonic(nostr_minions::nostro2_signer::Language::English)
        .unwrap_or_default();
    let hex_key = new_key.nsec().unwrap_or_default();
    let onclick = {
        let keys = new_key.clone();
        Callback::from(move |_| {
            let keys = keys.clone();
            let key_ctx = key_ctx.dispatcher();
            yew::platform::spawn_local(async move {
                let pubkey = keys.public_key();
                let new_id =
                    nostr_minions::key_manager::UserIdentity::from_new_keys((*keys).clone())
                        .await
                        .expect("Failed to create new key");
                new_id
                    .clone()
                    .save_to_store()
                    .await
                    .expect("Failed to save new key");
                key_ctx.dispatch(nostr_minions::key_manager::NostrIdAction::LoadIdentity(
                    pubkey, new_id,
                ));
            });
        })
    };
    let copy_key = {
        let hex_keys = hex_key.clone();
        Callback::from(move |_| {
            nostr_minions::browser_api::clipboard_copy(&hex_keys);
        })
    };
    let generate_new_key = {
        let keys = new_key.setter();
        Callback::from(move |_| {
            let new_key = nostr_minions::nostro2_signer::keypair::NostrKeypair::generate(true);
            keys.set(new_key);
        })
    };
    html! {
        <Card>
            <CardHeader>
                <CardTitle>{ language_ctx.t("login_new_key") }</CardTitle>
                <CardDescription class={classes!("flex-1")}>
                    { language_ctx.t(r#"login_key_secret_message"#) }
                </CardDescription>
            </CardHeader>
            <CardContent class={classes!("space-y-4")}>
                <Tabs default_value="mnemonic" class={classes!("w-full")}>
                    <TabsList class={classes!("justify-stretch", "w-full", "flex")}>
                        <TabsTrigger value="mnemonic">{ language_ctx.t("login_seed_phrase") }</TabsTrigger>
                        <TabsTrigger value="hex-key">{ language_ctx.t("login_passkey") }</TabsTrigger>
                    </TabsList>
                    <TabsContent value="mnemonic" class={classes!("space-y-4")}>
                        <p class={classes!("font-bold", "text-muted-foreground", "select-none", "pointer-events-none")}>
                            { language_ctx.t("login_physical_copy") }
                        </p>
                        <div class={classes!("font-bold", "text-sm", "grid", "grid-cols-3", "gap-2")}>
                            { mnemonic.split_whitespace().enumerate().map(|(i, word)| {
                                html! {
                                    <div class={classes!("text-center", "text-sm", "font-bold", "flex", "gap-1")}>
                                        <p class={classes!("font-bold", "text-muted-foreground", "select-none", "pointer-events-none")}>
                                            { format!("{}. ", i + 1) }
                                        </p>
                                        <p class={classes!("text-muted-foreground")}>{ format!("{word}") }</p>
                                    </div>
                                }
                            }).collect::<Html>() }
                        </div>
                    </TabsContent>
                    <TabsContent value="hex-key" class={classes!("space-y-4")}>
                        <p class={classes!("font-bold", "text-muted-foreground", "select-none", "pointer-events-none")}>
                            { language_ctx.t("login_save_key") }
                        </p>
                        <div class={classes!("flex", "gap-2" , "items-center")}>
                        <h3 class={classes!("flex-1", "font-medium", "leading-none", "text-wrap", "max-w-xs", "break-all", "mr-4")}>
                            { &*hex_key }
                        </h3>
                        <Button
                            onclick={copy_key}
                            r#type={shady_minions::ui::ButtonType::Button}
                            variant={shady_minions::ui::ButtonVariant::Outline}>
                            <lucide_yew::Copy class={classes!("h-6", "w-6", "text-muted-foreground")} />
                        </Button>
                        </div>
                    </TabsContent>
                </Tabs>
                <Button
                    r#type={shady_minions::ui::ButtonType::Button}
                    {onclick}
                    class={classes!("mt-4", "mr-4")}>
                    { language_ctx.t("common_save") }
                </Button>
                <Button
                    r#type={shady_minions::ui::ButtonType::Button}
                    variant={shady_minions::ui::ButtonVariant::Outline}
                    onclick={generate_new_key}
                    class={classes!("mt-4", "mr-4")}>
                    { language_ctx.t("login_generate_new") }
                </Button>
            </CardContent>
        </Card>
    }
}
