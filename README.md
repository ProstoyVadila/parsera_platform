# Web Scraper Platform

## Description

This is a web scraper platform based on microservices architecture. It is designed to be scalable, fast, manageable via API and work around the clock. The main idea behind this scrapper is to get a lot of persistent data from websites on a daily basis. It's not suitable for one-time scrapping.

## Architecture

### Microservices

![microservices architecture](/utils/microservices_architecture.png)

### Services

- [**Api Gateway**](#api-gateway) is responsible for authentication, adding new crawlers and sites to scrapping process and getting data.
- [**Scheduler**](#scheduler) is an orchestrator responsible for scheduling routine tasks, managing scraping state.
- [**Status Manager**](#status-manager) is responsible for listening scheduler events, storing events logs and updating status on fronted via websockets.
- [**Scraper**](#scraper) is responsible for getting data from websites.
- [**Heavy Artillery**](#heavy-artillery) is responsible for getting data from websites using Selenium WebDriver when common scraper fails.
- [**Extractor**](#extractor) is responsible for parsing data.
- [**Anonymizer**](#anonymizer) is responsible for managing the proxies and store them in Redis.
- [**Database Manager**](#database-manager) is responsible for storing parsed data and its rotation.
- [**Broker**](#rabbitmq-as-a-broker) is responsible for asynchronous communication between services.

### Databases

- **Users DB** is a PostgreSQL database stores users information.
- **Routines DB** is a PostgreSQL database stores crawlers, routines and tasks metadata.
- **Scraped Data DB** is a PostreSQL database (by now) stores all parsed data.
- **Event Logs DB** is an Elasticsearch database responsible stores all events.
- **Prometheus Storage** is a time series database for storing metrics.

### RabbitMQ as a broker

I tried to keep it as simple as possible. Api Gateway just pubslish event to the message broker and forgot about it. Scheduler as an Orchestrator consumes all events and can produce messages to all services. Status Manager only gets all events as well to update Frontend with actual status of task. All other service can only consume events from Scheduler and publish to it.

![broker architecture](/utils/rabbit_architecture.png)

[Why I chose RabbitMQ?](#why-not-kafka)

## Whys

### Why microservices?

I chose microservices architecture because it's scalable. It's easy to add new services and scale them independently. For example, if we need to crawl more sites simultaneously, we can just add more Scraper instances. If we need to add new functionality, it's not a big issue to append new services (for example, services to transform and load data to get a whole ETL process). Microservice architecture allows to separate business logic domains naturally and maintain and develop them independently. But of course everything has a cost. And the biggest issues in that approach are pretty fast growing complexity, keep up strict interface policy and maintain fault tolerance for each service.

### Why split crawling process into two independent services?

First of all, It's different processes. Scraping or basically getting html (or json, xml as well) contains its own bunch of issues to handle. Especially if target resource's trying to avoid you to get thier data. Your scraper should look like a natural traffic. It includes changing headers and other parameters of request, using proxy and random intervals between request to the same resource. So there is no need for extracting data process to know that. And on the other hand successful request doesn't mean you get a desirable data. Invalid xpath can stand on the way. That is why it's important to have a mechanism to reextract data without resending a request.

### Why Rust?

I chose Rust for four reasons. Its resource efficiency, type safety, speed and maintainability. That's true it's pretty hard to write code in Rust comparing to other languages and its ecosystem could be ~~better~~ bigger. But Rust is strongly typed language and it has pretty explicit syntax. Include here an extremly strict memory usage and you will have a language allows you to build safe and easy to maintain services. And imo Cargo is one of the best built-in package managers.

### Why not Kafka?

I chose RabbitMQ because it's easier to work with and it seems like RabbitMQ is a sufficient option for my purposes for a long time. But Kafka would be a pretty good choice. I cover that in [What's next chapter](#whats-next)

## Service Description

### Api Gateway

Blabla

### Scheduler

This service is responsible for scheduling the tasks for Scrapper. It is written in Python using FastAPI framework. It sets values to RabbitMQ queue for Scrapper and stores them in Postgres. Scheduler manages routine tasks to refresh the data from the websites as well.

### Scraper

This service is responsible for scrapping the data from the websites. It is written in Rust using Tokio and Reqwest. It gets the data from RabbitMQ queue and updates it in Postgres.

### Heavy Artillery

Blabla

### Extractor

This service is responsible for parsing the data. It is written in Python. It parses data from websites using xpaths and stores them in db.

### Anonymizer

This service is responsible for managing the proxies for Scrapper. It is written in Go using Gin framework. It gets the proxies from resource and manages thier availability.

### Database manager

figure out what to use (postgres, mongo, scylladb)

### Status manager

Blabla

## What's Next?
