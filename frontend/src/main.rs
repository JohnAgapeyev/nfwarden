use leptos::*;
use leptos_meta::*;

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

    view! {
        <Html
            lang="en"
            dir="ltr"
            attr:data-theme="dark"
        />
        <Title text="My super awesome test title for my website"/>
        <Meta charset="utf-8"/>
        <Meta name="description" content="Hidden internal description"/>
        <MyCustomHeader />
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me: " {move || count.get()}
        </button>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    provide_meta_context();
    mount_to_body(|| view! { <App/> });
}
