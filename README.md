# TickTap Rust Demonstration Microservice Ticketing API

Ticktap is a fictional service corporation that provides digital access to in person and virtual events and services for its customers (Conferences, Travel, films, media events, etc.) For example, viewers of The New York Philharmonic Orchestra may purchase tickets for a symphony. This application represents a way Ticktap may engineer some of their core services.

## Motivation
There are a myriad of ways to engineer a backend API. This application is an attempt to show a clean, modern way of doing that in Rust. This application is to showcase common elements of a microservice such as REST, basic business logic, database interaction, etc. A mark of a well-designed application is a new deveoper should be able to jump in and understand quickly with minimal fuss. This application shows a defined standard structure for a designing a microservice: the controllers, services, and repositories, among other design elements.

Note: Some security elements were omitted for ease of demonstration

## Application Showcase Highlights

Ticktap exists to showcase an approach to the following elements that would be part of most microservices:

- HTTP requests/responses in REST style
- Object (JSON) marshalling and unmarshalling
- CRUD operations on a remote database
- Application organization (controllers, services, repositories, etc)
- Service-to-service communication
- Externalized application configuration (Environment Variables)
- Sample business logic
- Containerization
- Error handling
- Logging
- Unit tests
- Documentation (comments, README, swagger)

## Terminology


- Account: Refers to an individual account (consumer or corporate) or a host (e.g. New York Orchestra)
- Event: An event (not the computing science kind) or service (say travel) that occurs for a duration of time. An event will have an associated maximum capacity. An event must have an associated host account.
- Ticket: A unit of representation for an individual person at an event.
- Transaction: An account can purchase one to many tickets for an event. Multiple tickets can be purchased in one transaction. Business logic in ticktap ensures that tickets can only be required when capacity has not been met.

## Architecture
![alt text](https://i.imgur.com/J2fkafh.png)

The ticktap demonstration application is composed of two services:

- Main Service: facilitates most of the requests
- Ticketing Service: facilites ticket creation/deletion from a transaction

**A user interface is coming, but for now manual http requests must be used.**

## Installation
Like most microservices and networked applications, Ticktap has many dependencies. Both the main service and the ticketing service must be running, as they require each other when a request is made.


### Requirements
This application requires:

- Rust 1.51+
- MySQL 8.0+

### Build via command line
This way is not recommended for development or production, as it is clunky compared to the alternatives.
To build:
```./mvnw package```

To run a service:
```cargo run```

### Build via docker-compose

```docker-compose up -d```


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

### Suggestions

- For local development, stand up the mysql databases in docker. Here is [mysql](https://hub.docker.com/_/mysql) for docker.
- Use `docker-compose.yaml` for standing up the database locally
- A great tool for managing mysql is [DBeaver](https://dbeaver.io/download/). This can be used to load in the schema.
- Ensure the databases are not running on the same port
- Run main service and ticketing service in separate development IDE's. Use IntelliJ or VSCode
- Postman is a great tool for interacting with the application. [Here](https://www.postman.com/)
- Recommended instantiation order: stand up the databases, then both services.

### External Dependencies


#### Databases

Ticktap requires databases for each of the services. The schemas are in the services respective `./schemas` folder. The databases and the schemas must be up prior to the ticktap main service and ticketing service running.

## Usage
Use cURL or Postman (or equivalent) to interact with the application.

Documentation for API usage is in swagger. To visualize this documentation, copy contents from `swagger.yml` to this editor:

[Swagger Editor](https://editor.swagger.io/)