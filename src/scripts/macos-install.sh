#!/bin/sh

# 设置在出现非零退出代码时停止脚本执行
set -e

# 获取当前执行目录
username=$(/bin/ls -l /dev/console | /usr/bin/awk '{ print $3 }')
current_dir="/Users/${username}/.cpc"

# 删除旧目录，准备下载最新版本
rm -rf ${current_dir}
mkdir -p ${current_dir}

# 检查与GitHub连通性
if curl --connect-timeout 3 https://api.github.com/ > /dev/null 2>&1; then
    repo_url="https://github.com/iewnfod/CAIE_Code.git"
else
    repo_url="https://gitee.com/ricky-tap/CAIE_Code.git"
fi

# 从选定的远程仓库下载最新版本
git clone --depth=1 ${repo_url} ${current_dir} || exit 1

# git config 设置
git config --global --add safe.directory ${current_dir} || exit 2

# 权限设置
chown -R ${username} ${current_dir} || exit 3

# 链接到 bin 目录
ln_script="ln -sf ${current_dir}/bin/cpc /usr/local/bin/cpc || exit 4"
# 链接到 man 目录
man_script="mkdir -p /usr/local/share/man/man1 && ln -f ${current_dir}/man/cpc.1 /usr/local/share/man/man1/cpc.1 || exit 5"

osascript <<EOF
do shell script "${ln_script}" with administrator privileges
do shell script "${man_script}" with administrator privileges
EOF
