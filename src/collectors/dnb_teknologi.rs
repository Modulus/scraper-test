use super::base::StringDataCollector;
use scraper::Html;
use scraper::Selector;
use scraper::node::Node;
use reqwest;

pub struct DnbTeknologiCollector {
    url: String
}

impl DnbTeknologiCollector {
    pub fn new() -> DnbTeknologiCollector {
        DnbTeknologiCollector {
            url: "https://www.dnb.no/sparing/fond/dnb-teknologi".into()
        }
    }

    fn extract_name(e:  &Node) -> Option<String> {
        let text = e.as_text()?.to_string();
        println!("e: {:?}", text);
        if !text.contains(":"){
            return Some(text.trim().into());
        }
        println!("Skipping value with : {:?}", text);
        return None;
    }

    pub async fn get_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let html: String = reqwest::get(url).await?.text().await?;
        return Ok(html);
    }
}

impl StringDataCollector for DnbTeknologiCollector {
    async fn collect(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let html: String = DnbTeknologiCollector::get_html(&self.url).await?;

        let document = Html::parse_document(&html);

        let ul_selector = Selector::parse("ul.dnb-ul")?;
        let li_selector = Selector::parse("li")?;
    
        let mut forvaltere : Vec<String> = Vec::new();
        for ul_element in document.select(&ul_selector) {
            if ul_element.has_children() && ul_element.children().count() >= 3 {
                for element in ul_element.select(&li_selector) {
                    if let Some(node) = element.first_child() {
                        if let Some(text) = DnbTeknologiCollector::extract_name(node.value()){
                            println!("Adding value: {:?}", text);
                            forvaltere.push(text);
                        }
                    }
                }
       
            }
        }
    
        Ok(forvaltere)
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_extract_name_has_valid_input_returns_name(){
        let html = r#"<li>Anders Tandberg-Johansen</li>"#;
        let document = Html::parse_document(html);
        let selector = Selector::parse("li").unwrap();
        let element = document.select(&selector).next().unwrap();
        let name = DnbTeknologiCollector::extract_name(element.first_child().unwrap().value()).unwrap();
        assert_eq!("Anders Tandberg-Johansen", name);
    }

    #[test]
    fn test_extract_name_has_invalid_input_returns_option_none(){
        let html = r#"<li>Forvaltere: </li>"#;
        let document = Html::parse_document(html);
        let selector = Selector::parse("li").unwrap();
        let element = document.select(&selector).next().unwrap();
        let name = DnbTeknologiCollector::extract_name(element.first_child().unwrap().value());
        assert!(name.is_none());
    }

    #[test]
    fn test_extract_name_has_empty_input_returns_option_none(){
        let html = r#"<li><p></p></li>"#;
        let document = Html::parse_document(html);
        let selector = Selector::parse("li").unwrap();
        let element = document.select(&selector).next().unwrap();
        let name = DnbTeknologiCollector::extract_name(element.first_child().unwrap().value());
        assert!(name.is_none());
    }


}

