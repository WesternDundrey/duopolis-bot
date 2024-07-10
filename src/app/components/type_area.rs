use leptos::{*,html::Input}


#[component]
pub fn TypeArea(cx: Scope, send: Action<String, Results<String, ServerFnError>>) -> impl IntoView {
    view! {cx,
        <div>
            <form on:submit=move |ev|{
                ev.prevent_default();
                let input = input_ref.get().expect("input to exist");
                send.dispatch(input.value());
                input.set_value("");

            }>
                <input type="text" class = "w-2/3 p-4 border-gray-300 rounded-full" node_ref=input_ref/>

                <input type="submit" class= "h-full p-4 bg-blue-500 text-white rounded-full cursor-pointer"/>

    }
}