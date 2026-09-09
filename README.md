<h1 style="font-size:60px" align="center">Waveless</h1>
<h2 align="center">Build fast and type-safe microservices.</h2>

<br>

_Waveless_ is an extensible config-driven framework for building microservices.

A handful number of these frameworks already exists, however many of them require building a complete backend stack, also a handful number of them might not be optimized nor fast (e.g. LLM generated codebases are not guaranted to be fast, not even correct), Waveless aims to provide a solid foundation to any kind of microservice application, leveraging a solid extensible API any _executor_, database connector, authentication method could be implemented and managed directly from TOML files.

> At the moment, `waveless_sql` provides MySQL and Postgres support which includes: query executor, database connection, endpoint generator (MySQL only) and authentication implementations (MySQL only).

Endpoints are defined in TOML files, these could be set to handle regular HTTP requests, SSE and even WebSockets connections. The execution atom is called _executor_, each endpoint has a pipeline composed of an _executor_ tree which enables modeling complex behaviour.

For more details visit the project's website.
