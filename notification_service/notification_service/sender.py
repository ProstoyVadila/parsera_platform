from abc import ABC, abstractmethod


class Sender(ABC):

    @property
    def _sender_creds() -> str:
        NotImplementedError

    @abstractmethod
    def send(reviever: str, msg: str):
        NotImplementedError
