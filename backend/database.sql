drop table if exists todo_item;
drop table if exists todo_list;

create table todo_list (
    id serial primary key,
    title varchar(150)
);

create table todo_item (
    id serial primary key,
    title varchar(150) not null,
    procent integer not null,
    list_id integer not null,
    foreign key (list_id) references todo_list(id),
    deadline varchar(150) not null
);

insert into todo_list (title) values ('List 1'), ('List 2');
insert into todo_item (title, procent, list_id, deadline) values('todo 1', 10, 1, '20/12/2020'), ('todo 2', 20, 1, '10/10/2030'), ('todo 3', 30, 2, '16/2/2021');
