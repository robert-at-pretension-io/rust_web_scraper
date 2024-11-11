# Documentation Crawler and Processor

A powerful Rust-based tool for crawling, scraping, and processing technical documentation using AI. This tool helps automate the collection and formatting of documentation from various sources.

## Features

- 🔍 Search-based URL discovery using SerpAPI
- 🕷️ Intelligent web crawling with depth control
- 🤖 AI-powered content processing using OpenAI's GPT models
- 📝 Markdown conversion and formatting
- 🔄 Documentation refresh capabilities
- 📊 Metadata tracking and management
- 🚫 URL filtering and selection
- 💾 SQLite database for content storage
- 🔄 Progress tracking with processed URLs
- ⚡ Async processing with status indicators

## Prerequisites

Before setting up this project, ensure you have:

- Rust (latest stable version)
- SQLite3
- API keys for:
  - OpenAI
  - SerpAPI
  - ScrapingBee

## Installation

1. Clone the repository:
```bash
git clone [repository-url]
cd [repository-name]
```

2. Create a `.env` file in the project root with your API keys:
```bash
OPENAI_API_KEY=your_openai_key
SERPAPI_KEY=your_serpapi_key
SCRAPING_BEE_API_KEY=your_scrapingbee_key
```

3. Build the project:
```bash
cargo build --release
```

## Usage

The tool can be used in both CLI and interactive modes.

### Interactive Mode

Simply run the program without arguments:
```bash
cargo run --release
```

This will present a menu with options to:
- Search for documentation URLs
- Scrape specific URLs
- Crawl from a starting point
- Refresh existing documentation

### CLI Mode

#### Search for URLs
```bash
cargo run --release -- search --query "api documentation websockets" --save --num-urls 10
```

#### Scrape URLs from a file
```bash
cargo run --release -- scrape --urls-file urls.txt --output-dir output
```

#### Crawl from a starting URL
```bash
cargo run --release -- crawl --url "https://example.com/docs" --output-dir output
```

#### Refresh existing documentation
```bash
cargo run --release -- refresh --output-dir output
```

## Project Structure

- `src/`
  - `ai/` - AI processing logic using OpenAI
  - `crawling/` - Web crawling functionality
  - `db/` - Database operations
  - `logging/` - Logging utilities
  - `metadata/` - Documentation metadata management
  - `refresh/` - Documentation refresh functionality
  - `scraping/` - Web scraping using ScrapingBee
  - `search/` - Search functionality using SerpAPI
  - `utils/` - Common utilities

## Configuration

### URL Filtering

- `disallow_urls.txt` - Contains URLs to skip during crawling
- `processed_urls.txt` - Tracks already processed URLs

### Output

- `output/` - Directory for processed markdown files
- `output/metadata.json` - Metadata for all processed documents

### Database

- `data/data.db` - SQLite database for storing documents and search results

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

MIT License

## Acknowledgments

- OpenAI for AI processing
- SerpAPI for search capabilities
- ScrapingBee for web scraping
