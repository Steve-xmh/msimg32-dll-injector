# msimg32-dll-injector

一个近乎通用的劫持 msimg32.dll 实现自定义 dll 加载的加载器，支持 `x86`/`x86_64` 架构。（ARM 暂不支持）

默认将会搜索当前工作目录下的 `inject.txt` 来确认需要注入的 DLL，然后依次调用 `LoadLibraryW` 函数加载 DLL。

[注入方法参考](https://www.pediy.com/kssd/pediy12/131397.html)
