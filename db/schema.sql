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

-- last message in all chats
with last_chat_message as (select chat_id, message, max(sent_at) as sent_at
                           from message
                           group by chat_id)

select chat.id     as chat_id,
       chat.name   as chat_name,
       sent_at,
       lcm.message as last_message
from chat
         left join last_chat_message lcm on chat.id = lcm.chat_id;
