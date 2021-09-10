extern crate abode;

mod header;

use abode::init;
use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;

use abode::files::FileError;
use abode::files::FileState;
use abode::files::FileStatus;
use abode::network::Network;
use abode::server::Server;
use header::Header;

fn main() {
    println!("Your humble Abode.");

    let args = read_args();

    // Load networks
    let mut networks_status = init::get_file_status("");

    // Printout and start server
    if args.len() == 1 {
        printout_networks(&mut networks_status);
    }

    if args.len() > 1 {
        match args[1].as_str() {
            // Display network status
            "view" => printout_networks(&mut networks_status),
            // Add a new network to files, reload view
            // "create" => {
            //     if args.len() < 3 {
            //         println!("create command requires name: create [name]");
            //     } else {
            //         let fresh = Network::new(&args[2]);
            //         // Add network
            //         networks_status.persist_network_to_file(fresh);
            //     }
            // }
            "of" => {
                if args.len() < 3 {
                    println!("Whose abode do you want to crash?\nabode of serena\nTo see your own abode: abode of me");
                } else {
                    match args[2].as_str() {
                        "me" => print_header(),
                        _ => {
                            // bring in a user's abode
                            // Make request to another user:
                            // 1. setup TCP with peer
                            // 2. pull in header, and filetree
                            // 3. put file tree in memory (as a tree)
                            // 4. leave when finished
                        }
                    }
                }
            }
            // Request file
            "req" => {
                if args.len() < 3 {
                    println!("req command requires name: req [name]");
                } else {
                    Server::request_file(&args[2]);
                }
            }
            // "send" => send_file_to_all_devices(&mut networks_status),
            // Remove a network
            "close" => {}
            //TODO
            // "invite" => {
            //     if args.len() < 4 {
            //         println!("invite command requires net_id and device name: invite UUID name");
            //     }
            //     add_to_network(&mut networks_status, &args[2], &args[3]);
            // }
            _ => {
                println!("Commands available are 'view', 'create', 'req', and 'invite'");
            }
        }
    }
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
