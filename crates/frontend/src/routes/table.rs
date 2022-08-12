use yew::prelude::*;

// Tables headers can be derived from the table rows
// We need three types:
// - Table
// - Row
// - Cell

#[function_component(Cell)]
pub fn cell(props: &CellProps) -> Html {
    html! {
        <td>{ props.value.clone() }</td>
    }
}

#[derive(Properties, PartialEq)]
pub struct CellProps {
    pub heading: String,
    pub value: String,
}

#[function_component(Row)]
pub fn row(props: &RowProps) -> Html {
    html! {
        <tr>{ for props.children.iter() }</tr>
    }
}

#[derive(Properties, PartialEq)]
pub struct RowProps {
    pub children: ChildrenWithProps<Cell>,
}

#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    let headings: Vec<String> = props
        .children
        .iter()
        .next()
        .unwrap()
        .props
        .children
        .iter()
        .map(|cell| cell.props.heading.clone())
        .collect();

    html! {
        <div style="text-align: center">
            <h2>{ props.title.clone() }</h2>
                <table class="table table-striped table-hover table-bordered table-dark">
                    <thead class="table-dark">
                        <tr>
                            { for headings.iter().map(|heading| html!{<th>{ heading.clone() }</th>}) }
                        </tr>
                    </thead>
                    <tbody>
                        { for props.children.iter()}
                    </tbody>
                </table>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TableProps {
    pub title: String,
    pub children: ChildrenWithProps<Row>,
}
