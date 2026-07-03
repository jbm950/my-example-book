use std::net::SocketAddr;

use crate::app::bus_controller;

pub async fn run(server_addr: SocketAddr) {
    let controller_task = tokio::spawn(bus_controller(server_addr));
    let _ = controller_task.await;

    //_ = rt5_5r_interval.tick() => {
    //    let cmd_msg = CommandMessage {
    //        word: CmdWord::new(5, Subaddress { address: 5, tr: TxRx::R }, 1),
    //        data: PowerCommand::InjectFault(Fault::OverTemp).to_data_words(),
    //    };
    //}
}
