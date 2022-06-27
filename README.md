# XYB Signer

校友邦自动签到（**仅为学习目的开发，使用者自行承担可能的后果**）

## 使用方法

根据从[Release](https://github.com/M4tsuri/xyb_signer/releases)页面下载对应的程序并解压。

按照`config_template.txt`中的内容编辑配置文件，双击**同一目录下**的可执行文件即可。可以设置定时任务每天触发。

初次使用需要进行一定的交互，大致如下：

```
[INPUT] Do you want to set a password? [Y/n]: y
[INPUT] Please input the verify code you received: 730717
[INPUT] Please input your new password: *******
[INFO] Password successfully reset.
[INFO] Login success.
[INFO] New sessionid saved.
[INFO] Project list retrieved: 
        0. project 1
        1. project 2
[INPUT] Choose a project: 0
[INFO] Retrieving traineeId for project...done (id = 114514)
[SUCCESS] successfully signed.
```

初次设置完成后不要修改`config.json`的内容，以后使用直接运行即可。

## 定时任务设置方法

在设置定时任务前，请确保你手动使用该脚本签到成功过。

### Windows

参见 https://www.cnblogs.com/sui776265233/p/13602893.html

注意“起始于”设置为`config.json`所在路径，定时每天触发。

### MacOS & Linux

使用crontab，假设你将压缩包解压到了路径$XYB

命令行执行

```
crontab -e
```

弹出的页面输入`i`然后输入

```
0 1 1-31 6-7 MON-FRI cd $XYB && ./xyb_signer
```

输入`:wq`回车即可。
