use bincode::deserialize;
use crossbeam_channel::{Receiver, Sender};
use laminar::{Packet, SocketEvent};
use rustyhack_lib::message_handler::player_message::PlayerReply;

pub fn run(
    _sender: Sender<Packet>,
    receiver: Receiver<SocketEvent>,
    channel_sender: Sender<PlayerReply>,
) {
    loop {
        info!("Waiting for packet to be received.");
        if let Ok(event) = receiver.recv() {
            info!("Packet received. Processing...");
            match event {
                SocketEvent::Packet(packet) => {
                    let msg = packet.payload();
                    let address = packet.addr();
                    let player_reply =
                        deserialize::<PlayerReply>(msg).expect(&*String::from_utf8_lossy(msg));
                    info!("Received {:?} from {:?}", player_reply, address);

                    match player_reply {
                        PlayerReply::PlayerCreated => {
                            channel_sender
                                .send(PlayerReply::PlayerCreated)
                                .expect("Player created thread message didn't send.");
                        }
                        PlayerReply::AllMaps(message) => {
                            channel_sender
                                .send(PlayerReply::AllMaps(message))
                                .expect("All Maps thread message didn't send.");
                        }
                        PlayerReply::UpdatePosition(message) => {
                            channel_sender
                                .send(PlayerReply::UpdatePosition(message))
                                .expect("All Maps thread message didn't send.");
                        }
                        PlayerReply::UpdateOtherEntities(message) => {
                            channel_sender
                                .send(PlayerReply::UpdateOtherEntities(message))
                                .expect("All Maps thread message didn't send.");
                        }
                    }
                }
                SocketEvent::Connect(connect_event) => {
                    info!("Server connected at: {}", connect_event)
                }
                SocketEvent::Timeout(address) => {
                    info!("Server connection timed out: {}", address);
                }
                _ => {}
            }
        }
    }
}