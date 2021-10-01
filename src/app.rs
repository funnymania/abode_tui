use crate::dialog::Dialog;
use crate::header::Header;
use abode::network::Network;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Text};
use tui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap};
use tui::Terminal;

use std::error::Error;

pub struct App<'a> {
    pub title: String,
    pub enhanced_graphics: bool,
    list: List<'a>,
    pub list_len: usize,
    pub data: Vec<Network>,
    pub list_state: ListState,
    pub view: View,
    dialog: Option<Dialog>,
}

pub enum View {
    Networks(Header),
    Devices(Header),
}

impl<'a> App<'a> {
    pub fn new(title: &str, enhanced_graphics: bool, networks: Vec<Network>) -> App {
        let mut ui_list = Vec::new();
        for network in networks.iter() {
            ui_list.push(ListItem::new(network.name().clone()));
        }

        ui_list.push(ListItem::new("+ add network"));
        ui_list.push(ListItem::new("- remove network"));

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        App {
            title: title.to_string(),
            enhanced_graphics,
            data: networks,
            list_len: ui_list.len(),
            list: List::new(ui_list),
            list_state,
            view: View::Networks(Header::new()),
            dialog: None,
        }
    }

    /// helper function to create a centered rect using up
    /// certain percentage of the available rect `r`
    pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage((100 - percent_y) / 2),
                    Constraint::Percentage(percent_y),
                    Constraint::Percentage((100 - percent_y) / 2),
                ]
                .as_ref(),
            )
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage((100 - percent_x) / 2),
                    Constraint::Percentage(percent_x),
                    Constraint::Percentage((100 - percent_x) / 2),
                ]
                .as_ref(),
            )
            .split(popup_layout[1])[1]
    }

    pub fn draw<B>(&mut self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>>
    where
        B: Backend,
    {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            match &self.view {
                View::Networks(header) | View::Devices(header) => {
                    let block = Paragraph::new(header.content()).block(
                        Block::default()
                            .title("Your Humble, Abode")
                            .borders(Borders::ALL)
                            .style(Style::default().fg(Color::Black).bg(Color::Magenta)),
                    );
                    f.render_widget(block, chunks[0]);
                }
            }

            // Fill with networks
            let list = self
                .copy_list()
                .block(Block::default().title("Loot").borders(Borders::ALL))
                .style(Style::default().fg(Color::Black).bg(Color::Magenta))
                .highlight_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::ITALIC | Modifier::BOLD),
                )
                .highlight_symbol(">>");
            f.render_stateful_widget(list, chunks[1], &mut self.list_state);

            let demo_txt = Text::from(
                "\n.-. .-. .  . .-.
                |  )|-  |\\/| | |
                `-' `-' '  ' `-' ",
            );

            let demo_paragraph = Paragraph::new(demo_txt)
                .style(
                    Style::default()
                        .fg(Color::White)
                        .bg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                )
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
            f.render_widget(demo_paragraph, chunks[2]);

            //Is there a popup to render?
            match &self.dialog {
                Some(dialog) => {
                    let block = Paragraph::new(format!("{}", dialog.content)).block(
                        Block::default()
                            .title(format!("{}", dialog.title))
                            .borders(Borders::ALL),
                    );

                    let area = App::centered_rect(60, 20, f.size());
                    f.render_widget(Clear, area); //this clears out the background
                    f.render_widget(block, area);
                }
                None => {}
            }
        })?;

        Ok(())
    }

    /// Index.0 is a code which says which list to displayy
    /// Change list to represent what is expected given 'view'
    pub fn change_list(&mut self, index: usize) {
        match self.view {
            View::Networks(_) => {
                //RemoveDevice dialog
                if index == self.data.len() - 1 {
                }
                //AddDevice dialog
                else if index == self.data.len() - 2 {
                }
                //List networks
                else {
                    let mut ui_list = Vec::new();
                    for network in self.data.iter() {
                        ui_list.push(ListItem::new(network.name().clone()));
                    }

                    // Last items in list are for adding and removing
                    ui_list.push(ListItem::new("+ add network"));
                    ui_list.push(ListItem::new("- remove network"));
                    self.list = List::new(ui_list);
                }
            }
            View::Devices(_) => {
                //RemoveNetwork dialog
                if index == self.data.len() - 1 {
                }
                //AddNetwork dialog
                else if index == self.data.len() - 2 {
                }
                //List devices
                else {
                    let mut ui_list = Vec::new();
                    for device in self.data[index].members() {
                        ui_list.push(ListItem::new(device.name().clone()));
                    }

                    // Last items in list are for adding and removing
                    ui_list.push(ListItem::new("+ add device"));
                    ui_list.push(ListItem::new("- remove device"));
                    self.list = List::new(ui_list);
                }
            }
        }
    }

    pub fn copy_list(&self) -> List<'a> {
        self.list.clone()
    }

    pub fn move_down(&mut self) {
        match self.list_state.selected() {
            Some(place) => {
                if place != self.list_len - 1 {
                    self.list_state.select(Some(place + 1));
                }
            }
            None => (),
        }
    }

    pub fn move_up(&mut self) {
        match self.list_state.selected() {
            Some(place) => {
                if place != 0 {
                    self.list_state.select(Some(place - 1));
                }
            }
            None => (),
        }
    }

    pub fn move_left(&mut self) {
        match self.list_state.selected() {
            Some(thing) => {
                if let View::Devices(header) = &self.view {
                    self.view = View::Networks(header.clone());
                    self.change_list(thing);
                }
            }
            None => (),
        }
    }

    pub fn move_right(&mut self) {
        match self.list_state.selected() {
            Some(thing) => {
                if let View::Networks(header) = &self.view {
                    self.view = View::Devices(header.clone());
                    self.change_list(thing);
                }
            }
            None => (),
        }
    }
}
