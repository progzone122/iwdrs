use iwdrs::known_netowk::KnownNetwork;
use iwdrs::session::Session;
use iwdrs::station::Station;
#[tokio::main]
async fn main() {
    let session: Session = Session::new().await.expect("Failed to create session");
    let station: Station = session.station().expect("Failed to get station");

    station.scan().await.expect("Failed to scan networks"); // scan networks

    let known_networks: Vec<KnownNetwork> = session.known_networks().await; // get all available networks
    for network in known_networks {
        println!("{:#?}", network.name().await.unwrap());
    }
}