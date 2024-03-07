#!/bin/sh

# 设置在出现非零退出代码时停止脚本执行
set -e

# 获取当前执行目录
save_dir="/usr/local/sbin"
current_dir="${save_dir}/CAIE_Code"
loggedInUser=$(/bin/ls -l /dev/console | /usr/bin/awk '{ print $3 }')

# 从远程下载最新版本
git clone https://github.com/iewnfod/CAIE_Code.git $save_dir

# 如果git config失败，则返回退出代码1
git config --global --add safe.directory $current_dir || exit 1

# 如果chown失败，则返回退出代码1
chown -R $loggedInUser $current_dir || exit 1

# 链接到 bin 目录
ln -sf ${current_dir}/bin/cpc /usr/local/bin/cpc || exit 1

# 链接到 man 目录，如果失败则返回退出代码1
mkdir -p /usr/local/share/man/man1 && ln -f ${current_dir}/man/cpc.1 /usr/local/share/man/man1/cpc.1 || exit 1
