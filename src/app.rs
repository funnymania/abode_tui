use abode::network::Network;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Text};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use tui::Terminal;

use std::error::Error;

pub struct App<'a> {
    pub title: String,
    pub enhanced_graphics: bool,
    list: List<'a>,
    pub list_len: usize,
    pub data: Vec<Network>,
    pub cursor: usize,
    pub view: View,
}

#[derive(PartialEq)]
pub enum View {
    Networks,
    Devices,
}

impl<'a> App<'a> {
    pub fn new(title: &str, enhanced_graphics: bool, networks: Vec<Network>) -> App {
        let mut ui_list = Vec::new();
        for network in networks.iter() {
            ui_list.push(ListItem::new(network.name().clone()));
        }

        App {
            title: title.to_string(),
            enhanced_graphics,
            data: networks,
            list_len: ui_list.len(),
            list: List::new(ui_list),
            cursor: 0,
            view: View::Networks,
        }
    }

    pub fn draw<B>(&self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>>
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
            let block = Block::default()
                .title("Your Humble, Abode")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Black).bg(Color::Magenta));
            f.render_widget(block, chunks[0]);

            // Fill with networks
            let list = self
                .copy_list()
                .block(Block::default().title("Loot").borders(Borders::ALL))
                .style(Style::default().fg(Color::LightCyan).bg(Color::Magenta))
                .highlight_style(
                    Style::default()
                        .fg(Color::Black)
                        .add_modifier(Modifier::ITALIC | Modifier::BOLD),
                )
                .highlight_symbol(">>");
            f.render_widget(list, chunks[1]);

            let demo_txt = Text::from(
                "\n.-. .-. .  . .-.
                |  )|-  |\\/| | |
                `-' `-' '  ` `-' ",
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
        })?;

        Ok(())
    }

    /// Index.0 is a code which says which list to displayy
    /// Change list to represent what is expected given 'view'
    pub fn change_list(&mut self, view: View, index: usize) {
        match view {
            View::Networks => {
                let mut ui_list = Vec::new();
                for network in self.data.iter() {
                    ui_list.push(ListItem::new(network.name().clone()));
                }
                self.list = List::new(ui_list);
            }
            View::Devices => {
                let mut ui_list = Vec::new();
                for device in self.data[index].members() {
                    ui_list.push(ListItem::new(device.name().clone()));
                }
                self.list = List::new(ui_list);
            }
        }
    }

    pub fn copy_list(&self) -> List<'a> {
        self.list.clone()
    }

    pub fn move_down(&mut self) {
        if self.cursor != self.list_len - 1 {
            self.cursor += 1
        }
    }

    pub fn move_up(&mut self) {
        if self.cursor != 0 {
            self.cursor -= 1
        }
    }

    pub fn move_left(&mut self) {
        if self.view != View::Networks {
            self.view = View::Networks;
            self.change_list(View::Networks, self.cursor);
        }
    }

    pub fn move_right(&mut self) {
        if self.view != View::Devices {
            self.view = View::Devices;
            self.change_list(View::Devices, self.cursor);
        }
    }
}
