
from datetime import datetime
from uuid import UUID

import orjson
from enum import StrEnum
from pydantic import BaseModel, Field


def orjson_dumps(v, *, default):
    # orjson.dumps returns bytes, to match standard json.dumps we need to decode
    return orjson.dumps(v, default=default).decode()


class Model(BaseModel):
    class Config:
        json_loads = orjson.loads
        json_dumps = orjson_dumps


class EventCommand(StrEnum):
    REGISTER_CRAWLER = "register_crawler"
    SCRAPE_PAGE = "scrape_page"
    EXTRACT_PAGE = "extract_page"
    STORE_PAGE = "store_page"
    NotifyUser = "notify_user"
    SLEEP = "sleep"


class Priority(StrEnum):
    TOP = "top"
    HIGH = "high"
    COMMON = "common"
    LOW = "low"


class ExternalData(Model):
    pass


class NotificationLevel(StrEnum):
    JOBS_DONE = "JobsDone"
    JOBS_FAILED = "JobsFailed"
    STATISTICS = "Statistics"
    DO_NOT_DISTURB = "DoNotDisturb"


class NotifyVia(Model):
    email: str
    telegram: str


class NotifyEvery(StrEnum):
    DAY = "day"
    WEEK = "week"
    MONTH = "month"


class NotificationOptions(Model):
    level: NotificationLevel
    via: list[NotifyVia]
    every: NotifyEvery | None = None


class Page(Model):
    id: UUID
    crawler_id: UUID
    site_id: UUID
    url: str
    domain: str
    is_pagination: str
    time_reparsed: int
    priority: Priority
    notification: NotificationOptions
    xpaths: dict[str, str]
    created_at: datetime
    updated_at: datetime
    html: str | None = None
    data: dict[str, str] | None = None
    meta: str | None = None


class EventProtocolData(StrEnum):
    EXTERNAL = ExternalData
    INTERNAL = Page


class EventProtocol(Model):
    command: EventCommand
    data: EventProtocolData
