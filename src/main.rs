#![recursion_limit="256"]
use stdweb::web::Date;
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    console: ConsoleService,
    correct_answer: bool,
}

pub enum Msg {
    Answer(Weekday)
}


#[derive(Debug)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
            correct_answer: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Answer(Weekday::Monday) => {
                self.console.log("Monday ehh");
            }
            Msg::Answer(Weekday::Tuesday) => {
                self.console.log("Tuesday ehh");
            }
            Msg::Answer(Weekday::Wednesday) => {
                self.console.log("Wednesday ehh");
            }
            Msg::Answer(Weekday::Thursday) => {
                self.console.log("Thursday ehh");
            }
            Msg::Answer(Weekday::Friday) => {
                self.correct_answer = true;
                self.console.log("Friday ehh");
            }
            Msg::Answer(Weekday::Saturday) => {
                self.console.log("Saturday ehh");
            }
            Msg::Answer(Weekday::Sunday) => {
                self.console.log("Sunday ehh");
            }
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <nav class="menu">
                    <button onclick=|_| Msg::Answer(Weekday::Monday)>{ "Monday" }</button>
                    <button onclick=|_| Msg::Answer(Weekday::Tuesday)>{ "Tuesday" }</button>
                    <button onclick=|_| Msg::Answer(Weekday::Wednesday)>{ "Wednesday" }</button>
                    <button onclick=|_| Msg::Answer(Weekday::Thursday)>{ "Thursday" }</button>
                    <button onclick=|_| Msg::Answer(Weekday::Friday)>{ "Friday" }</button>
                    <button onclick=|_| Msg::Answer(Weekday::Saturday)>{ "Saturday" }</button>
                    <button onclick=|_| Msg::Answer(Weekday::Sunday)>{ "Sunday" }</button>
                </nav>
                <p>{ if self.correct_answer {"That's right"} else {"What was your answer again?"} }</p>
                <p>{ Date::new().to_string() }</p>
                <img id="cheatImage" src="help.png" alt="A cheatsheet for computing the weekday."></img>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}