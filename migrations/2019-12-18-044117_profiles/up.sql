create table if not exists profiles (
    id serial primary key,
    name varchar not null,
    photo varchar not null,
    video varchar not null,
    needed int not null,
    collected int not null,
    description text not null,
    published boolean not null default 'f',
    created_at timestamp not null default now()
);
