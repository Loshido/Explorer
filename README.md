# A self hosted web server that imitates FTP

It allows you to access your files from your phone or any device anywhere.
Originally built to enhance the FTP experience & provide a lightweight software (14mb) to access a distant computer (my home server).

## Env variables

| Variable | Default | What? |
| --- | --- | --- |
| `ROCKET_ADDRESS` | `0.0.0.0` | interface binding |
| `ROCKET_PORT` | `80` | port binding |
| `JWT_SECRET` | ... | JWT signing secret |
| `AUTH_VERSION` | `1` | changing version allows to invalidate tokens |
| `PASS_FILE` | `./pass` | Where the password are located |
| `COOKIE_DOMAIN` | `localhost` | Domain of the app |
| `COOKIE_SECURE` | `no` | Whether the app is under TLS (yes/no) |
| `EXPLORER_DATA` | `./data` | Where is the data located |

Be aware that for most usecase this app is contenerized, 
you may not need to change `EXPLORER_DATA` variable.

### PASS_FILE syntax

The file must follow this pattern

```
xxxxx-xxxxx-xxxxx
xxxxx-xxxxx-xxxxx
xxxxx-xxxxx-xxxxx
xxxxx-xxxxx-xxxxx
...
```
one key per line, 17 characters each including separators.

## Docker

`docker build -t explorer .`\
`docker run -d -p 80:80 -e JWT_SECRET=??? --name explorer explorer`

![login](./img/login.png)
![folder](./img/folder.png)
![not found](./img/404.png)