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
    deadline date
);

insert into todo_list (title) values ('List 1'), ('List 2');

insert into todo_list (title) values ('List 1'), ('List 2');
insert into todo_item (title, procent, list_id) values('todo 1', 10,1),('todo 2', 20,1), ('todo 3', 30,2);
