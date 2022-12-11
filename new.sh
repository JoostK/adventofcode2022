SESSION=$(cat session.txt)

if [ -z "$SESSION" ]; then
  echo "Please write your AoC session to session.txt"
  exit 1
fi

# Read the day from the first argument or default to today's day.
DAY=${1:-"$(date +%-d)"}
if [ -z "$DAY" ]; then
  echo "Missing day argument"
  exit 1
fi

PROJECT="day$DAY"
if [ -d "$PROJECT" ]; then
  echo "Day $DAY already exists"
  exit 1
fi

cp -R template/ "$PROJECT"
sed -i '' "s/template/$PROJECT/" "$PROJECT/Cargo.toml"
sed -i '' "s/template/$PROJECT/" "$PROJECT/src/main.rs"
sed -i '' "s/template/$PROJECT/" "$PROJECT/benches/benchmark.rs"
sed -i '' "s/\# NEW/\"$PROJECT\",\n    # NEW/g" Cargo.toml
cargo build
#curl --cookie "session=$SESSION" "https://adventofcode.com/2022/day/$DAY/input" > "$PROJECT/src/input.txt"
echo "Open https://adventofcode.com/2022/day/$DAY"
