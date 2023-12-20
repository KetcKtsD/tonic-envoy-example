## tonic + envoy

```shell
git submodule update --init --recursive
docker compose pull prereqs envoy commonjs-client
docker compose up tonic-server envoy commonjs-client
```

Open a browser tab, and visit http://localhost:8081/echotest.html.
