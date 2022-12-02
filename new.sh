DAY=$1
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
sed -i '' "s/\# NEW/\"$PROJECT\",\n    # NEW/g" Cargo.toml
cargo build
curl --cookie "session=$AOC_SESSION" "https://adventofcode.com/2022/day/$DAY/input" > "$PROJECT/src/input.txt"
