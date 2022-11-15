# cqrs-es-demo-sqlite

> A demo application using the [cqrs-es](https://github.com/serverlesstechnology/cqrs) framework
> with a backing SQLite repository.

Now using the [Axum server](https://crates.io/crates/axum-server) for a much simpler layout.

## Requirements
- rust 1.53 or greater
- [curl](curl/test_api.sh) (or your favorite Restful client) 

## Installation

Clone this repository

    git clone https://github.com/johnbcoces/cqrs-es-demo-sqlite

Start the application

    cargo run

Call the API, the easiest way to do this is the `test_api.sh` curl script found in the `curl` directory.
Note that the command calls are configured to return a 204 status with no content, 
only the query call will return a `200 OK` response with a body.
For feedback on state you should call a query.

### Docs you might want

- Documentation of cqrs-es crates as well as an introduction to CQRS [can be found here](https://doc.rust-cqrs.org/).
