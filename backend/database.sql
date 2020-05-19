drop table if exists todo_item;

create table todo_item (
    id serial primary key,
    title varchar(150) not null,
    procent integer not null,
    deadline date
);

insert into todo_item (title, procent) values('todo 1', 10),('todo 2', 20), ('todo 3', 30);
