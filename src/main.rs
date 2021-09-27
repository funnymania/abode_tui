extern crate abode;
extern crate tui;

mod app;
// mod extensions;
mod header;

use abode::init;
use keycode::{KeyMap, KeyMappingId};
use std::{
    env,
    error::Error,
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::Terminal;

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode};

use abode::files::FileError;
use abode::files::FileStatus;
use abode::server::Server;
use header::Header;

use crate::app::App;

enum Event<I> {
    Input(I),
    Tick,
}

// ISSUE: How do we pull in Keycodes? (I suggest what we are using for arranges)
// ISSUE: Event handling
fn main() -> Result<(), Box<dyn Error>> {
    println!("Your humble Abode.");

    let args = read_args();

    // Time between ticks
    let tick_rate: u64 = 250;
    // Whether unicode symbols are used
    let enhanced_graphics = true;
    // Do we quit?
    let should_quit = false;

    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Clear screen.
    terminal.clear()?;

    let (tx, rx) = mpsc::channel();

    // Load networks
    let mut networks_status = init::get_file_status("");
    let mut all_networks = Vec::new();
    match networks_status {
        FileStatus::Ok(nets) => {
            all_networks = Vec::from(nets);
        }
        FileStatus::Err(fe) => match fe {
            FileError::Other(msg) => panic!("{:?}", msg),
        },
    };

    let mut app = App::new("Abode Demo", enhanced_graphics, all_networks);

    // Don't understand rn
    let tick_rate = Duration::from_millis(tick_rate);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            // poll for tick rate duration, if no events, sent tick event.
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if event::poll(timeout).unwrap() {
                if let CEvent::Key(key_event) = event::read().unwrap() {
                    tx.send(Event::Input(key_event)).unwrap();
                }
            }
            if last_tick.elapsed() >= tick_rate {
                tx.send(Event::Tick).unwrap();
                last_tick = Instant::now();
            }
        }
    });

    // Static screen
    loop {
        app.draw(&mut terminal);

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('j') => {
                    app.move_down()
                    // disable_raw_mode()?;
                    // execute!(
                    //     terminal.backend_mut(),
                    //     LeaveAlternateScreen,
                    //     DisableMouseCapture
                    // )?;
                    // terminal.show_cursor()?;
                    // break;
                }
                // KeyCode::Char(c) => app.on_key(c),
                // KeyCode::Left => app.on_left(),
                // KeyCode::Up => app.on_up(),
                // KeyCode::Right => app.on_right(),
                // KeyCode::Down => app.on_down(),
                _ => {}
            },
            Event::Tick => {}
        }

        if should_quit {
            break;
        }
    }

    // Printout and start server
    if args.len() == 1 {
        // printout_networks(&mut networks_status);
    }

    // if args.len() > 1 {
    //     match args[1].as_str() {
    //         // Display network status
    //         "view" => printout_networks(&mut networks_status),
    //         // Add a new network to files, reload view
    //         // "create" => {
    //         //     if args.len() < 3 {
    //         //         println!("create command requires name: create [name]");
    //         //     } else {
    //         //         let fresh = Network::new(&args[2]);
    //         //         // Add network
    //         //         networks_status.persist_network_to_file(fresh);
    //         //     }
    //         // }
    //         "of" => {
    //             if args.len() < 3 {
    //                 println!("Whose abode do you want to crash?\nabode of serena\nTo see your own abode: abode of me");
    //             } else {
    //                 match args[2].as_str() {
    //                     "me" => print_header(),
    //                     _ => {
    //                         // bring in a user's abode
    //                         // Make request to another user:
    //                         // 1. setup TCP with peer
    //                         // 2. pull in header, and filetree
    //                         // 3. put file tree in memory (as a tree)
    //                         // 4. leave when finished
    //                     }
    //                 }
    //             }
    //         }
    //         // Request file
    //         "req" => {
    //             if args.len() < 3 {
    //                 println!("req command requires name: req [name]");
    //             } else {
    //                 Server::request_file(&args[2]);
    //             }
    //         }
    //         // "send" => send_file_to_all_devices(&mut networks_status),
    //         // Remove a network
    //         "close" => {}
    //         //TODO
    //         // "invite" => {
    //         //     if args.len() < 4 {
    //         //         println!("invite command requires net_id and device name: invite UUID name");
    //         //     }
    //         //     add_to_network(&mut networks_status, &args[2], &args[3]);
    //         // }
    //         _ => {
    //             println!("Commands available are 'view', 'create', 'req', and 'invite'");
    //         }
    //     }
    // }

    Ok(())
}

// fn send_file_to_all_devices(networks_status: &mut FileState) {
//     // file reference to 'msg.txt'
//     let mut file = fs::OpenOptions::new()
//         .read(true)
//         .write(true)
//         .open("/Users/xiao/network/src/msg.txt")
//         .unwrap();
//
//     // send to all network devices
//     let mut content = [0; 256];
//
//     file.read(&mut content).unwrap();
//
//     for network in networks_status.networks().iter_mut() {
//         println!("{}", network.name());
//         for device in network.members() {
//             // send content to device
//             match device.send_to_device(&content) {
//                 Err(e) => println!("{}", e),
//                 _ => println!("Sent to device {}.", device.id()),
//             }
//         }
//     }
// }
//
/// TODO: Conform to Filestatus
// fn add_to_network(networks_status: &mut FileState, net_id: &str, device_name: &str) {
//     // add to file
//     match networks_status.persist_device_to_file(net_id, device_name) {
//         Ok(device) => {
//             for network in &mut networks_status.networks().iter_mut() {
//                 if network.id().as_str() == net_id {
//                     network.add_member(device);
//                     println!("Device added.");
//                     return;
//                 }
//             }
//         }
//         Err(msg) => println!("{}", msg),
//     }
//
//     println!("Network not found");
// }

fn printout_networks(networks_status: &mut FileStatus) {
    // Print to standard out
    match networks_status {
        FileStatus::Ok(nets) => {
            for network in nets.iter() {
                println!("Network");
                println!("{} - {}", network.id(), network.name());
                println!("Peer Devices");
                if network.members().len() == 0 {
                    println!("None");
                }

                for device in network.members() {
                    println!("{}", device.id());
                }
            }
        }
        FileStatus::Err(fe) => match fe {
            FileError::Other(msg) => panic!("{:?}", msg),
        },
    }
}

fn print_header() {
    let header = Header::new();
    print!("\n~*~\n{}\n~*~\n\nLeave", header.content());
}

fn read_args() -> Vec<String> {
    env::args().collect()
}
