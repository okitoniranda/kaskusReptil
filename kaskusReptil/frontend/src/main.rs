use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct App {
    link: ComponentLink<Self>,
    threads: Vec<String>,
    ads: Vec<String>,
}

enum Msg {
    FetchThreads,
    FetchAds,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            link: ctx.link().clone(),
            threads: vec![],
            ads: vec![],
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchThreads => {
                // Simulasi pengambilan data thread
                self.threads = vec!["Thread 1".into(), "Thread 2".into()];
                true
            }
            Msg::FetchAds => {
                // Simulasi pengambilan data ads
                self.ads = vec!["Ad 1".into(), "Ad 2".into()];
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "Kaskus Reptil" }</h1>
                <div>
                    <button onclick={ctx.link().callback(|_| Msg::FetchThreads)}>{ "Lihat Forum" }</button>
                    <ul>
                        { for self.threads.iter().map(|thread| html! { <li>{ thread }</li> }) }
                    </ul>
                </div>
                <div>
                    <button onclick={ctx.link().callback(|_| Msg::FetchAds)}>{ "Lihat Marketplace" }</button>
                    <ul>
                        { for self.ads.iter().map(|ad| html! { <li>{ ad }</li> }) }
                    </ul>
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<App>::new().mount_to_body();
}
