use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html!(
        <div>
            {r#"Hi!!!"#}
        </div>
    )
}

fn main(){
    yew::start_app::<App>();
}