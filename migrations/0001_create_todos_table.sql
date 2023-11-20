create table if not exists users
(
    id varchar not null primary key,
    name text not null,
    email text not null unique,
    password text not null
);

create table if not exists user_sessions
(
    id varchar not null primary key,
    user_id varchar not null references users(id),
    token text not null unique,
    created_at timestamp not null default current_timestamp,
    expires_at timestamp not null
);

create table if not exists todos
(
    id varchar not null primary key,
    user_id varchar not null references users(id),
    task text not null,
    completed boolean not null
);
