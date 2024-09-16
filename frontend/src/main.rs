use gloo::net::http::Request;
use leptos::*;
//use leptos::error::Result;
use leptos_meta::*;

use nftables::NftOutput;

async fn fetch_tables(_value: i32) -> String {
    let resp = Request::get("/api/v1/get_tables").send().await.unwrap();
    assert_eq!(resp.status(), 200);

    resp.text().await.unwrap()

    //match resp.json::<Result<NftOutput, String>>().await {
    //    Ok(parsed) => "MYTEST".to_string(),
    //    Err(err_msg) => err_msg.to_string(),
    //}
}

#[component]
fn MyCustomHeader() -> impl IntoView {
    view! {
        <header>
            <h1>This is my declaration of war!</h1>
            <p>I am a pretty little child born of sunflowers and gold</p>
            <p>TEST TEST TEST</p>
        </header>
    }
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let once = create_local_resource(move || count.get(), fetch_tables);

    view! {
        <Html
            lang="en"
            dir="ltr"
            attr:data-theme="dark"
        />
        <Stylesheet href="tailwind_output.css"/>
        <Title text="My super awesome test title for my website"/>
        <Meta charset="utf-8"/>
        <Meta name="description" content="Hidden internal description"/>
        <MyCustomHeader />
        <main>
            <aside>
                <p class="p-6 text-4xl">"This is my text"</p>
            </aside>
            <button
                on:click=move |_| {
                    set_count.update(|n| *n += 1);
                    once.refetch();
                }
            >
                "Click me: " {move || count.get()}
            </button>
            <p>"TESTING: " {move || once.get()}</p>
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    provide_meta_context();
    mount_to_body(|| view! { <App/> });
}
