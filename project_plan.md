# Documentation Aggregator

## Overview

A Rust-based application for aggregating, storing, and searching software documentation from various online sources.

## Core Components

1. Terminal Interface (ratatui)
   - Interactive menu-driven CLI
   - Database management commands
   - Search functionality

2. Database (SQLx)
   - Efficient storage of scraped content
   - URL tracking and deduplication
   - Vector embeddings for semantic search

3. Content Processing
   - URL scraping via ScrapingBee
   - Search result aggregation via SerpAPI
   - Content analysis with OpenAI API
   - Semantic search capabilities

## Data Flow

1. Input Sources
   - URLs from file (urls.txt)
   - Search queries from file (queries.txt)
   - Direct CLI input

2. Processing Pipeline
   - URL sanitization and deduplication
   - Content scraping and extraction
   - AI-powered content analysis
   - Vector embedding generation
   - Database storage

3. Search & Retrieval
   - Semantic search queries
   - Natural language question answering
   - Content summarization
   - Interactive filtering

## Technical Stack

- Backend: Rust with Tokio async runtime
- Database: SQLx for database operations
- UI: Ratatui for terminal interface
- APIs: 
  - ScrapingBee for web scraping
  - SerpAPI for search results
  - OpenAI for content processing
