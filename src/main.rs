extern crate gio;
extern crate gtk;

use glib::clone;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    ApplicationWindow, CellRendererText, Label, TreeStore, TreeIter, ListStore, Orientation, TreeView, TreeViewColumn,
    WindowPosition,
};

mod libs;
use std::env::args;

use libs::node::Node;
use libs::json_reader::JsonReader;
use std::fs::read;

fn create_tree(parent: &TreeIter, childs: Vec<Node>, model: &TreeStore)
{
    for item in childs{
        let iter = model.insert_with_values(Some(&parent), None, &[0], &[&item.value]);
        create_tree(&iter, item.node, model);
    }
}

fn create_and_fill_model(path: &std::path::Path) -> TreeStore {
    // Creation of a model with two rows.
    let model = TreeStore::new(&[String::static_type()]);

    let reader = JsonReader::new();
    let data = reader.read_file(path);

    // Filling up the tree view.
    for item in data{
        let iter = model.insert_with_values(None, None, &[0], &[&item.value]);
        create_tree(&iter, item.node, &model);
    }
    //let iter2 = model.insert_with_values(Some(&iter), None, &[0, 1], &[&2u32, &entries[1]]);
    model
}

fn append_column(tree: &TreeView, id: i32) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

    column.pack_start(&cell, true);
    // Association of the view's column with the model's `id` column.
    column.add_attribute(&cell, "text", id);
    tree.append_column(&column);
}

fn create_and_setup_view() -> TreeView {
    // Creating the tree view.
    let tree = TreeView::new();

    tree.set_headers_visible(false);
    // Creating the two columns inside the view.
    append_column(&tree, 0);
    append_column(&tree, 1);
    tree
}

fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);

    window.set_title("Task2");
    window.set_position(WindowPosition::Center);

    // Creating a vertical layout to place both tree view and label in the window.
    let vertical_layout = gtk::Box::new(Orientation::Vertical, 0);
    let read_button = gtk::Button::new_with_label(&"Read");

    read_button.connect_clicked(clone!(@weak read_button, @weak vertical_layout, @weak window => move |_| {
        let file_chooser = gtk::FileChooserDialog::new(
            Some("Open File"),
            Some(&window),
            gtk::FileChooserAction::Open,
        );
        file_chooser.add_buttons(&[
            ("Open", gtk::ResponseType::Ok),
            ("Cancel", gtk::ResponseType::Cancel),
        ]);

        if file_chooser.run() == gtk::ResponseType::Ok
        {
            let filename = file_chooser.get_filename().expect("Couldn't get filename");

            // Creation of the label.
            let label = Label::new(None);

            let tree = create_and_setup_view();

            let model = create_and_fill_model(filename.as_path());
            // Setting the model into the view.
            tree.set_model(Some(&model));
            // Adding the view to the layout.
            vertical_layout.add(&tree);
            // Same goes for the label.
            vertical_layout.add(&label);
        }

        file_chooser.destroy();
        window.show_all();
    }));

    vertical_layout.pack_start(&read_button, false, false, 0u32);
    window.add(&vertical_layout);

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.simple_treeview"),
        Default::default(),
    )
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}