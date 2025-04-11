## 编程
在 disk_inode 增加一个 ref_count 字段，之所以不加在 inode，感觉 inode 每次 find 都会返回新的 inode， inode 只适合只读？

但是我想知道有没有方法把 ref_count 和 inode 放在一起。为什么 inode 每次要 new 一个新的？

如果 link 就加上 ref count。

对于 unlink 需要更新目录 dirent，因为不知道怎么缩小 disk node size，所以直接写了个空的来覆盖之前的 dirnet。

## 问答 ch6
在我们的easy-fs中，root inode起着什么作用？如果root inode中的内容损坏了，会发生什么？

是主目录，如果损坏了就读不了任何文件了...

## 问答 ch7
举出使用 pipe 的一个实际应用的例子。

ls | grep xxx 这样的命令，把一个命令的结果传递给下一个命令作为参数。

如何使用 cat 和 wc 完成一个文件的行数统计？

cat xxx | wc -l

如果需要在多个进程间互相通信，则需要为每一对进程建立一个管道，非常繁琐，请设计一个更易用的多进程通信机制。

可以弄一片共享的区域。