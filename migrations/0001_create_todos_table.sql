create table if not exists todos
(
    id varchar not null primary key,
    task text not null,
    completed boolean not null
);
