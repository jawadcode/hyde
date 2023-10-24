#! /usr/bin/env python3

from faker import Faker
from faker.providers import lorem
from mdgen import MarkdownPostProvider
from datetime import datetime, timezone

POSTS_DIR = "posts/"
fake = Faker()
fake.add_provider(lorem)
fake.add_provider(MarkdownPostProvider)

for i in range(0, 200):
    filename = f"post{(i + 1):03}.md"
    title = fake.sentence(nb_words=3)
    date_time = datetime.now(timezone.utc).isoformat(sep="T")
    language = "en-GB"
    tags = ""
    content = fake.post(size="large")
    result = f"""---
title: {title}
timestamp: {date_time}
language: {language}
tags: {tags}
---

{content}"""

    with open(POSTS_DIR + filename, "w") as f:
        f.write(result)