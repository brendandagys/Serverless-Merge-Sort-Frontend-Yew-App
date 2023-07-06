use gloo_net::http::Request;
use serde::Deserialize;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Deserialize)]
struct ResponseBody {
    message: String,
}

#[function_component(App)]
fn app() -> Html {
    let input_node_ref = use_node_ref();

    let message_handle = use_state(|| "".to_string());

    let oninput = {
        let input_node_ref = input_node_ref.clone();

        Callback::from(move |_| {
            let input = input_node_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                message_handle.set(input.value());
            }
        })
    };

    let response_handle = use_state(|| "".to_string());
    let response = (*response_handle).clone();

    let make_sort_request = {
        Callback::from(move |_| {
            let response_handle = response_handle.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response: ResponseBody = Request::post("https://sort.brendandagys.com")
                    .body("{\"message\": \"4, 3, 11, 153, 7543, 2, 9, 9, 22, 3, 2, 1\"}")
                    .unwrap()
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                response_handle.set(response.message);
            });
        })
    };

    html!(
        <div>
            <div class="form">
                <h1>{"Merge Sorter"}</h1>
                <div>
                    <input ref={input_node_ref} oninput={oninput} />
                    <button onclick={make_sort_request}>{"Send"}</button>
                </div>

                <small>{"Enter comma-separated numbers to be sorted..."}</small>
            </div>

            <div class="response">
                <h1>{response}</h1>
            </div>
        </div>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
