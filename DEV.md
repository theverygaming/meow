This is here so i don't forget, cursed setup i know

postgres:
```
nix-shell -p gnumake -p postgresql
make postgrescontainer
```

api:
```
cd api
nix-shell
DATABASE_URL=postgres://postgres:postgres@127.0.0.1/postgres cargo run
```

frontend:
```
cd frontend
nix-shell
npm install
npm run dev
```

caddy:
```
nix-shell -p caddy
caddy run --config Caddyfile
```
