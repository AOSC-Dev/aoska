#!/bin/bash

# aoska_maintainer 便利腳本

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
AOSKA_MAINTAINER="$SCRIPT_DIR/src-tauri/target/debug/aoska_maintainer"
MOCK_DATA_DIR="$SCRIPT_DIR/src-tauri/mock_data"

# 確保 mock_data 目錄存在
mkdir -p "$MOCK_DATA_DIR"

# 確保工具已編譯
if [ ! -f "$AOSKA_MAINTAINER" ]; then
    echo "正在編譯 aoska_maintainer..."
    cd "$SCRIPT_DIR/src-tauri"
    cargo build --bin aoska_maintainer
    cd - > /dev/null
fi

# 展示使用方法
show_usage() {
    echo "使用方法:"
    echo "  $0 index <input.toml> [output.json]     # 生成主索引文件（默認輸出到 src-tauri/mock_data/aoska_index.json）"
    echo "  $0 recommend <input.toml> [output.json] # 生成推薦索引文件（默認輸出到 src-tauri/mock_data/recommend_index.json）"
    echo "  $0 package <input.toml> <pkg_name> [output.json]   # 生成軟件包詳細信息文件（默認輸出到 src-tauri/mock_data/packages/<pkg_name>/meta.json）"
    echo ""
    echo "示例:"
    echo "  $0 index examples/index.toml"
    echo "  $0 recommend examples/recommend.toml"
    echo "  $0 package examples/package_firefox.toml firefox"
    echo "  $0 package examples/package_firefox.toml firefox custom_meta.json  # 自定義輸出文件名"
}

# 檢查參數
if [ $# -lt 2 ]; then
    show_usage
    exit 1
fi

# 對於 package 類型，需要額外的 pkg_name 參數
if [ "$1" = "package" ] && [ $# -lt 3 ]; then
    echo "錯誤: package 類型需要指定 pkg_name 參數"
    show_usage
    exit 1
fi

TYPE="$1"
INPUT="$2"

# 根據類型處理參數
if [ "$TYPE" = "package" ]; then
    PKG_NAME="$3"
    OUTPUT="$4"
else
    OUTPUT="$3"
fi

# 檢查輸入文件是否存在
if [ ! -f "$INPUT" ]; then
    echo "錯誤: 輸入文件 '$INPUT' 不存在"
    exit 1
fi

# 設置默認輸出文件名（如果沒有提供）
if [ -z "$OUTPUT" ]; then
    case "$TYPE" in
        "index")
            OUTPUT="$MOCK_DATA_DIR/aoska_index.json"
            ;;
        "recommend")
            OUTPUT="$MOCK_DATA_DIR/recommend_index.json"
            ;;
        "package")
            # 為軟件包創建專用目錄
            PKG_DIR="$MOCK_DATA_DIR/packages/$PKG_NAME"
            mkdir -p "$PKG_DIR"
            OUTPUT="$PKG_DIR/meta.json"
            ;;
        *)
            echo "錯誤: 未知的類型 '$TYPE'"
            show_usage
            exit 1
            ;;
    esac
else
    # 如果提供了輸出文件名，但不是絕對路径，則放到對應目錄
    if [[ "$OUTPUT" != /* ]]; then
        if [ "$TYPE" = "package" ]; then
            PKG_DIR="$MOCK_DATA_DIR/packages/$PKG_NAME"
            mkdir -p "$PKG_DIR"
            OUTPUT="$PKG_DIR/$OUTPUT"
        else
            OUTPUT="$MOCK_DATA_DIR/$OUTPUT"
        fi
    fi
fi

# 根據類型調用相應命令
case "$TYPE" in
    "index")
        echo "正在生成主索引文件到: $OUTPUT"
        "$AOSKA_MAINTAINER" generate-index -i "$INPUT" -o "$OUTPUT"
        ;;
    "recommend")
        echo "正在生成推薦索引文件到: $OUTPUT"
        "$AOSKA_MAINTAINER" generate-recommend -i "$INPUT" -o "$OUTPUT"
        ;;
    "package")
        echo "正在生成軟件包詳細信息文件到: $OUTPUT"
        "$AOSKA_MAINTAINER" generate-package -i "$INPUT" -o "$OUTPUT"
        ;;
    *)
        echo "錯誤: 未知的類型 '$TYPE'"
        show_usage
        exit 1
        ;;
esac
