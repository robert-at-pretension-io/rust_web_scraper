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
## Project Artifacts and Directories

### Output Directory (`output/`)
The main directory where processed documentation and metadata are stored.

#### Structure:
- `*.md` - Processed markdown files
  - Filenames are AI-generated based on content
  - Contains formatted documentation with consistent structure
  - Includes metadata headers and proper markdown formatting
- `metadata.json` - Project-wide metadata file containing:
  - Document summaries
  - Processing timestamps
  - Source URLs
  - AI model information
  - Custom metadata fields
  - Processing success/failure status

### Data Directory (`data/`)
Contains persistent storage and database files.

#### Contents:
- `data.db` - SQLite database containing:
  - Documents table (processed content)
  - Search queries history
  - Search results
  - Application logs
  - Processing metadata

### Logs Directory (`logs/`)
Created automatically to store various log files.

#### Log Files:
- `app.log` - General application logs
  - Processing status
  - Error messages
  - System events
  - Timestamps for all operations
- `openai.log` - AI interaction logs
  - Prompts sent to OpenAI
  - Responses received
  - Processing times
  - Model usage

### Text Files

#### URL Management
- `urls.txt`
  - Input file for URLs to process
  - Can be manually created or generated from search
  - One URL per line
  - Used by scrape command

- `processed_urls.txt`
  - Tracks successfully processed URLs
  - Prevents duplicate processing
  - Automatically updated during operations
  - Used for resume capability

- `disallow_urls.txt`
  - URLs to skip during crawling
  - Can be manually edited
  - Used for filtering unwanted domains/paths
  - Automatically updated based on user selections

### Temporary Files
The script may create temporary files during operation:

- `.tmp_*` files for content processing
- Cache files for API responses
- Temporary database journal files
- Lock files during concurrent operations

### File Formats

#### Markdown Files
```markdown
# Document Title

- Source: [Original URL]
- Last Updated: [Timestamp]
- Purpose: [Processing Purpose]
- AI Model: [Model Used]

[Processed Content]
```

#### metadata.json
```json
{
  "project_name": "Documentation Project",
  "created_at": "2023-01-01T00:00:00Z",
  "last_updated": "2023-01-01T00:00:00Z",
  "documents": [
    {
      "filename": "example-doc.md",
      "title": "Example Documentation",
      "description": "Brief description",
      "source_url": "https://example.com/docs",
      "last_updated": "2023-01-01T00:00:00Z",
      "purpose": "Documentation purpose",
      "ai_model": "gpt-4o-mini",
      "word_count": 1000,
      "tags": ["api", "documentation"],
      "processing_time": 2.5,
      "success": true,
      "error_message": null,
      "custom_metadata": {}
    }
  ],
  "custom_metadata": {}
}
```

### Storage Considerations

- The `output/` directory can grow large with many documents
- Log files are not automatically rotated
- The SQLite database will grow with usage
- Consider periodic cleanup of:
  - Old log files
  - Temporary files
  - Processed URLs for completed projects

### Backup Recommendations

Important files to backup:
1. `output/metadata.json`
2. `data/data.db`
3. Custom URL lists
4. Configuration files

### File Permissions

The script needs read/write access to:
- Current directory
- `output/` directory
- `data/` directory
- `logs/` directory
- All `.txt` files

### Clean Up

To clean up all artifacts:
```bash
rm -rf output/*
rm -rf logs/*
rm data/data.db
rm processed_urls.txt
rm urls.txt
```

Keep `output/.gitkeep` and essential configuration files.
