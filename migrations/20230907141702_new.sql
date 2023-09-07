-- Your SQL goes here
create table task
(
    id          text                                    not null primary key,
    created_at datetime                                not null,
    updated_at datetime                                not null,
    status      text check (status in ('init', 'done')) not null,
    name       text                                    not null
);
