
from notification_service.libs.email import EmailSender

sender = EmailSender("localhost", 1025, "blabla@fake.me", "blabla", "pass")

sender.send("test@test.tt", "A B O B U S")
