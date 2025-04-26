Steps to run project:
1. Dwonload the Docker.
2. Run the command docker-compose up -d. It will run the postgres container.
3. Go to src folder and run command cargo run 
4. Test all the api

cargo install diesel_cli --no-default-features --features postgres

diesel setup
diesel migration generate create_users_and_posts
diesel migration run
diesel print-schema > src/schema.rs
diesel print-schema > src/schema.rs


