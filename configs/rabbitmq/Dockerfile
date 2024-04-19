FROM rabbitmq:3.11.7-management-alpine


ARG RABBITMQ_ADMIN_USER
ARG RABBITMQ_ADMIN_PASSWORD
ARG RABBITMQ_GUEST_USER
ARG RABBITMQ_GUEST_PASSWORD
ARG RABBITMQ_SERVICE_USER
ARG RABBITMQ_SERVICE_PASSWORD

COPY conf/rabbitmq.conf conf/definitions.json scripts/init.sh /etc/rabbitmq/

# hashing passwords from env vars and replacing them in the config file
RUN chmod +x /etc/rabbitmq/init.sh && ./etc/rabbitmq/init.sh
