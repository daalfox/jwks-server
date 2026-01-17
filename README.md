# JWKS server

A minimal server built to just host a single JWKS file on a static route

# How to use

```bash
docker run -d -ti -p 9090:80 -v ./jwks:/jwks:ro daalfox/jwks-server:latest
```

note: Make sure your jwks file is named `jwks.json` and mounted currectly
