use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
enum RcvResult {
    Success,
    Failure,
}

struct SendData {
    name: String,
    resp: oneshot::Sender<RcvResult>,
}

async fn send_data(tx: mpsc::Sender<SendData>, name: &str) {
    let (resp_tx, resp_rx) = oneshot::channel();

    tx.send(SendData {
        name: name.to_string(),
        resp: resp_tx,
    })
    .await
    .unwrap();

    let res = resp_rx.await.unwrap();
    println!("{name} got {:?}", res);
}

#[tokio::main]
async fn main() {
    let capacity = 32;
    let (tx, mut rx) = mpsc::channel(capacity);
    let tx2 = tx.clone();

    tokio::spawn(send_data(tx2, "Cool Guy"));
    tokio::spawn(send_data(tx, "Bad Guy"));

    while let Some(SendData { name, resp }) = rx.recv().await {
        println!("Got name {:?} from a task", name);

        match name.as_str() {
            "Cool Guy" => {
                let _ = resp.send(RcvResult::Success);
            }
            "Bad Guy" => {
                let _ = resp.send(RcvResult::Failure);
            }
            _ => {}
        }
    }
}
