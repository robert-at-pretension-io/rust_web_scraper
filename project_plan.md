# Application 

## Interface
-> terminal menu-based application with utilities for this application
-> create rust models that map to sqlx database table
-> Once the sites are in the database, show a user interface that also does semantic searches on the descriptions

## Purpose
-> Creating a local, searchable database of content
-> Augment software projects with the requisite documentation in one place

## Function
-> Allow either loading URLS via file OR command line interface
-> Auto-aggregate urls for scraping via SERPAPI
-> Similar for search queries, can also pull these one by one from a text file (queries put in queries.txt)
-> Scrape the site via scrapingbee (urls from sites.txt) 
-> Process the website via async openai crate
-> Pull data to a file -- useful for pulling out documentation and including the docs in new files. This will allow for using the latest software library information in software projects


## Db Models
-> URL, Description, raw content, interesting details, vector of encodings for the content of the interesting details, uuids

## Structure
-> Async application using the tokio rust crate -- allowing for one application running both the web server AND terminal menu

## Flow
1. Provide a file containing a url per line (urls.txt)
    a. Once these are in the database -- delete the line from the file.
    b. For each url, check to see if the database contains that url already -- make sure to sanitize the url BEFORE checking that the url is in it
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