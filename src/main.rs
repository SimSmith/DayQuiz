#![recursion_limit="256"]
use stdweb::web::Date;
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use stdweb::js;
use stdweb::unstable::TryInto;

pub struct Model {
    console: ConsoleService,
    correct_answer: bool,
    question_date: Date,
}

pub enum Msg {
    Answer(Weekday)
}


#[derive(Debug)]
pub enum Weekday {  // Make use of implicit discriminator
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl Weekday {
    pub fn equal(self, nr: i32) -> bool {
        self as i32 == nr
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
            correct_answer: false,
            question_date: random_date(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Answer(day) => {
                self.correct_answer = day.equal(self.question_date.get_day());
                if self.correct_answer {
                    self.console.log("Sō desu!");
                    self.question_date = random_date();
                }
                else {
                    self.console.log("Chigaimasu...");
                }
            }
        }
        self.correct_answer
    }

    fn view(&self) -> Html<Self> {
        let date_txt = date_to_question(&self.question_date);
        let respones_txt = if self.correct_answer {"That was right!"} else {"What was your answer again?"};
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
                <p>{ date_txt }</p>
                <p>{ respones_txt }</p>
                <img id="cheatImage" src="help.png" alt="(A cheatsheet for computing the weekday)"></img>
            </div>
        }
    }
}

fn random_date() -> Date {
    let start_day = Date::utc(2019, 0, 1, 0, 0, 0, 0);
    let end_day = Date::utc(2020, 0, 1, 0, 0, 0, 0);
    let random = js! { return Math.random() };
    let rand_point: f64 = random.try_into().unwrap();  // Between [0,1)
    let rand_time = (end_day - start_day) * rand_point + start_day;
    Date::from_time(rand_time)
}

fn date_to_question(d: &Date) -> String {
    format!("What day was {} / {:02} / {:02}?", d.get_full_year(), d.get_month()+1, d.get_date())
}

fn main() {
    yew::start_app::<Model>();
}