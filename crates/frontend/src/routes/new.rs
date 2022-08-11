use yew::prelude::*;

use crate::routes::keys::Key;

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

#[function_component(New)]
pub fn new() -> Html {
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
        <div class="container text-light my-3" style="max-width: 600px;">
            <div class="row justify-content-center">
                <div class="">
                    <h1>{ "New Key" }</h1>
                        <form action="" method="post" class="form" role="form">
                            <div class="form-group  required"><label class="control-label" for="name">{ "Key Name" }</label>
                                <input class="form-control" id="name" name="name" required=true type="text" value="" />
                            </div>

                            <div class="form-group "><label class="control-label" for="description">{ "Description" }</label>
                                <input class="form-control" id="description" name="description" type="text" value="" />
                            </div>

                            <input class="btn btn-primary" id="submit" name="submit" type="submit" value="Add Key" />
                            <input class="btn btn-secondary" formnovalidate=true id="cancel" name="cancel" type="submit" value="Cancel" />
                        </form>
                    </div>
                </div>
            </div>
    }
}
