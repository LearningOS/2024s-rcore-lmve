# 实验 2 内存管理
虚拟内存的开启 (配置) satp CSR register，来使能 MMU 启用分页模式。在此之前所有的地址都是物理地址，而 MMU 启用分页模式后，所有的地址都是虚拟地址。在此机制下 虚拟地址（39 bit）将被 MMU 转换为物理地址（56 bit），若失败则会触发异常。

什么是 SV39 分页模式？
页表的数据结构
```
63               54  53          28 27          19 18         9 9            8  7   6    5     4     3     2     1     0
|   reserved 10 bit |  PPN2 26 bit |  PPN1 9 bit |  PPN0 9 bit |  RSW  2 bit  | D | A |  U  |  G  |  X  |  W  |  R  |  V  |
```
高 44 bits 是物理页号， 低 8 bits 是标志位。