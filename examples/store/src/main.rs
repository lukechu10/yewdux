use std::rc::Rc;

use yew::prelude::*;
use yewdux::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone)]
struct State {
    count: u32,
}

enum CountMsg {
    Increment,
}

struct CounterStore {
    state: Rc<State>,
}

impl Store for CounterStore {
    type Model = State;
    type Message = ();
    type Input = CountMsg;
    type Output = ();

    fn new(_link: StoreLink<Self>) -> Self {
        Self {
            state: Rc::new(State { count: 0 }),
        }
    }

    fn state(&mut self) -> &mut Rc<Self::Model> {
        &mut self.state
    }

    fn handle_input(&mut self, msg: Self::Input, _who: HandlerId) -> ShouldNotify {
        let state = Rc::make_mut(&mut self.state);
        match msg {
            CountMsg::Increment => {
                state.count += 1;
            }
        }

        true
    }
}

struct Model {
    dispatch: DispatchProps<CounterStore>,
}

impl Component for Model {
    type Message = ();
    type Properties = DispatchProps<CounterStore>;

    fn create(dispatch: Self::Properties, _link: ComponentLink<Self>) -> Self {
        // Magically increment counter for this example.
        dispatch.send(CountMsg::Increment);
        Self { dispatch }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, handle: Self::Properties) -> ShouldRender {
        self.dispatch.neq_assign(handle)
    }

    fn view(&self) -> Html {
        let count = self.dispatch.state().count;
        let onclick = self.dispatch.callback(|_| CountMsg::Increment);
        html! {
            <>
            <h1>{ count }</h1>
            <button onclick=onclick>{"+1"}</button>
            </>
        }
    }
}

type App = WithDispatch<Model>;

fn main() {
    yew::start_app::<App>();
}
