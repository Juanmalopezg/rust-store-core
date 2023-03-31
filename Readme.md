## rust-store-core

This is the backend component of a simple e-commerce app. The backend provides a REST API for accessing a product list, adding items to the shopping cart, and processing payments.

## Usage

To run the application, you need to have Rust and Cargo installed on your system. 

Once you have Rust and Cargo installed, you can run the application with the following command:

`cargo run`


By default, the application listens on port **3000**. You can change the port by setting the `PORT` environment variable.

## Docker

Alternatively, you can run the application in a Docker container. To build the Docker image, run 

`docker build -t store-core .`

Next, to run the Docker container, use:

`docker run --name store-core -d -p 3000:3000 store-core`


The application should now be running on http://localhost:3000.


