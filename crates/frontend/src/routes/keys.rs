use yew::prelude::*;

#[derive(PartialEq)]
pub struct Key {
    pub name: String,
    pub description: String,
    pub active: bool,
}

#[derive(Properties, PartialEq)]
pub struct KeyListProps {
    keys: Vec<Key>,
}

#[function_component(KeyList)]
pub fn key_list(props: &KeyListProps) -> Html {
    props
        .keys
        .iter()
        .map(|key| {
            html! {
                <tr>
                    <td>{ key.name.to_string() }</td>
                    <td>{ key.description.to_string() }</td>
                    <td>
                        {
                            if key.active {
                                "Active".to_string()
                            } else {
                                "Inactive".to_string()
                            }
                        }
                    </td>
                    <td>
                        <a class="btn btn-outline-primary" href="#" role="button">{ "Edit" }</a>
                    </td>
                </tr>
            }
        })
        .collect()
}

#[function_component(Keys)]
pub fn keys() -> Html {
    let keys = vec![
        Key {
            name: "key1".to_string(),
            description: "this is key 1".to_string(),
            active: true,
        },
        Key {
            name: "key2".to_string(),
            description: "this is key 2".to_string(),
            active: true,
        },
        Key {
            name: "key4".to_string(),
            description: "this is key for".to_string(),
            active: true,
        },
    ];
    html! {
            <div class="container text-light my-3">
        <div class="row justify-content-center">
            <div style="text-align: center">
                <h2>{"Keys"}</h2>
                <div class="container py-2">
                    <a class="btn btn-primary" href="/add-key" role="button">{ "Add Key" }</a>
                </div>
                <table class="table table-striped table-hover table-bordered table-dark">
                    <thead class="table-dark">
                        <tr>
                            <th>{ "Key" }</th>
                            <th>{ "Description" }</th>
                            <th>{ "Status" }</th>
                            <th></th>
                        </tr>
                    </thead>
                    <tbody>
                        <KeyList keys={ keys } />
                    </tbody>
                </table>
            </div>
        </div>
    </div>

    }
}