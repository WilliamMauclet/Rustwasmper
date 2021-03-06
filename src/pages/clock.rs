use js_sys::Date;
use std::time::Duration;
use yew::{
    prelude::*,
    services::{IntervalService, Task},
};

pub struct Clock {
    seconds: u32,
    // callback_tick: Callback<()>,
    job: Option<Box<dyn Task>>,
}

pub enum Msg {
    Tick,
}

impl Component for Clock {
    type Message = Msg;
    type Properties = ();

    fn create(_: (), link: ComponentLink<Self>) -> Self {
        // ConsoleService::count_named("Clock created");

        let handle = IntervalService::spawn(Duration::from_secs(1), link.callback(|_| Msg::Tick));
    
        Self {
            seconds: 0,
            job: Some(Box::new(handle)),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Tick => {
                self.seconds += 1;
                // ConsoleService::info(&format!("Time: {}", self.seconds));
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        self.seconds = 0;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div>{ &format!("Seconds: {}", self.seconds) } </div>
                <p>{ Date::new_0().to_string().as_string().unwrap() }</p>
            </div>
        }
    }
}
