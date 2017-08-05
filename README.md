# diesel_demo
diesel+rocket web rest api 

This is a REST API project template that uses [Rocket](https://github.com/SergioBenitez/Rocket) framework with [Diesel](https://github.com/diesel-rs/diesel) ORM backed up with Postgresql database.

- You need nightly version of Rust for [Rocket](https://github.com/SergioBenitez/Rocket).
 ```rustup default nightly```
- Install diesel_cli.
  ```
     apt get install libsqlite3-dev
     apt get install libmysqlclient-dev
     apt get install libpq-dev
     cargo install diesel_cli
  ```
- Run database migration from your project directory. First make sure you have a working database url in your .env file.
    ```disel migration run```
- Ignite your diesel powered rocket api 
    ```cargo run```
- Skeleton code is taken from [mgattozi](https://github.com/mgattozzi/mgattozzi)

