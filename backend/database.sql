drop table if exists todo_item;
drop table if exists todo_list;

create table todo_item (
    id serial primary key,
    title varchar(150) not null,
    procent integer not null,
    deadline varchar(150) not null
);

insert into todo_item (title, procent, deadline) values('todo 1', 10, '20/20/3202'), ('todo 2', 20, '20/20/3202'), ('todo 3',30, '20/20/3202');
