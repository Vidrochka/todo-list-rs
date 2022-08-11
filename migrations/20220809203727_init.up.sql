CREATE TABLE users (
    id uuid,
    login varchar(128),
    password varchar(128),

    PRIMARY KEY(id)
);

CREATE TABLE todo_lists (
    id UUID,
    user_id UUID NOT NULL,
    name varchar(1024) NOT NULL,

    PRIMARY KEY(id),
    CONSTRAINT fk__user_id__users__id
        FOREIGN KEY(user_id)
            REFERENCES users(id)
            ON DELETE CASCADE
);

create unique index idx__user_id on todo_lists using btree (user_id);

CREATE TABLE tasks (
    id UUID,
    todo_list_id UUID NOT NULL,
    description TEXT NOT NULL,
    "order" INT NOT NULL,

    PRIMARY KEY(id),
    CONSTRAINT fk__todo_list_id__todo_lists__id
        FOREIGN KEY(todo_list_id)
            REFERENCES todo_lists(id)
            ON DELETE CASCADE
);