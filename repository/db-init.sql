CREATE TABLE IF NOT EXISTS publications_of_interest (
    user_uuid VARCHAR(80) PRIMARY KEY,
    publications_of_interest json NOT NULL
);

CREATE TABLE IF NOT EXISTS subscription_topics (
    user_uuid VARCHAR(80) NOT NULL,
    site_key VARCHAR(40) NOT NULL,
    subscription_topics json NOT NULL,
    PRIMARY KEY (user_uuid, site_key)
);