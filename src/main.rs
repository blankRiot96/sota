use yew::prelude::*;

enum Msg {
    AddOne,
    AddTwo,
}

struct CounterComponent {
    count: i64,
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.count += 1;
                true // Re-render component
            }
            Msg::AddTwo => {
                self.count += 2;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="container">
                <p>{self.count}</p>
                <button class="bg-blue-500 hover:bg-blue-300 px-2 py-2 rounded"
                onclick={link.callback(|_| Msg::AddTwo)}>{ "+2" }</button>

                <button
                class="bg-blue-500 hover:bg-blue-300 px-2 py-2 rounded"
                onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
            </div>
        }

    }
}

fn main() {
    yew::start_app::<CounterComponent>();
}
