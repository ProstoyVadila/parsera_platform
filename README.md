# Web Scraper Platform

This is a web scraper platform based on microservices architecture. It is designed to be scalable, fast, manageable via API and work around the clock. The main idea behind this scraper platform is to get a lot of persistent data from websites on a daily basis. It's not suitable for one-time scrapping.

## Architecture

### Preconditions

1. **I need to handle an increasing amount of users.** So my system should be scalable.
2. **A common scraping scenario usually includes periodical scraping and/or pagination scraping.** So I need a scheduler to manage routine tasks and pagination distribution among scrapers.
3. **When a user downloads their data I don't really need it anymore.** So I don't have to store all parsed data _forever_. That means I don't really need a wide-column database like Cassandra, a common relational solution is more than enough.
4. **A user can put invalid xpaths to their crawler**. So I should have notification and reparsing mechanisms without sending a request again.
5. **Some web pages are pretty popular and many users may want to scrape them**. So I should store scraping pages in a cache for some time to avoid unnecessary scraping. _This feature only makes sense if there are a certain number of active users in the system._

### Microservices

This architecture below with some changes could be useful in different scraping scenarios.

![microservices architecture](/docs/microservices_architecture.png)

### Services

- [**Api Gateway**](#api-gateway) is responsible for authentication and api for whole backend.
- [**Scheduler**](#scheduler) orchestrates routine tasks and manages scraping state.
- [**Status Manager**](#status-manager) listens to all events, updates status on frontend via websockets.
- [**Scraper**](#scraper) is responsible for getting data from websites.
- [**Heavy Artillery**](#heavy-artillery) scrapes data from websites using Selenium WebDriver and other advanced techinques when common scraper fails.
- [**Extractor**](#extractor) is responsible for parsing data.
- [**Anonymizer**](#anonymizer) manages the proxies and stores them in Redis.
- [**Database Manager**](#database-manager) stores parsed data and its rotation.
- [**Notification Service**](#notification-service) notifies users via email or messenger when thier tasks are done.
- [**Broker**](#rabbitmq-as-a-broker) is responsible for asynchronous communication between services.

### Databases

<!-- TODO: update this -->

- **Users DB** is a PostgreSQL database stores users information.
- **Routines DB** is a PostgreSQL database stores crawlers, routines and tasks metadata.
- **Scraped Data DB** is a PostreSQL database (by now) stores all parsed data.
- **Event Logs DB** is an Elasticsearch database responsible stores all events.
- **Prometheus Storage** is a time series database for storing metrics.

### RabbitMQ as a broker

I tried to keep it as simple as possible. Api Gateway just publishes a new event to the message broker and forgets about it. Scheduler as an Orchestrator consumes all events and can produce messages to all services. Status Manager recieves all events as well to update Frontend with an actual status of the task. All other services can only consume events from Scheduler and publish updates to it.

![broker architecture](/docs/rabbit_architecture.png)

[Why I chose RabbitMQ?](#why-not-kafka)

## Whys

### Why microservices?

I chose microservices architecture because it's scalable. It's easy to add new services and scale them independently. For example, if we need to crawl more sites simultaneously, we can just add more Scraper instances. If we need to add new functionality, it's not a big issue to append new services (for example, services to transform and load data to get a whole ETL process). Microservice architecture allows to separate business logic domains naturally and maintain and develop them independently. But of course everything comes at a cost. And the biggest issues in that approach are pretty fast growing complexity, a requirement to keep up strict interface policy and maintaining fault tolerance for each service.

### Why split crawling process into two independent services?

First of all, those are different processes. Scraping or basically getting html (or json, xml etc) contains its own bunch of issues to handle. Especially if the target site is trying to forbid you from getting their data. Your scraper should look like a natural traffic. It includes changing headers and other parameters of request, using proxy and random intervals between request to the same resource. So there is no need for extracting data process to know that. Second of all, a successful request doesn't mean you get desirable data. An invalid xpath can stand in the way. That is why it's important to have a mechanism to reextract data without resending a request.

### Why Rust?

I chose Rust for four reasons. Its resource efficiency, type safety, speed and maintainability. Surely, writing code in Rust is pretty hard compared to other languages and its ecosystem could be ~~better~~ bigger. But Rust is strongly typed language and it has pretty explicit syntax. Count in an extremly strict memory usage and you will have a language that allows you to build safe and easy-to-maintain services. And imo Cargo is one of the best built-in package managers.

### Why not Kafka?

I chose RabbitMQ because it's easier to work with and it seems like this broker is a sufficient option for my purposes for a long time. But Kafka would be a pretty good choice too. I cover that in [What's next chapter](#whats-next)

## Service Description

### Api Gateway

Blabla

### Scheduler

blabla

### Scraper

blabla

### Heavy Artillery

Blabla

### Extractor

blabla

### Anonymizer

blabla

### Database manager

blabla

### Notification Service

bombom

### Status manager

blabla

## What's Next?

With an increasing amount of users (>10k) the next step in this system's evolution could be a migration from RabbitMQ to Kafka. It is a way more scalable and persistent message broker. Perhaps, it could be useful to look closer at Apache Pulsar as well.

At the same time our system will be under more and more pressure on the database with scraped data. Read operations may experience larger latency just trying to catch a free connection because of writing processes. A good solution would be to implement Command Query Responsibility Segregation pattern (CQRS). That will make the read and write operations independent.

If at some point it's clear we have to store all scraped data for a long time the best choice would be to change the relational database to a wide-column solution. A simple model for scraped data doesn't need changes and migrations and its indexes are pretty obvious. That means we're successfuly avoiding the biggest issue with wide-columns. In our architecture the write operation significantly prevails over the read. That type of databases are the best in this scenario. And finally, wide-column databases can be scaled extremely easily compared to others. So Cassandra or ScyllaDB could be a perfect match. Adding a pipeline for storing very old data in S3 bucket is not a bad idea as well.

Another step will be scaling and spliting Scheduler. It will aggregate tremendous amount of events and it's still a single point of failure. Zookeeper could be a good option to fix the last issue. Btw Scheduler has too many actions to do. Orchestrating event flow, managing routine tasks and storing events log. With some difficulties that logic could be separated into different services. Such as Decision, Orchestrator and Routine Manager. But it is quite a big challenge and should be done with caution about data consistency. And perhaps, Pulsar can help here too by containing all that logic into its Functions. Anyway all decisions about Scheduler should be done after a comprehensive analysis of all system's metrics in production environment.

There is another option to make the system's life easier. It's not the best idea but it should be discussed as well. If we have a constant high load over the system, Scheduler could form and send batches of scraping events instead of publishing each one independently. It will decrease pressure over the message broker and potentially speed things up because Scraper and Extractor process events concurrenty and they will not lose time handling each event in consumer. But there are many pitfalls on the way. First of all, it will increase the complexity of an already complicated Scheduler's logic. It should figure out how to form those batches properly keeping in mind that the full batch of events to scrape the same site will maximize the chances to be marked as unwanted traffic and be banned. And at the same time there is a big question on how to handle situations when some events in the batch were successful and some were not. So this approach creates a lot of issues that must be solved.
