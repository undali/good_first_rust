# Good First Rust

🦀 **A tool to find "Good First Issues" for Rust projects on GitHub**

## 🌐 Live At **👉 [https://goodfirstrust.return.one/](https://goodfirstrust.return.one/)**

## 📖 About

Good First Rust is a web application that helps newcomers to the Rust ecosystem find beginner-friendly issues to contribute to open source projects. It automatically aggregates and displays "good first issues" from Rust repositories on GitHub, making it easy to discover contribution opportunities.

Currently, it doesn't use API tokens, so we can make maximum of 60 requests per hour to GitHub API. It's sufficient for now.

## ✨ Features

- **Curated Issues**: Automatically(once in every 20 minutes) fetches and store issues labeled as "good first issue" from Rust repositories
- **Star Filter**: Filter issues by repository star count to find popular projects
- **Repository Info**: View repository details including star count and direct links
- **Relative Time**: See how recently issues were created (e.g., "2 hours ago", "3 days ago")
- **Pagination**: Browse through multiple pages of issues

## 🛠️ Tech Stack

- **Backend**: Rust with Axum, Reqwest, Tokio
- **Database**: SQLite for caching issues (rusqlite)
- **Frontend**: HTML, Tailwind CSS, Vanilla JavaScript
- **API**: GitHub REST API for fetching issues

## 🚀 Getting Started

### Running Locally

1. Clone the repository:
   ```bash
   git clone https://github.com/undali/good_first_rust.git
   cd good_first_rust
   ```

2. Build and run:
   ```bash
   cargo run
   ```

3. Open your browser and navigate to `http://localhost:27412/`

## 📝 License

This project is open source and available under the MIT License.

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.
