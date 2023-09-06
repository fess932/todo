-- Your SQL goes here
create table task
(
    id          text                                    not null primary key,
    create_time datetime                                not null,
    update_time datetime                                not null,
    status      text check (status in ('init', 'done')) not null,
    title       text                                    not null,
    message     text                                    not null
)
