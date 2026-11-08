#!/usr/bin/env bash
# Rust 学习示例启动器
#   ./run.sh              列出所有示例
#   ./run.sh 03           运行 03-ownership
#   ./run.sh 13 out.log   运行 13-mini-grep 并传入参数
set -euo pipefail
cd "$(dirname "$0")"

list_cases() {
  echo "可用示例："
  for d in [0-9][0-9]-*; do
    title=$(head -1 "$d/README.md" 2>/dev/null | sed 's/^# //')
    printf "  %-24s %s\n" "$d" "${title:-（无说明）}"
  done
  echo
  echo "用法: ./run.sh <编号|目录名> [传给程序的参数...]"
  echo "例如: ./run.sh 03      或   ./run.sh 13 error data/sample.log -i"
}

if [[ $# -eq 0 || "$1" == "list" || "$1" == "-h" || "$1" == "--help" ]]; then
  list_cases
  exit 0
fi

key="$1"
shift

dir=""
for d in [0-9][0-9]-*; do
  if [[ "$d" == "$key" || "$d" == "$key"-* ]]; then
    dir="$d"
    break
  fi
done

if [[ -z "$dir" ]]; then
  echo "没有找到示例 '$key'，下面是可以选的："
  echo
  list_cases
  exit 1
fi

echo "==> 运行 $dir"
exec cargo run --quiet --manifest-path "$dir/Cargo.toml" -- "$@"
