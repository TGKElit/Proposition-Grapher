# Proposition-Grapher

## Installation

### Database
Install and configure PostgreSQL:

[Ubuntu](https://ubuntu.com/server/docs/databases-postgresql)
[macOS](https://www.postgresql.org/download/macosx/)

Create a database, then go to /back_end and create a .env using .env.example as a template.
In /back_end, run ```sqlx migrate run```
### Frontend
Go to /front_end and run ```npm install``` or equivalent.
Then run ```npm run build``` or equivalent.
### Backend
Go to /back_end and install [rust and cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).
Then run ```cargo run```

If everything is setup correctly, the website should be running on localhost:3000.

## Testing
In /back_end, run ```cargo test```
