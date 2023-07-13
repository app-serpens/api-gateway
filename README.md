Linux
instalar openssl com a libssl-dev pra ubuntu
instalar postgresql na versão 14 de preferencia
instalar libpq-dev
sudo -i -u postgres
createuser --interactive
digite serpens
dê a permissão de superusuário
psql
alter role serpens with password 'serpens-dev';
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";