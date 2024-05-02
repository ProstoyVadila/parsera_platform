import smtplib

from notification_service.sender import Sender


class EmailSender(Sender):

    def __init__(self, smtp_host: str, smtp_port: str, email: str, login: str, password: str) -> None:
        self._smtp_host = smtp_host
        self._smtp_port = smtp_port
        self._email = email
        self._login = login
        self._passwrod = password
        
    @property
    def _sender_creds(self) -> str:
        return self._email

    def send(self, reciever: str, msg: str):
        try:
            server = smtplib.SMTP(self._smtp_host, self._smtp_port)
            server.ehlo()
            server.sendmail(reciever, self._sender_creds, msg)
        except Exception as exc:
            print(exc)