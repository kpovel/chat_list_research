create table if not exists chat (
    id   integer primary key autoincrement,
    name text not null
);

create table if not exists message (
    id      integer primary key,
    chat_id integer  not null,
    message text     not null,
    sent_at datetime not null default current_timestamp,

    foreign key (chat_id) references chat (id)
);


insert into chat (name)
values ('chat_deeznuts');
insert into message (chat_id, message)
values (1, 'last message');


-- last message from a chat
select *
from message
where chat_id = 1
ORDER BY sent_at desc 
limit 1;

