# MySQL Docker Container

This repository contains a Docker Compose file for running a MySQL database server in a Docker container.

## Setup

1. Clone this repository to your local machine.
2. Make sure you have Docker and Docker Compose installed.
3. Create a file called `.env` in the `mysql-container` directory with the following contents:

```bash
MYSQL_ROOT_PASSWORD=myrootpassword
MYSQL_USER=myuser
MYSQL_PASSWORD=mypassword
MYSQL_ALLOW_EMPTY_PASSWORD=no
MYSQL_RANDOM_ROOT_PASSWORD=no
MYSQL_CHARACTER_SET_SERVER=utf8mb4
MYSQL_COLLATION_SERVER=utf8mb4_general_ci
MYSQL_INITDB_SKIP_TZINFO=no
```

4. Start the MySQL container by running the following command in the `mysql-container` directory:

```bash
docker-compose --env-file .env up -d
```

5. Populate the `users` table with some sample data. Here is the SQL code to create and populate the `users` table with some sample data:

```sql
CREATE DATABASE IF NOT EXISTS steelstack;
USE steelstack;

DROP TABLE IF EXISTS users;

CREATE TABLE users (
  id mediumint(8) unsigned NOT NULL auto_increment,
  name varchar(255) default NULL,
  phone varchar(100) default NULL,
  email varchar(255) default NULL,
  country varchar(100) default NULL,
  PRIMARY KEY (id)
) AUTO_INCREMENT=1;

INSERT INTO users (name, phone, email, country)
VALUES
  ("Pearl Patton", "1-242-385-2722", "sit@google.couk", "China"),
  ("Anjolie Crosby", "1-149-719-3542", "ac@hotmail.net", "South Korea"),
  ("Mercedes Wiggins", "(456) 315-7635", "vulputate.risus@icloud.org", "Mexico"),
  ("Luke Foreman", "(457) 811-1212", "vitae.erat.vivamus@icloud.com", "Ukraine"),
  ("Keelie Shelton", "(957) 814-5163", "et.libero@hotmail.ca", "India");
```

Feel free to modify the sample data or add your own data to the `users` table as needed.
