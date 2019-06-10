use gtk;
use gtk::prelude::*;
use gtk::{Button, HeaderBar, Orientation};

use crate::settings::Settings;
use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;

pub struct FontSettingsUi<'a> {
   settings: &'a Rc<RefCell<Settings>>,
}

impl<'a> FontSettingsUi<'a> {
   pub fn new(settings: &'a Rc<RefCell<Settings>>) -> Self {
      Self { settings }
   }

   pub fn show<T: IsA<gtk::Window>>(&mut self, parent: &T) {
      let dlg = gtk::Dialog::new_with_buttons(
         Some("Font Settings"),
         Some(parent),
         gtk::DialogFlags::DESTROY_WITH_PARENT,
         &[("Ok", gtk::ResponseType::Ok.into())],
      );

      dlg.set_default_size(800, 600);
      let content = dlg.get_content_area();

      //let header_bar = gtk::HeaderBar::new();
      //header_bar.set_title("Font Settings");
      //header_bar.set_show_close_button(true);
      //header_bar.show();
      //
      //
    let link_button = gtk::Label::new(None);
    link_button.set_markup(
        "# Temporary Changes",
    );
      content.pack_start(&link_button, true, true,10);

      //dlg.set_titlebar(&header_bar);

      //add_plug_btn.connect_clicked(clone!(dlg => move |_| {
      //show_add_plug_dlg(&dlg, &manager_ref, &plugs_panel);
      //}));

      //content.pack_start(&*pages, true, true, 0);
      content.show_all();

      let ok: i32 = gtk::ResponseType::Ok.into();
      if dlg.run() == ok {
         // Do nothing at the moment
      }

      dlg.destroy();
   }
}
