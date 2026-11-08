#!/usr/bin/env bash
# 一键配置国内 Rust 开发环境（macOS / Linux 通用）
# 用法:
#   bash scripts/setup-rust-cn.sh              # 只配镜像
#   bash scripts/setup-rust-cn.sh --with-tools # 顺带装常用工具
#   MIRROR=ustc bash scripts/setup-rust-cn.sh  # 换镜像: rsproxy(默认) | ustc | aliyun | tuna | sjtu
set -euo pipefail

CARGO_HOME_DIR="${CARGO_HOME:-$HOME/.cargo}"
CONFIG="$CARGO_HOME_DIR/config.toml"
MIRROR="${MIRROR:-rsproxy}"
WITH_TOOLS=0
[[ "${1:-}" == "--with-tools" ]] && WITH_TOOLS=1

case "$MIRROR" in
  rsproxy) INDEX="sparse+https://rsproxy.cn/index/" ;;
  ustc)    INDEX="sparse+https://mirrors.ustc.edu.cn/crates.io-index/" ;;
  aliyun)  INDEX="sparse+https://mirrors.aliyun.com/crates.io-index/" ;;
  tuna)    INDEX="sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/" ;;
  sjtu)    INDEX="sparse+https://mirrors.sjtug.sjtu.edu.cn/crates.io-index/" ;;
  *) echo "未知 MIRROR: $MIRROR"; exit 1 ;;
esac

info() { printf "\033[1;34m==>\033[0m %s\n" "$*"; }
ok()   { printf "\033[1;32m  ✓\033[0m %s\n" "$*"; }
warn() { printf "\033[1;33m  !\033[0m %s\n" "$*"; }

# ---------- 0. 前置检查 ----------
if ! command -v cargo >/dev/null 2>&1; then
  echo "未找到 cargo，请先安装 Rust:"
  echo '  curl --proto "=https" --tlsv1.2 -sSf https://rsproxy.cn/rustup-init.sh | sh'
  exit 1
fi
CARGO_VER=$(cargo --version 2>/dev/null | awk '{print $2}') || true
if [[ -z "$CARGO_VER" ]]; then
  echo "cargo 存在但无法运行（rustup 没有默认 toolchain？），先执行: rustup default stable"
  exit 1
fi
info "cargo $CARGO_VER / $(uname -sm)"

# ---------- 1. 备份并写入 cargo config ----------
mkdir -p "$CARGO_HOME_DIR"
if [[ -f "$CONFIG" ]] && grep -q "rsproxy\|crates.io-index" "$CONFIG" 2>/dev/null; then
  cp "$CONFIG" "$CONFIG.bak.$(date +%Y%m%d%H%M%S)"
  warn "已有镜像配置，已备份后覆盖"
elif [[ -f "$CONFIG" ]]; then
  cp "$CONFIG" "$CONFIG.bak.$(date +%Y%m%d%H%M%S)"
  warn "已有自定义配置，已备份后覆盖（如需保留请手动合并）"
fi

cat > "$CONFIG" <<EOF
# 由 setup-rust-cn.sh 生成 — 镜像: $MIRROR
[source.crates-io]
replace-with = "mirror-sparse"

[source.mirror-sparse]
registry = "$INDEX"

[registries.mirror]
index = "$INDEX"

[net]
git-fetch-with-cli = true

[alias]
b  = "build"
r  = "run"
t  = "test"
c  = "check"
cl = "clippy --all-targets -- -D warnings"
f  = "fmt"
EOF
ok "已写入 $CONFIG (镜像: $MIRROR)"

# ---------- 2. rustup 镜像写进 .zshenv（GUI 进程也能读到） ----------
ZSHRC="$HOME/.zshrc"
ZSENV="$HOME/.zshenv"
touch "$ZSENV"
if ! grep -q "RUSTUP_DIST_SERVER" "$ZSENV" 2>/dev/null; then
  cat >> "$ZSENV" <<'EOF'

# rustup 国内镜像（由 setup-rust-cn.sh 追加）
export RUSTUP_DIST_SERVER=https://rsproxy.cn
export RUSTUP_UPDATE_ROOT=https://rsproxy.cn/rustup
EOF
  ok "已向 ~/.zshenv 追加 rustup 镜像变量"
else
  warn "~/.zshenv 已有 RUSTUP_DIST_SERVER，跳过"
fi
if grep -q "RUSTUP_DIST_SERVER" "$ZSHRC" 2>/dev/null; then
  warn "~/.zshrc 里也有 RUSTUP_DIST_SERVER，两处冲突时以 .zshenv 为准，建议删掉 .zshrc 里的两行"
fi

# ---------- 3. 系统依赖 ----------
if command -v brew >/dev/null 2>&1; then
  for pkg in cmake pkg-config; do
    if brew list "$pkg" >/dev/null 2>&1; then ok "$pkg 已安装"; else
      info "安装 $pkg ..."; brew install "$pkg" >/dev/null 2>&1 && ok "$pkg" || warn "$pkg 安装失败，可忽略（仅部分 -sys 包需要）"
    fi
  done
else
  warn "未检测到 Homebrew，跳过系统依赖（cmake/pkg-config 需自行安装）"
fi

# ---------- 4. 可选工具 ----------
if [[ "$WITH_TOOLS" == "1" ]]; then
  for t in cargo-edit cargo-watch; do
    if cargo install --list 2>/dev/null | grep -q "^$t "; then ok "$t 已安装"; else
      info "安装 $t ...(需要几分钟)"; cargo install "$t" --locked >/dev/null 2>&1 && ok "$t" || warn "$t 安装失败"
    fi
  done
fi

# ---------- 5. 验收 ----------
info "验收：新建临时项目并真实拉取依赖"
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT
START=$(date +%s)
VERIFY_FAILED=0
if ( set -e
     cd "$TMP"
     cargo new smoke --quiet
     cd smoke
     cargo add serde --features derive --quiet
     cargo fetch --quiet
   ); then
  END=$(date +%s)
  ok "依赖拉取耗时 $((END-START))s（正常 1~5s；>30s 说明镜像没生效）"
else
  END=$(date +%s)
  VERIFY_FAILED=1
  warn "验收失败（耗时 $((END-START))s），检查 $CONFIG 或网络连通性"
fi

if [[ -d "$CARGO_HOME_DIR/registry/index" ]]; then
  ok "索引目录已生成: $(ls -1 "$CARGO_HOME_DIR/registry/index" | head -3 | tr '\n' ' ')"
fi

echo
if [[ "$VERIFY_FAILED" == "0" ]]; then
  info "完成。开新终端或执行 'source ~/.zshenv' 后生效。"
else
  warn "配置已写入但验收未通过，请按上面提示排查后再试。"
fi
echo "  撤销: rm $CONFIG && source ~/.zshenv"
