docker Compose file: 
# https://docs.docker.com/compose/compose-file/

how to run docker:
docker-compose up -d

how to run project:
cargo run

EndPoints:

    get TODO by ID
    GET -> http://localhost:8080//todos/{id}

    get all todos
    GET -> http://localhost:8080//todos

    create new TODO
    POST -> http://localhost:8080/todos

        needed: 
        -Content-Type: application/json

        {
            "title": "name: string",
            "procent": {procent: int},
            "deadline": "date: string"
        }

    edit TODO
    PUT -> http://localhost:8080/todos/{id}

        needed: 
            -Content-Type: application/json

            {
                "title": "name: string",
                "procent": {procent: int},
                "deadline": "date: string"
            }

    delete TODO
    DELETE -> http://localhost:8080/todos/{id}
