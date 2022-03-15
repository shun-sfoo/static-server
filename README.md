# static-server

a static server written by rust and react

## build

### change yarn mirror (option)

```bash
yarn global add yrm
yrm use taobao
```

### front

```bash
cd my-app
yarn install
echo REACT_APP_API_URL=http://local_ip_address:port > .env
yarn build
```

### backend

`cargo build --release`
