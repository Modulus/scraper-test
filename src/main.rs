pub mod collectors;


use collectors::dnb_teknologi::DnbTeknologiCollector;
use collectors::base::StringDataCollector;


#[tokio::main]
async fn main() {
    let collector = DnbTeknologiCollector::new();
    let forvaltere = collector.collect().await.unwrap();
    println!("{:?}", forvaltere);   

}
