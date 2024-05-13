# Rust REST API

This is a RESTful API built with Rust, designed to handle posts, likes, and user authentication features including logins and sign-ups. The service uses PostgreSQL for efficient data storage and retrieval, supporting scalability as user interactions increase.

## Features

- **Posts Management**: Create, update, and delete posts.
- **Likes**: Users can like posts.
- **User Authentication**: Support for user logins and sign-ups.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

You will need PostgreSQL installed on your machine. Follow the instructions for your operating system:

#### Ubuntu

```bash
sudo apt update
sudo apt install postgresql postgresql-contrib
```

#### macOS

```bash
brew install postgresql
brew services start postgresql
```

#### Windows

Download and install PostgreSQL from the [official website](https://www.postgresql.org/download/).

### Rust Installation

Ensure you have Rust installed:

```bash
rustup update
rustup install stable
```

### Database Setup

1. Start PostgreSQL service:

    ```bash
    sudo service postgresql start  # Ubuntu
    brew services start postgresql  # macOS
    ```

2. Create a new PostgreSQL database:

    ```bash
    sudo -u postgres psql  # Ubuntu
    psql postgres  # macOS or Windows (if added to PATH)
    ```

    In the PostgreSQL shell:

    ```sql
    CREATE DATABASE your_database_name;
    \q
    ```

    Making Tables
    ```
    CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL
    );

    CREATE TABLE tweets (
    id SERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    likes INTEGER,
    created_at TIMESTAMP NOT NULL
    );
    ```

3. Set up your `.env` file with the PostgreSQL connection string:

    ```env
    DATABASE_URL=postgres://username:password@localhost/your_database_name
    JWT_SECRET=your_jwt_secret
    ```

### Running the Application

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/your_username/your_project.git
cd your_project
```

Run the application:

```bash
cargo run
```

This will start the server on `http://localhost:8080`.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.