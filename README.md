# todo-list-rs
Api для todo листа

---

## Формат ошибки

```json
{
    "status_code": "код + название ошибки",
    "detail": "описание ошибки или null"
}
```

## Доступные запросы

### Ping

Тестовый запрос для проверки работы


***Api:***

GET: ``` http://localhost:8080/ping ```

***Ответ:***

```pong```

---

### Register

Запрос для регистрации

***Api:***

POST: ``` http://localhost:8080/api/user/register ```

***Тело:***

```json
{
    "login": "test",
    "password": "test2"
}
```

***Ответ:***

```uuid v4 - ид пользователя```

---

### Login

Запрос для авторизации

***Api:***

POST: ``` http://localhost:8080/api/user/login ```

***Тело:***

```json
{
    "login": "test",
    "password": "test2"
}
```

***Ответ:***

```токен для bearer token авторизации```

---

### Create list

Создание нового списка задач

***Api:***

POST: ``` http://localhost:8080/api/list ```

***Заголовки:***

```Заголовок с bearer token полученным из запроса login```

***Тело:***

```json
{
    "name": "test_list"
}
```

***Ответ:***

```uuid v4 - ид списка```

---

### Delete list

Удаление списка задач

***Api:***

DELETE: ``` http://localhost:8080/api/list ```

***Заголовки:***

```Заголовок с bearer token полученным из запроса login```

***Ответ:***

```Количество удаленных задач```

---

### Update list

Изменение коллекции задач (только название)

***Api:***

PATCH: ``` http://localhost:8080/api/list ```

***Заголовки:***

```Заголовок с bearer token полученным из запроса login```

***Тело:***

```json
{
    "name": "test_list1"
}
```

***Ответ:***

```uuid v4 - ид списка```

---

### Get list

Запрос коллекции задач (только название списка)

***Api:***

GET: ``` http://localhost:8080/api/list ```

***Заголовки:***

```Заголовок с bearer token полученным из запроса login```

***Ответ:***

```json
{
    "id": "c6443c9f-e23d-41c9-ac5c-57c16e5cad10",
    "user_id": "eab21395-62b7-44e0-8e2f-e2bb91e76afc",
    "name": "test_list"
}
```

---

### Get task

Запрос списка задач

***Api:***

GET: ``` http://localhost:8080/api/task ```

***Заголовки:***

```Заголовок с bearer token полученным из запроса login```

***Ответ:***

```json
[
    {
        "id": "9f07e3f6-608c-49a5-a2d5-a6197ba44054",
        "todo_list_id": "c6443c9f-e23d-41c9-ac5c-57c16e5cad10",
        "description": "test 1",
        "order": 1
    },
    {
        "id": "bf38800f-beda-4732-a1a8-38bb9ca2f5ae",
        "todo_list_id": "c6443c9f-e23d-41c9-ac5c-57c16e5cad10",
        "description": "test 3",
        "order": 2
    },
]
```

---

### Get task range

Запрос списка задач

***Api:***

GET: ``` http://localhost:8080/api/task/range ```

***Query параметры:***

* offset
* count

***Заголовки:***

```Заголовок с bearer token полученным из запроса login```

***Ответ:***

```json
[
    {
        "id": "9f07e3f6-608c-49a5-a2d5-a6197ba44054",
        "todo_list_id": "c6443c9f-e23d-41c9-ac5c-57c16e5cad10",
        "description": "test 1",
        "order": 1
    },
    {
        "id": "bf38800f-beda-4732-a1a8-38bb9ca2f5ae",
        "todo_list_id": "c6443c9f-e23d-41c9-ac5c-57c16e5cad10",
        "description": "test 3",
        "order": 2
    },
]
```

---

### Add task

Добавление задачи

***Api:***

POST: ``` http://localhost:8080/api/task ```

***Заголовки:***

```Заголовок с bearer token полученным из запроса login```

***Тело:***

```json
// вставка в конец
{
    "description": "test 4",
    "position": "end"
}

// вставка после указанной задачи
{
    "description": "test 1",
    "position": {
        "after": {
            "task_id": "80d3def8-0791-4038-94c3-1791ae33999d"
        }
    }
}

// вставка перед указанной задачей
{
    "description": "test 1",
    "position": {
        "before": {
            "task_id": "80d3def8-0791-4038-94c3-1791ae33999d"
        }
    }
}
```

***Ответ:***

```uuid v4 - ид задачи```

---

### Delete task

Удаление задачи

***Api:***

DELETE: ``` http://localhost:8080/api/task/{task_id} ```

***Заголовки:***

```Заголовок с bearer token полученным из запроса login```

***Ответ:***

```json
{
    "id": "4ea747ca-4338-4a7d-b978-223312c25723",
    "todo_list_id": "8a642276-50c7-4111-be00-d3b6c8aa85f9",
    "description": "test 6",
    "order": 6
}
```

---

### Update task

Обновление задачи (только описание)

***Api:***

Patch: ``` http://localhost:8080/api/task/{task_id} ```

***Заголовки:***

```Заголовок с bearer token полученным из запроса login```

***Тело:***

```json
{
    "description": "test2"
}
```

***Ответ:***

```json
{
    "id": "26b64886-53bd-49c1-bd5d-788e24de979f",
    "todo_list_id": "5bcbb7e8-f814-48bc-aaf3-b76a308a45ff",
    "description": "test2",
    "order": 2
}
```

---

### Move task

Перемещение задачи

***Api:***

POST: ``` http://localhost:8080/api/task/{task_id}/move ```

***Заголовки:***

```Заголовок с bearer token полученным из запроса login```

***Тело:***

```json
//перемещение задачи в конец
{
    "position": "end"
}

//перемещение задачи после указанной задачи
{
    "position": {
        "after": {
            "task_id": "bf38800f-beda-4732-a1a8-38bb9ca2f5ae"
        }
    }
}

//перемещение задачи перед указанной задачей
{
    "position": {
        "before": {
            "task_id": "bf38800f-beda-4732-a1a8-38bb9ca2f5ae"
        }
    }
}
```

***Ответ:***

```json
{
    "id": "0a0d7f67-5da6-4146-9526-af9850d8a747",
    "todo_list_id": "c6443c9f-e23d-41c9-ac5c-57c16e5cad10",
    "description": "test 2",
    "order": 3
}
```

---

## Конфигурирование

Конфигурируется через ```.env``` файл

* бд
  * **POSTGRES_HOST** - адрес бд
  * **POSTGRES_PORT** - порт бд
  * **POSTGRES_DB** - название бд
  * **POSTGRES_USER** - юзер для бд
  * **POSTGRES_PASSWORD** - пароль для бд

* system
  * **RUST_BACKTRACE** - трассировка для разработки
  * **RUST_LOG** - unused
  * **actix_web** - уровень логирования web actix
  * **DATABASE_URL** - строка подключения к бд

* settings
  * **TODO_SERVICE_IP** - адрес сервиса
  * **TODO_SERVICE_PORT** - порт сервиса

* logging
  * **TODO_SERVICE_LOG_PATH** - адрес файла логов
  * **TODO_SERVICE_FILE_LOG_LEVEL** - уровень логирования в файл
  * **TODO_SERVICE_CONSOLE_LOG_LEVEL** - уровень логирования в консоль

* auth
  * **BEARER_KEY** - ключ щифрования токенов

---

## Настройка

Для удобной работы с бд можно воспользоваться ```docker-compose.postgres.yml```