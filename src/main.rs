use scraper::Html;
use scraper::Selector;

#[tokio::main]
async fn main() {
    let forvaltere = get_forvaltere().await;
    println!("{:?}", forvaltere);   

}


async fn get_forvaltere() -> Vec<String> {
    let html = reqwest::get("https://www.dnb.no/sparing/fond/dnb-teknologi").await.unwrap().text().await.unwrap();

    let document = Html::parse_document(&html);
    // print!("document: {:?}", document);
    // print!("=====================");

    // let h2_selector = Selector::parse("h2.dnb-heading").unwrap();

    println!("Forvaltere");
    let ul_selector = Selector::parse("ul.dnb-ul").unwrap();
    let li_selector = Selector::parse("li").unwrap();

    let mut forvaltere : Vec<String> = Vec::new();
    for ul_element in document.select(&ul_selector) {
        if ul_element.has_children() && ul_element.children().count() >= 4 {
            for elelm in ul_element.select(&li_selector) {
                let value = elelm.first_child().unwrap().value();
                println!("value: {:?}", value.as_text());
                forvaltere.push(value.as_text().unwrap().to_string().trim().to_string());
            }
        }
   
    }


    forvaltere
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    fn get_excpected_forvaltere() -> Vec<String>{
        let forvaltere = vec![
            "Anders Tandberg-Johansen".into(),
            "Sverre Bergland".into(),
            "Erling Thune".into(),
            "Erling Haugan Kise".into()
        ];
        return forvaltere;
    }

    #[tokio::test]
    async fn test_get_dnb_teknologi_forvaltere_matches_expected_values() {
        let expected = get_excpected_forvaltere();
        let actual = get_forvaltere().await;
        assert_eq!(expected, actual);
    }
}



