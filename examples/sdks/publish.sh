#!/bin/bash

# 发布脚本
set -e

echo "正在发布监控SDK..."
cd vc-monitor-sdk/pkg
if npm publish --access=public; then
    echo "监控SDK发布成功！"
else
    echo "监控SDK发布失败！"
    exit 1
fi

echo "正在发布座席SDK..."
cd ../../vc-agent-sdk/pkg
if npm publish --access=public; then
    echo "座席SDK发布成功！"
else
    echo "座席SDK发布失败！"
    exit 1
fi

echo "所有SDK发布成功！"
