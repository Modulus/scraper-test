use scraper::Html;
use scraper::Selector;
use scraper::node::Node;


#[tokio::main]
async fn main() {
    let forvaltere = get_forvaltere().await;
    println!("{:?}", forvaltere);   

}


async fn get_forvaltere() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let html = reqwest::get("https://www.dnb.no/sparing/fond/dnb-teknologi").await?.text().await?;

    let document = Html::parse_document(&html);

    println!("Forvaltere");
    let ul_selector = Selector::parse("ul.dnb-ul")?;
    let li_selector = Selector::parse("li")?;

    let mut forvaltere : Vec<String> = Vec::new();
    for ul_element in document.select(&ul_selector) {
        if ul_element.has_children() && ul_element.children().count() >= 3 {
            for element in ul_element.select(&li_selector) {
                if let Some(node) = element.first_child() {
                    if let Some(text) = extract_name(node.value()){
                        println!("Adding value: {:?}", text);
                        forvaltere.push(text);
                    }
                }
            }
   
        }
    }
    
    Ok(forvaltere)
}

fn extract_name(e:  &Node) -> Option<String> {
    let text = e.as_text().unwrap().to_string();
    println!("e: {:?}", text);
    if !text.contains(":"){
        return Some(text.trim().into());
    }
    println!("Skipping value with : {:?}", text);
    return None;
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
        let actual = get_forvaltere().await.unwrap();
        assert_eq!(expected, actual);
    }
}



