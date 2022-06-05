use {
  askama::Template,
  clap::Parser as _,
  pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag},
  std::fs,
  std::path::PathBuf,
};

#[derive(Template)]
#[template(path = "index.html", escape = "none")]
struct Index {
  title: Option<String>,
  slides: Vec<String>,
}

#[derive(clap::Parser)]
struct Args {
  input: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Args::parse();

  let input = fs::read_to_string(args.input)?;

  let mut markdown = Vec::new();

  for event in Parser::new_ext(&input, Options::all()) {
    if let Event::Rule = event {
      markdown.push(Vec::new());
      continue;
    }
    if markdown.is_empty() {
      markdown.push(Vec::new());
    }
    markdown.last_mut().unwrap().push(event);
  }

  let title = {
    if let Some(markdown) = markdown.first() {
      let title_markdown = markdown
        .iter()
        .skip_while(|event| !matches!(event, Event::Start(Tag::Heading(HeadingLevel::H1, ..))))
        .skip(1)
        .take_while(|event| !matches!(event, Event::End(Tag::Heading(HeadingLevel::H1, ..))))
        .cloned()
        .collect::<Vec<Event>>();

      if title_markdown.is_empty() {
        None
      } else {
        let mut title = String::new();
        pulldown_cmark::html::push_html(&mut title, title_markdown.into_iter());
        Some(title)
      }
    } else {
      None
    }
  };

  let mut slides = Vec::new();
  for markdown in markdown {
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, markdown.into_iter());
    slides.push(html.trim().into());
  }

  let index = Index { title, slides };

  println!("{}", index.render().unwrap());

  Ok(())
}
