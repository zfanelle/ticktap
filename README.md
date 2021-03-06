# TickTap Rust Demonstration Microservice API

TickTap is a fictional ticket services corporation that provides digital access to . This can include travel, conferences, films, media events, etc. 

## Motivation
There are a myriad of ways to engineer an API. This application is an attempt to show a clean, modern way of doing that in Rust. This application is to showcase common elements of a microservice such as REST, basic business logic, database interaction, ORM's, exceptions, service-to-service communication, deployment configuration, clooud nativity, and logging. This application will also showcase a defined structure for a microservice: the controllers, services, and repositories.

Note: Some security elements were omitted for the purposes of demonstration

### Showcase Highlights


## Installation
TickTap API can be run locally or run in a container


### Requirements
This application requires:

- Rust 1.49.0
- Dependencies listed in `Cargo.toml` below

### Build via command line

```cargo build```


To run:
```cargo run```


#### VS Code Run/Debug Configuration
Below is a sample debug configuration if VS Code IDE is used to run the application:

```
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Cargo Debug",
            "cargo": {
                "args": [
                    "build",
                ],
                "filter": {
                    "kind": "bin"
                }
            },
            "args": [
                // "user_arg1",
                // "user_arg2"
            ],
            "cwd": "${workspaceFolder}"
        },
    ]

}
```

## Usage
Use cURL or Postman (or equivalent) to interact with the application. 

Documentation for API usage is in swagger. To visualize this documentation, copy contents from `swagger.yml` to this editor:

[Swagger Editor](https://editor.swagger.io/)