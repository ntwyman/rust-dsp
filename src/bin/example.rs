extern crate gtk_graph;
extern crate dspr;

use rgtk::gtk;

fn main() {
	gtk::init();
	println!("Major: {}, Minor: {}", gtk::get_major_version(), gtk::get_minor_version());
}