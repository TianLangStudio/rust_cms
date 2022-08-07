SCRIPT_PATH="$(cd "$(dirname "$0")" >/dev/null 2>&1 && pwd)"
cd "$SCRIPT_PATH/.." || exit
echo "pwd=$(pwd)"
echo "run mysql use docker"
docker run --name rust_cms_mysql --rm -p 23306:3306  -e MYSQL_ROOT_PASSWORD=rust_cms -d mysql:5.7 --character-set-server=utf8mb4 --collation-server=utf8mb4_unicode_ci
sleep 3
mysql -uroot -prust_cms -P23306 -h127.0.0.1 < ./doc/db/rust_cms.sql
