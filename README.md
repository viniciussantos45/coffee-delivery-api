# Rust Coffee API (English)

This is a simple coffee shop application built with Rust and the Axum web framework. The application allows users to create orders, view their orders, and manage coffee items.

## Getting Started

To get started with the application, you'll need to have Rust and Cargo installed on your system. You can install Rust and Cargo by following the instructions on the official Rust website.

Once you have Rust and Cargo installed, you can clone the repository and run the following command to start the application:

```shell
cargo run
```

This will start the application on <http://localhost:3333>.

## API Endpoints

The following API endpoints are available in the application:

- `GET /`: Returns a list of coffee items.
- `POST /users/create`: Creates a new user.
- `POST /orders/create`: Creates a new order.
- `GET /orders/my_orders`: Returns a list of orders for the authenticated user.
- `POST /session`: Authenticates a user and returns a JWT token.

## Database

The application uses a PostgreSQL database to store coffee items, users, and orders. You'll need to have PostgreSQL installed on your system and create a database named `coffee_shop` before running the application.

You can create the database by running the following command:

```shell
createdb coffee_shop
```

## Authentication

The application uses JWT tokens for authentication. To authenticate a user, you'll need to send a POST request to the `/session` endpoint with the user's email and password in the request body. The endpoint will return a JWT token, which you can use to authenticate subsequent requests.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Code Structure

- The `main.rs` file contains the entry point of the application. It defines the server and the API endpoints.
- The `handlers` directory contains the handler functions for the API endpoints. Each handler function is responsible for handling a specific API endpoint.
- The `models` directory contains the data models used by the application. These models are used to represent coffee items, users, and orders.
- The `schema` directory contains the database schema definitions used by the application. These schema definitions are used to create and modify the database tables.

## Render upload

Using Dockerfile for generate image to deply my api in render platform

</br>
</br>
</br>

# Rust Coffee API (Português)

Este é um simples aplicativo de café construído com Rust e o framework web Axum. O aplicativo permite que os usuários criem pedidos, visualizem seus pedidos e gerenciem itens de café.

## Começando

Para começar a usar o aplicativo, você precisará ter Rust e Cargo instalados em seu sistema. Você pode instalar o Rust e o Cargo seguindo as instruções no site oficial do Rust.

Depois de ter o Rust e o Cargo instalados, você pode clonar o repositório e executar o seguinte comando para iniciar o aplicativo:

```shell
cargo run
```

Isso iniciará o aplicativo em <http://localhost:3333>.

## Pontos de Extremidade da API

Os seguintes pontos de extremidade da API estão disponíveis no aplicativo:

- `GET /`: Retorna uma lista de itens de café.
- `POST /users/create`: Cria um novo usuário.
- `POST /orders/create`: Cria um novo pedido.
- `GET /orders/my_orders`: Retorna uma lista de pedidos para o usuário autenticado.
- `POST /session`: Autentica um usuário e retorna um token JWT.

## Banco de Dados

O aplicativo usa um banco de dados PostgreSQL para armazenar itens de café, usuários e pedidos. Você precisará ter o PostgreSQL instalado em seu sistema e criar um banco de dados chamado `coffee_shop` antes de executar o aplicativo.

Você pode criar o banco de dados executando o seguinte comando:

```shell
createdb coffee_shop
```

## Autenticação

O aplicativo usa tokens JWT para autenticação. Para autenticar um usuário, você precisará enviar uma solicitação POST para o ponto de extremidade /session com o e-mail e a senha do usuário no corpo da solicitação. O ponto de extremidade retornará um token JWT, que você pode usar para autenticar solicitações subsequentes.

## Licença

Este projeto está licenciado sob a Licença MIT. Veja o arquivo LICENSE para detalhes.

## Estrutura do Código

- O arquivo `main.rs` contém o ponto de entrada do aplicativo. Ele define o servidor e os pontos de extremidade da API.
- O diretório `handlers` contém as funções de tratamento para os pontos de extremidade da API. Cada função de manipulação é responsável por tratar um ponto de extremidade específico da API.
- O diretório `models` contém os modelos de dados usados pelo aplicativo. Esses modelos são usados para representar itens de café, usuários e pedidos.
- O diretório `schema` contém as definições do esquema do banco de dados usadas pelo aplicativo. Essas definições de esquema são usadas para criar e modificar as tabelas do banco de dados.
