SCRIPT_PATH="$(cd "$(dirname "$0")" >/dev/null 2>&1 && pwd)"
cd "$SCRIPT_PATH/.." || exit
echo "pwd=$(pwd)"
./bin/init_db.sh
cargo run --release
