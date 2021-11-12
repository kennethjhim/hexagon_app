//! # A more advanced application Create-Read-Update-Delete

use druid::{AppLauncher, WindowDesc, Widget, PlatformError, Data, Lens, Size, WidgetExt, Color, LensExt};
use druid::widget::{Label, Flex, Align, TextBox, Button, Scroll, List, CrossAxisAlignment, MainAxisAlignment, Either};
use druid::lens::{self};
use druid::im::{Vector, vector};

mod fetch_all_pokemons;
mod create_pokemon;
use crate::repositories::pokemon::Repository;
use crate::gui::fetch_all_pokemons::{Error, Response};
use std::sync::Arc;

const WINDOW_TITLE: &str = "CRUD";
const WINDOW_SIZE: Size = Size::new(500., 350.);
const WINDOW_SIZE_MIN: Size = Size::new(400., 250.);
const PADDING: f64 = 8.;

pub fn main(repo: Arc<dyn Repository>) {
    let data = AppData::new(repo);

    let window = WindowDesc::new(build_ui())
        .window_size(WINDOW_SIZE)
        .with_min_size(WINDOW_SIZE_MIN)
        .title(WINDOW_TITLE);

    match AppLauncher::with_window(window)
        .launch(data) {
        Ok(_) => (),
        _ => (),
    };
}

enum Status {
    Ok,
    BadRequest,
    NotFound,
    Conflict,
    InternalServerError,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Data)]
enum PokemonType {
    Electric,
    Fire,
}

// this is the Response with index struct
#[derive(Clone, Data, Lens)]
struct ListItem {
    index: usize,
    item: Response,
}

impl ListItem {
    fn new(i: usize, s: Response) -> Self {
        Self {
            index: i,
            item: s,
        }
    }
}


#[derive(Clone, Data, Lens)]
struct AppData{
    prefix: String,
    number: String,
    name: String,
    types: String,
    list: Vector<ListItem>,
    current: Option<usize>  // selected list item
}

impl AppData {
    fn new(repo: Arc<dyn Repository>) -> Self {

        let list = match fetch_all_pokemons::execute(repo.clone()) {
            Ok(pokemon_list) => pokemon_list.into_iter()
                                .enumerate()
                                .map(|(i, s)| ListItem::new(i, s))
                                .collect(),
            Err(e) => Vector::from(vec![]),
        };

        let last_num = list.last().unwrap().item.number.clone() + 1;
        let last_num_str = last_num.to_string();

        Self {
            prefix: "".into(),
            number: last_num_str,
            name: "".into(),
            types: "".into(),
            list,
            current: None
        }
    }

    fn filter(&self) -> Vector<ListItem> {
        let f = self.prefix.to_lowercase();
        self.list
            .clone()
            // .unwrap()
            .into_iter()
            .filter(|s| s.item.name.to_lowercase().contains(f.as_str()))
            .collect()
    }
}

// fn reindex(v: &Vector<ListItem>) -> Vector<ListItem>{
//     let v2 = v.clone();
//     v2
//         .into_iter()
//         .enumerate()
//         .map(|(i, s)| ListItem::new(i, s.item))
//         .collect::<Vector<ListItem>>()
// }

fn build_ui() -> impl Widget<AppData> {

    // HEADER
    let head = Align::left(Flex::row()
        .with_child(Label::new("Filter Prefix:  "))
        .with_child(TextBox::new()
            .lens(AppData::prefix)
            // .controller(FilterController)
        )).fix_height(30.);

    // BODY
    let right_1 = Flex::row()
        .with_child(Label::new("Number:  "))
        .with_child(TextBox::new().lens(AppData::number));

    let right_2 = Flex::row()
        .with_child(Label::new("Name:  "))
        .with_child(TextBox::new().lens(AppData::name));

    let right_3 = Flex::row()
        .with_child(Label::new("Type:  "))
        .with_child(TextBox::new().lens(AppData::types));

    let right = Flex::column()
        .with_child(right_1)
        .with_spacer(PADDING)
        .with_child(right_2)
        .with_spacer(PADDING)
        .with_child(right_3)
        .cross_axis_alignment(CrossAxisAlignment::End)
        .padding(8.0);

    let list = Scroll::new(List::new(|| {
        new_item()
    }))
        .vertical()
        .lens(lens::Identity.map(
            // Expose shared data with children data
            // Default: 'data.list.clone()' in place of data.filter()
        |data: &AppData| (data.current, data.filter()),
        |data: &mut AppData, (current, _list)| {
            data.current = current;
        }))
        .expand_width();

    let left = Flex::column()
        .with_flex_child(list, 1.)
        .expand()
        .padding(8.0)
        .background(Color::grey(0.4))
        .border(Color::grey(0.6), 2.0);

    let main_body = Flex::row()
        .main_axis_alignment(MainAxisAlignment::Start)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_flex_child(left, 1.)
        .with_spacer(PADDING)
        .with_child(right)
        .expand()
        .padding(PADDING)
        .border(Color::grey(0.2), 2.0)
        .rounded(5.0);

    // FOOTER
    // let btn_create = Button::new("Create")
    //     .on_click(|_, data: &mut AppData, _| {
    //         // Just slap it onto the end
    //         let s = format!("{}, {}", data.surname, data.name);
    //         let new = ListItem::new(data.list.len(), s);
    //         data.list.append(vector![new])
    //     });
    //
    // let btn_update = Button::new("Update")
    //     .on_click(|_, data: &mut AppData, _| {
    //         if let Some(i) = data.current {
    //             let new = format!("{}, {}", data.surname, data.name);
    //             data.list[i].item = new
    //         }
    //     });
    //
    // let btn_delete = Button::new("Delete")
    //     .on_click(|_, data: &mut AppData, _| {
    //         if let Some(i) = data.current {
    //             //remove the item
    //             data.list.remove(i);
    //             data.list = reindex(&data.list);
    //             // set the current selection to nothing (could also be nearest element etc.)
    //             data.current = None;
    //         }
    //     });
    //
    // let foot = Align::left(Flex::row()
    //     .with_child(btn_create)
    //     .with_spacer(PADDING)
    //     .with_child(btn_update)
    //     .with_spacer(PADDING)
    //     .with_child(btn_delete));

    // ROOT
    Flex::column()
        .with_child(head)
        .with_spacer(PADDING)
        .with_flex_child(main_body, 1.)
        .with_spacer(PADDING)
        // .with_child(foot)
        .padding(PADDING * 2.)
}

// create the list item widget
fn new_item() -> impl Widget<(Option<usize>, ListItem)> {
    Either::new(|data: &(Option<usize>, ListItem), _:&_| {
        if data.0.is_some() {
            data.0.unwrap() == data.1.index
        } else {
            false
        }},
    // TODO: Generalise this?
    Label::new(|data: &(Option<usize>, ListItem), _:&_|{
        data.1.item.name.to_string()
    })
        //format the true branch with some background colour
        .background(Color::rgba(0.2, 0.2, 0.6, 0.5))
        .expand_width(),
        // do not format the false branch
        Label::new(|data: &(Option<usize>, ListItem), _:&_| {
        data.1.item.name.to_string()
    }))
        .on_click(|_, data, _| {
        data.0 = Some(data.1.index);
    })
}