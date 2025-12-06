cargo build --release
echo "\n"

for day in $(ls src/day*.rs | sed 's|[^0-9]||g' | sort -n); do
  echo "DAY $day"
  cargo run --release --quiet --bin day$day 2>/dev/null
  echo "\n"
done
