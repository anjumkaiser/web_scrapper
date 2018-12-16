use reqwest;
use select::document::Document;
use select::predicate::{Name, Class, Predicate};
use prettytable::{Table, Row, Cell};

fn main() {
    println!("Hello, world!");

    let url = "https://news.ycombinator.com";

    hacker_news(url);

}


fn hacker_news(url: &str) {
  let mut table = Table::new();

  let row = Row::new(vec![Cell::new("Sr."), Cell::new("Title"), Cell::new("Url")]);
  table.add_row(row);


  let resp = reqwest::get(url).unwrap();
  assert!(resp.status().is_success());

  let document = Document::from_read(resp).unwrap();

  for node in document.find(Class("athing")) {
    //println!("\n\n{:?}", node);

    let mut rank = node.find(Class("rank")).next().unwrap().text();
    rank = rank.trim().to_string();
    rank.truncate(rank.len()-1);
    //rank = rank[...rank.length()-1)].unwrap();
    //let irank = rank.parse::<i32>().unwrap();
    //println!("{:?}", rank);

    let story = node.find(Class("title").descendant(Name("a"))).next().unwrap().text();
    
    let url = node.find(Class("title").descendant(Name("a"))).next().unwrap().attr("href").unwrap();
    // println!("rank {:?} title {:?} url {:?}", rank, story, url);

    let row = Row::new(vec![Cell::new(&rank), Cell::new(&story), Cell::new(&url)]);
    table.add_row(row);
  }

  table.printstd();
}