#!/user/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install --version='~0.6' sqlx-cli --no-default-features --features rustls,postgres"
  echo >&2 "to install it."
  exit 1
fi

# 유저 설정: 커스텀 설정 없으면 기본값 - postgres
DB_USER="${POSTGRES_USER:=postgres}"
# 비밀번호 설정: 커스텀 설정 없으면 기본값 - password
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# DB 이름 설정: 커스텀 설정 없으면 기본값 - newsletter
DB_NAME="${POSTGRES_DB:=newsletter}"
# 포트 설정: 커스텀 설정 없으면 기본값 - 5432
DB_PORT="${POSTGRES_PORT:=5432}"

if [[ -z "${SKIP_DOCKER}" ]]
then
  # 도커 사용해서 postgres 구동
  docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d \
    --name "postgres_$(date '+%s')" \
    postgres -N 1000
    # ^ 테스트용으로 최대로 증가시킨 커넥션 수
fi

# Postgres가 명령어를 받을 수 있을 때까지 계속 확인(ping)한다
until PGPASSWORD="${DB_PASSWORD}" psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"