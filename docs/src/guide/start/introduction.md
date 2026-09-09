# Introduction

## What is Waveless?

_Waveless_ is an extensible config-driven framework for building microservices.

A handful number of these frameworks already exists, however many of them require building a complete backend stack, also a handful number of them might not be optimized nor fast (e.g. LLM generated codebases are not guaranted to be fast, not even correct), Waveless aims to provide a solid foundation to any kind of microservice application, leveraging a solid extensible API any _executor_, database connector, authentication method could be implemented and managed directly from TOML files.

> At the moment, `waveless_sql` provides MySQL and Postgres support which includes: query executor, database connection, endpoint generator (MySQL only) and authentication implementations (MySQL only).

Endpoints are defined in TOML files, these could be set to handle regular HTTP requests, SSE and even WebSockets connections. The execution atom is called _executor_, each endpoint has a pipeline composed of an _executor_ tree which enables modeling complex behaviour.

> Consider this case, let $E$ be an arbitrary endpoint composed of $e_1$, $e_2$, $e_3$ and $e_4$, we define $Children(E_i) = {\text{all children executors of }E_i}$. We define the following structure:
> $Children(e_1) = {e_2}$, $Children(e_2) = {e_3, e_4}$. The entry executor is $e_1$ which decides to which children delegate further execution or even returns the response early without delegating the request.

```viz
digraph ExecutorFlow {
    rankdir=TB;
    splines=true;
    node [fontname="Helvetica", style="rounded"];
    edge [fontname="Helvetica"];

    Start   [shape=circle, label="Incoming request"];
    e1      [shape=diamond, label=< <I>e</I><SUB>1</SUB><BR /><FONT POINT-SIZE="10">(entry)</FONT> >];
    e2      [shape=diamond, label=< <I>e</I><SUB>2</SUB> >];
    e3      [shape=box,     label=< <I>e</I><SUB>3</SUB> >];
    e4      [shape=box,     label=< <I>e</I><SUB>4</SUB> >];
    Response [shape=circle, label="Response"];

    Start -> e1;

    e1 -> e2 [label=" delegate ", penwidth=2];
    e1 -> Response [label=" early return ", style=dashed, color=red, fontcolor=red];

    e2 -> e3 [label=" delegate ", penwidth=2];
    e2 -> e4 [label=" delegate ", penwidth=2];
    e2 -> Response [label=" early return ", style=dashed, color=red, fontcolor=red];

    e3 -> Response [style=dashed, color=blue];
    e4 -> Response [style=dashed, color=blue];

    Note1 [shape=note, label=< <I>Children</I>(<I>e</I><SUB>1</SUB>) = { <I>e</I><SUB>2</SUB> } >, fontcolor=purple, fillcolor="white", fontsize=12];
    Note2 [shape=note, label=< <I>Children</I>(<I>e</I><SUB>2</SUB>) = { <I>e</I><SUB>3</SUB>, <I>e</I><SUB>4</SUB> } >, fontcolor=purple, fillcolor="white", fontsize=12];

    {rank=same; e1; Note1;}
    {rank=same; e2; Note2;}
    e1 -> Note1 [style=invis];
    e2 -> Note2 [style=invis];
}
```

Simply ask any LLM to build a web service with Waveless which is fast and safe by default instead of letting it write a full-blown project from scratch (prone to errors and not always the most performant).

## Reasoning about the compiler-executor architecture

A Waveless project consists of a main `config.toml` and an `endpoints` folder where all the endpoint definitions are located, you might have multiple endpoints files each defining multiple endpoints at once. When you create a new project these files are automatically generated with sensible defaults.

> When generating endpoints, all resulting endpoints will be serialized into `.generared_endpoints`, this might be useful to copy and modify them.

Waveless' CLI has two modes of operation, you can directly run the project with `$ waveless run` or you can _build_ the project to generate a _Waveless' object file_ (`$ waveless build`), this file contains the entire project in a compact file, it also generates a checksum of the generated endpoints on build and refuses execution if the checksum changes on next run, this is useful to quickly deploy entire projects. Simply run an object file with `$ waveless executor run <PATH>`.

> Building the project might also be useful in situations where you need to ship all executors in a single file (when possible). For example, WASM executors might be embedded directly in the object file.

## Extensibility

To make endpoint definition as generic and flexible as possible Waveless offers these mechanisms to extend the the core functionality. Currently you can add:

- Database connectors (currently only MySQL and Postgres backends are implemented).
- Endpoint generators - from simple schema discovery generation to complex LLM pipelines (soon...)
- Execution steps (executors) - handles requests (currently only MySQL and Postgres executors are implemented), in the future simple scripting and a WASM VM will be possible, currently you could write custom request handlers by implementing the `waveless_commons::http_executor::AnyHttpExecutor` trait and loading it into the binary (more on that later).
- Authentication mechanisms - currently have a simple role based email-password authentication (with session tokens) (implemented on the MySQL database connector).

### Loading custom components - strategy overview

Waveless is made from the ground-up to be modular and generic offering multiple possibilities for loading foreign components each method offering a balance between speed and complexity. We propose the following approaches:

1. Dynamic library loading: the least secure by far yet the easiest, you compromise memory safety, where a malicious component or a vulnerable one might compromise the main runtime's memory or crashing the whole application.
2. IPC mechanisms (**PoC**) - instead of loading foreign code into the main process, _Waveless'_ foreign components implement an interface for IPC, where the main runtime spawns worker for each component, even internal ones.
    - This is useful in situations where a request handler might crash with malformed requests, but as it's running as a worker, the main runtime is still running which respawns the crashed worker.
3. WASM virtual machine for creating executors (**PoC**).
4. Using _Waveless_ as a library: brings the _Waveless_ executor to your application, define all the _Waveless'_ components directly in your application and integrate an API with your code, without setting up complex authentication, database connection management or routing logic.
