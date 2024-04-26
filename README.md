# Web Scraper Platform

This is a web scraper platform based on microservices architecture. It is designed to be scalable, fast, manageable via API and work around the clock. The main idea behind this scrapper is to get a lot of persistent data from websites on a daily basis. It's not suitable for one-time scrapping.

## Architecture

### Preconditions

1. **I need to handle an increasing amount of users.** So my system should be scalable.
2. **A common scraping scenario usually includes periodical scraping or pagination scraping.** So I need a scheduler to manage routine tasks and pagination destribution among scrapers.
3. **I don't need to store all parsed data _endlessly_. When a user got it I don't need it anymore.** So I don't really need a wide-column database like Cassandra, a common relational solution is more than enough.
4. **A user can put invalid xpaths to thier crawler**. So I should have notification and reparsing mechanisms.

### Microservices

This architecture below with some changes could be useful in different scraping scenarios.

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

With increasing amount of users (>10k) the next step in evolution of that system could be migration from RabbitMQ to Kafka. It is a way more scalable and persistent message broker. Perhaps, it could be useful to look closer at Apache Pulsar as well.

At the same time our system will under more and more pressure on the database with scraped data. Read operations may experience larger latency just trying to catch a free connection from writing processes. A good solution would be implement Command Query Responsibility Segregation pattern (CQRS). This will make the read and write operations independent.

If at some point it's clear we have to store all scraped data no matter what the best choice would be change relational database to wide-column solution. A simple model for scraped data doesn't need changes and migrations and its indexes are pretty obvious. That means we're successfuly avoiding the biggest issue with wide-columns. In our architecture the write operation significantly prevails over the read. That type of databases are the best in this scenario. And finally, wide-column databases can be scaled extremly easily comparing to others. So Cassandra or ScyllaDB could be a perfect match. Add pipeline for storing old data in S3 bucket is not bad idea as well.

Another step will be scaling and spliting Scheduler. It will aggregate tremendous amount of events and it's still a single point of failure. Zookeeper could be a good option to fix the last issue. Meanwhile Scheduler have to many actions to do. Orchestrate event flow, manage routine tasks and store events log. With some difficulties it could be separated to different services. Decision, Orchestrator and Routine Manager for example. But it is quite a big challenge and should be done with all caution. And perhaps, Pulsar can help here either by implementing all that logic into its Functions. Anyway all decisions about Scheduler should be done after a comprehensive analysis of all system in production environment.
