# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v1.0] - 2025-02-05

### Added
- New Rust CLI portfolio project structure
- Comprehensive JSON-based configuration files for personal portfolio
- New modules in Rust CLI:
  - `education.rs`: Display educational background
  - `repositories.rs`: Showcase personal repositories and projects
- Expanded skills categorization in `skills.json`
- Detailed personal information JSON files:
  - `main.json`: Personal introduction details
  - `contact.json`: Contact information
  - `skills.json`: Technical skills across multiple domains
  - `experience.json`: Professional experience
  - `repositories.json`: Personal projects and repositories

### Changed
- Refactored `main.rs` to dynamically load personal information from JSON files
- Updated `skills.rs` to include more comprehensive skill categories
- Improved modularity of CLI portfolio application
- Enhanced README for better project understanding and setup instructions

### Improvements
- Implemented JSON-based configuration for easier personalization
- Added more detailed skill categories
- Created modular Rust CLI structure for easy expansion
