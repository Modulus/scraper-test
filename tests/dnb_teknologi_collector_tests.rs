
use scraper_test::collectors::dnb_teknologi::DnbTeknologiCollector;
use scraper_test::collectors::base::StringDataCollector;


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
async fn test_get_html_returns_html(){
    let html = DnbTeknologiCollector::get_html("https://www.dnb.no/sparing/fond/dnb-teknologi").await.unwrap();

    let forvaltere = get_excpected_forvaltere();

    assert!(html.len() > 0);
    assert!(html.contains("DNB Teknologi"));
    assert!(html.contains("Forvalterteamet i DNB Teknologi"));

    assert!(html.contains(forvaltere[0].as_str()));
    assert!(html.contains(forvaltere[1].as_str()));
    assert!(html.contains(forvaltere[2].as_str()));
    assert!(html.contains(forvaltere[3].as_str()));
}

#[tokio::test]
async fn test_get_forvaltere_has_four_of_them(){
    let collector = DnbTeknologiCollector::new();
    let forvaltere = collector.collect().await.unwrap();
    assert_eq!(4, forvaltere.len());
}

#[tokio::test]
async fn test_get_dnb_teknologi_forvaltere_matches_expected_values() {
    let expected = get_excpected_forvaltere();
    let collector = DnbTeknologiCollector::new();
    let actual = collector.collect().await.unwrap();
    assert_eq!(expected, actual);
}