# meow

Env init:
```
echo "AUTH_API_KEY=$(openssl rand -hex 64)" > web_secrets.env
```

Run:
```
docker compose up -d
```

backup (do it before every upgrade!):
```
docker compose exec -t postgres pg_dump --no-owner --no-privileges -c -U meow -d postgres > dump_`date +%Y-%m-%d"_"%H_%M_%S`.sql
```
