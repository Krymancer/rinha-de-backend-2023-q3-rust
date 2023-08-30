# Rinha Backend 2023 Q3

Essa é a minha implementação da [Rinha de Backend 2023 Q3](https://github.com/zanfranceschi/rinha-de-backend-2023-q3/), realizada pelo [Zanfranceschi](https://twitter.com/zanfranceschi).

Você pode ver todas as instruções e a spec da API [aqui](https://github.com/zanfranceschi/rinha-de-backend-2023-q3/blob/main/INSTRUCOES.md).

## Tecnologias utilizadas

- [Rust](https://www.rust-lang.org/)
- [Axum](https://github.com/tokio-rs/axum)
- [SQLx](https://github.com/launchbadge/sqlx)
- [Postgres](https://www.postgresql.org/)
- [Docker](https://www.docker.com/) e [Docker Compose](https://docs.docker.com/compose/)
- [Nginx](https://nginx.org)
- [Github Actions](https://docs.github.com/pt/actions/learn-github-actions/understanding-github-actions)
- [Gatling](https://gatling.io/)

## Infraestrutura

Toda a aplicação está dividida em diferentes containers como você pode ver no arquivo [docker-compose.yml](./docker-compose.yml). A aplicação tem 4 serviços:

- `api1` e `api2` que são duas imagens da API que rodam em paralelo
- `nginx` que é o proxy reverso que faz o balanceamento de carga entre as duas APIs
- `db` que é o banco de dados Postgres

## Aviso

⚠️ O código deste repositório não deve ser utilizado como referência para ambientes de produção. Algumas práticas foram aplicadas especificamente em prol da competição, e podem não ser saudáveis para sua aplicação.

## Minhas Redes

- [Twitter](https://twitter.com/joaodocodigo)
- [Linkedin](https://www.linkedin.com/feed/)

## Agradecimentos

- [Leonardo Vargas](https://twitter.com/leorcvargas) pela ideaização da Rinha, foi um evento muito bacana e eu aprendi demais.

- [Zanfranceschi](https://twitter.com/zanfranceschi) por sua aplicação em go que me serviu de inspiração, como também ajuda na configuração do postgress e nginx.

- [Rodrigo Navarro](https://github.com/reu) pelo exemplo em utilizar rust e axum com sqlx que me fez ter vontande de voltar a aprender rust para participar da rinha.

- [Vinicius Fonseca](https://twitter.com/distanteagle16) também pelo exemplo de utilizar rust, e pela ajuda da modelagem do banco de dados, utilizando o [pgtrm](https://www.postgresql.org/docs/current/pgtrgm.html)
