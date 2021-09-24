use abode::network::Network;
use tui::list::List;

pub struct App {
    pub title: String,
    pub enhanced_graphics: bool,
    pub list: List,
    pub data: Vec<Network>,
}

enum ViewName {
    Networks,
    Devices,
}

impl App {
    pub fn new(title: &str, enhanced_graphics: bool, networks: Vec<Network>) -> App {
        App {
            title: title.to_string(),
            enhanced_graphics,
            data: networks,
        }
    }

    /// Index.0 is a code which says which list to displayy
    pub fn change_list(&mut self, view: ViewName, index: usize) {
        // If we are in the Network view
        match view {
            View::Network => {
                self.list = List::new(data);
            }
            View::Devices => {
                self.list = List::new(data[index]);
            }
        }
    }
}
