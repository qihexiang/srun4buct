# SRUN4BUCT

一个用来辅助登录北化校园网的工具。

## 使用方法

### 安装依赖项目

本项目依赖于浏览器，支持Firefox、Chrome和Edge，其中Firefox和Chrome可以headless工作，适用于服务器和单片机环境。

针对不同的浏览器，需要准备好对应的驱动，如果要使用srun程序自动启动，应将其放置到PATH目录下，若不需要，则应该先手动启动驱动程序，然后使用srun-core来实现校园网登录和状态守护。

### 配置文件

配置文件应该放置于当前用户家目录下的`.srun.rs.json`文件中，示例如下：

```json
{
    "username": "2021200902",
    "password": "password here",
    "browser": "firefox",
    "driver": "http://localhost:4444",
    "interval": 300000,
    "headless": false,
    "waiting": [5000, 1000],
    "max_failed": 20
}
```

其中，driver是驱动程序启动后监听的路径，例如geckodriver启动后监听`http://localhost:4444`，若为chromedriver，则应该使用`http://localhost:9515`；interval是检查是否掉线的时间间隔，单位为毫秒；headless设置为true时，浏览器将以无图形界面的形式启动；waiting参数用于等待页面内容加载，其中第一个数字是等待的总时长，第二个数字是等待期间重新检查的间隔；max_failed是守护进程运行期间，可以发生错误的次数，当发生错误的次数大于该处指定数字时，程序会退出。

`srun`会根据配置文件自动从PATH启动一个新的WebDriver进程和一个`srun-core`进程，结束时应该会自动全部结束，而`srun-core`则会尝试连接到已经存在的服务。

## Todo

~~由于依赖项目中有一些-sys的crate，目前仍在想办法完成Windows/Linux平台的静态编译，若能顺利完成会发布二进制版本。~~

不搞了，搞不好，就动态链接吧。
