use yew::prelude::*;
use yew_router::prelude::*;
mod route;
use route::Route;

/*enum Msg {
    AddOne,
}*/

fn switch(selected_route: &Route) -> Html {
    match selected_route {
        Route::Home => html! {<div><ShowThisMonthNewspaper/><HideDetails/><div class = "important_text">{"Le journal du mois :"}</div></div>},
        Route::Details => html! {
            <div><HideDetails/>
                <p class = "important_text" style = "transform : translateY(800px); text-align: left; left : 50px;">{"Les articles:"}</p>
                <p class = "important_text" style = "transform : translateY(100px); text-align: left; left : 50px;">{"Edito :"}</p>
            </div>},
        Route::NotFound => html! {<div><HideDetails/><div id = "error">{"404 baby"}</div></div>},
    }
}

#[function_component(ShowThisMonthNewspaper)]
fn show_this_month_newspaper() -> Html{
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.push(Route::Details));
    html! {
        <div>
            <button {onclick} id = "month_journal">{ "Article" }</button>
        </div>
    }
}

#[function_component(HideDetails)]
fn hide_details() -> Html{
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.push(Route::Home));
    html! {
        <div>
            <button {onclick} class = "top_bar_text" style = "left : 100px">{ "Accueil" }</button>
        </div>
    }
}

#[function_component(Main)]
fn main() -> Html {
    html! {
        <div>
            <div id="top_bar"></div>
            
            //<div class="top_bar_text" style = "left : 100px">{"Acceuil"}</div>
            <div class="top_bar_text sections" style = "left : 370px">
                {"Rubriques"} 
                <div class = "sections-content">
                    <p>{"Décryptage"}</p>
                    <p>{"Culture"}</p>
                    <p>{"Monde"}</p>
                    <p>{"France"}</p>
                    <p>{"Autre"}</p>
                </div>
            </div>
            <div class="top_bar_text" style = "left : 700px">{"Informations"}</div>
            <BrowserRouter>
                <div>
                    <Switch<Route> render={Switch::render(switch)}/>
                </div>
            </BrowserRouter>
        </div>
    }
}

fn main() {
    yew::start_app::<Main>();
}