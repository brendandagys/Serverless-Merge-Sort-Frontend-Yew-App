use gloo_net::http::Request;
use web_sys::HtmlInputElement;
use yew::prelude::*;

const API_ENDPOINT: &str = "https://api.sort.brendandagys.com";

#[function_component(App)]
fn app() -> Html {
    let input_node_ref = use_node_ref();

    let message_handle = use_state(|| "".to_string());

    let response_handle = use_state(|| "".to_string());
    let response = (*response_handle).clone();

    let oninput = {
        let input_node_ref = input_node_ref.clone();
        let message_handle = message_handle.clone();

        Callback::from(move |_| {
            if let Some(input) = input_node_ref.cast::<HtmlInputElement>() {
                message_handle.set(input.value());
            }
        })
    };

    let send_api_request = {
        let message = (*message_handle).clone();

        Callback::from(move |_| {
            let message = message.clone();
            let response_handle = response_handle.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let response: Vec<i32> = Request::post(API_ENDPOINT)
                    .body(message.replace("[", "").replace("]", "").replace(" ", ""))
                    .unwrap()
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                response_handle.set(format!("{:?}", response));
            });
        })
    };

    let onkeydown = {
        let message = (*message_handle).clone();
        let send_api_request = send_api_request.clone();

        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" && !message.trim().is_empty() {
                send_api_request.emit(());
            }
        })
    };

    let onclick = Callback::from(move |_| {
        if !(*message_handle).trim().is_empty() {
            send_api_request.emit(());
        }
    });

    html!(
        <div class="box">
            <div class="form">
                <h1>{"Merge Sorter"}</h1>
                <img src="static/favicon.png" alt="logo" />
                <div>
                    <input ref={input_node_ref} {oninput} {onkeydown} />
                    <button {onclick}>{"Sort"}</button>
                </div>

                <small>{"Enter comma-separated numbers to be sorted..."}</small>
            </div>

            <div class="response"><h1>{response}</h1></div>
        </div>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
