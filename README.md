# Rust Web Server Architecture

This repo contains a basic architecture to use when developing a web service in Rust, it is async and well organized. This repository will be updated with new features and architectural changes.

## How the architecture works

The organization is very simple, inside the API folder is where will be created the controllers, you can declare individual routes or a compound with more than one route.

The services folder contains the services that can be used to process data, they are usually called by the controllers.