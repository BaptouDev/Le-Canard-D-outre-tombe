//use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/journal_du_mois")]
    Details,
    #[not_found]
    #[at("/404")]
    NotFound,
}