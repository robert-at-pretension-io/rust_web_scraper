# Application 

## Interface

The application provides two user interfaces:

1. Terminal Interface
   - Menu-based CLI application built with ratatui
   - Database management utilities
   - Command-line search and query capabilities

2. Database Integration
   - Rust models mapped to SQLx database tables
   - Efficient storage and retrieval of scraped content

3. Search Interface
   - Semantic search functionality for stored content
   - Natural language querying of document descriptions
   - Interactive results filtering and selection

## Purpose
-> Creating a local, searchable database of content
-> Augment software projects with the requisite documentation in one place

## Function
-> Allow either loading URLS via file OR command line interface
-> Auto-aggregate urls for scraping via SERPAPI -- the queries sent to SERPAPI will be derived from an openai invocation
-> Similar for search queries, can also pull these one by one from a text file (queries put in queries.txt)
-> Scrape the site via scrapingbee (urls from sites.txt) 
-> Process the website via async openai crate
-> Pull data to a file -- useful for pulling out documentation and including the docs in new files. This will allow for using the latest software library information in software projects


## Db Models
-> URL, Description, raw content, interesting details, vector of encodings for the content (like OpenAI's text embeddings) of the interesting details, uuids, query that lead to the result

## Structure
-> Async application using the tokio rust crate -- allowing for one application running both the web server AND terminal menu. The purpose of the website is to allow the user to peruse the resulting aggregated found documents. The user should also be able to use the openai endpoint to ask questions about selected documents from the frontend.

## Flow
1. Provide a file containing a url per line (urls.txt)
    a. Once these are in the database -- delete the line from the file.
    b. For each url, check to see if the database contains that url already -- make sure to sanitize the url BEFORE checking that the url is in it -- removing http/https, removing trailing slashes, making the same case and any other necessary standardizations
2. Alternatively, allow one query per line (queries.txt).
    a. async_openai will be used to "select" the urls from the search result that it thinks would be good to have in its database
2. On the website, allow searching by semantics based on user queries, then allow for summarizing or asking questions based on those selected sources.


## API services used:
-> www.scrapingbee.com
-> https://serpapi.com/dashboard 

## Rust crates used
-> SQLX
-> ratatui
-> tokio
-> async-openai
