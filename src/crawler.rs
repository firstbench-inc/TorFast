use std::io::Read;

pub async fn start(
    &mut self,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut success_count = 0;
    let mut failure_count = 0;

    while let Some(url) = self.to_visit.pop_front() {
        // Check if we need to stop
        if self.stop_flag.load(Ordering::Relaxed) {
            break;
        }

        println!("Processing URL: {}", url); // Debug statement

        match self.fetcher.fetch(&url).await {
            Ok(content) => {
                // Assuming fetch now directly returns the content
                println!("Successfully fetched URL: {}", url); // Debug statement

                let dom = match html5ever::parse_document(
                    rcdom::RcDom::default(),
                    Default::default(),
                )
                .from_utf8()
                .read_from(&mut content.as_bytes())
                {
                    Ok(dom) => dom,
                    Err(e) => {
                        println!("Error parsing HTML: {:?}", e);
                        failure_count += 1;
                        continue;
                    }
                };

                let mut parser = Parser::new();
                parser.parse_a_tags(dom.document.clone());
                let title = parser.get_title(dom.document.clone());

                let mut page_data = HashMap::new();
                page_data.insert("link".to_string(), url);
                page_data.insert("content".to_string(), content);
                if let Some(title_str) = title {
                    page_data.insert("title".to_string(), title_str);
                }

                match self.poster.post_url_data(&page_data).await {
                    Ok(_) => println!("Posted to Elasticsearch"),
                    Err(e) => {
                        println!(
                            "Failed to post to Elasticsearch: {:?}",
                            e
                        );
                        failure_count += 1;
                    }
                }

                self.to_visit.extend(parser.get_hrefs());

                success_count += 1;
            }
            Err(e) => {
                println!("Failed to fetch {}: {:?}", url, e);
                failure_count += 1;
            }
        }
    }

    println!("Successfully processed URLs: {}", success_count);
    println!("Failed to process URLs: {}", failure_count);

    Ok(())
}
