### 函数接口文档
#### File
1. readable: 返回可读属性
2. writable: 返回可写属性
3. read(mut user_buf): user_buf获取输入信息, 返回1
4. write(user_buf): 打印user_buf信息，返回长度

#### TaskControlBlockInner
1. 新增fd_table: Vec<Option<Arc<dyn File + Send + Sync>>>

#### Syscall
1. sys_write(fd, buf, len)：fd为程序输入到那个文件，将文件的信息输出出来
2. sys_read(fd, buf, len)：将用户输入放到buf中
3. sys_openat(dirfd, path, sflags, mode)：用各种不同的读写方式打开文件

#### BlockDevice
1. read_block(block_id, &mut buf)：将blockid对应的块内容读到buf中
2. write_block(block_id, &buf)：将buf写到对应block

一个块512字节，512*8bit
#### BlockCache
1. 由512个8bit，block_id, block_device:Arc<dyn BlockDevice>和modified组成
2. addr_of_offset(offset)：返回offset位置的cache元素。
3. get_ref(offset)：获取offset位置的模板T的对应指针。
4. get_mut(offset)：同上但可以修改。
5. read(offset): get_ref。
6. modify(offset): get_mut。
7. drop：如果修改了将结果写到对应block中。

#### ClockCacheManager
1. 由Arc<Mutex<BlockCache>>组成的VecDeque
2. get_block_cache(block_id, block_device)：从VecDeque中获取对应的block_cache，如果queue满了则弹出强引用计数为1的cache然后存入queue，就是弹出自己再加入。

### easy-fs布局
#### SuperBlock
1. 由合法性验证的魔数magic，文件系统的总块数total_blocks，索引节点位图，索引节点区域，数据块位图，数据块区域组成。
2. initialize()：有磁盘块管理器传入初始化参数
3. is_valid():判断是否合法。

#### Bitmap
1. 由start_block_id和blocks组成，起始块编号和块数
2. alloc(block_device)：先得到bits64_pos和inner_pos为第blockid块磁盘中未完全分配，然后发现是这一块的第bits64_pos个64位没完全分配，再找到这个第64位的第一个0处于的比特位赋值为1，并返回这一位在整个Bitmap中从startblock一开始处于第几位。

#### DiskInode
1. 由文件字节数size，直接块编号，间接块编号组成。
2. initialze()，初始化。
3. is_dir(): 是否为目录。
4. is_file(): 是否为文件。
5. get_block_id(inner_id, block_device): 获取inner_id对应的文件存储的blockid。
6. data_blocks(): 得到一共占了多少数据块，向上取整。
7. total_blocks(): 数据块与索引块的总和。
8. blocks_num_needed(new_size): 得到从size扩容到new_size需要的额外数据块。
9. increase_size(new_size, new_blocks, block_Device): 扩容函数。
10. clear_size(block_Device): 清空文件内容，返回一个Vec重新送给磁盘块管理器进行调度。
11. read_At(offset, buf, block_device): 将文件内容从offset字节开始的部分读到buf中。返回读到的字节数。
12. write_at(): 同上实现但是需要increase_size。

#### DirEntry
1. 由目录名和inode_number组成。

#### EasyFileSystem
1. 由block_device, inode_bitmap, data_bitmap, inode_area_start_block和data_area_Start_block组成。
2. create(block_Device, total_blocks, inode_bitmap_blocks): 创建efs实例并且将第0快设置为超级块
3. open(block_device)：将编号位0的超级块读出然后构造efs实例。
4. get_disk_inode_pos(inode_id): 得到inode_id对应的数据块在第几块以及offset。
5. get_Data_block_id(data_block_id)：同上，得到数据块所在的实际位置
6. alloc_inode(), alloc_Data(): 返回alloc所在的数据块id。
7. dealloc_Data(block_id): 将cache中blockid的块清零并且释放block_id-data_Area_Start_id的空间。
8. root_inode(efs): 获取根目录的inode。

#### Inode
1. 由block_id, block_offset, fs文件系统， block_Device组成。
2. read_disk_inode()
3. modify_disk_inode()
4. find(name): 根据名字得到对应存inode的位置并且构造Inode。
5. find_inode_id(name, disk_inode): 对文件系统进行遍历查找取出每个DirEntry并得到相应的目录名与name进行比对，返回DirEntry的Inodenumber。
6. ls同上但是把每个dirent的名字记下来。
7. create(name)：先查有没有当前name的inode，然后alloc一个新inode并且得到存储inode需要的数据块id和偏移，然后将新文件目录项插入到根目录内容中。
8. clear(): 
9. read_at(offset, buf: &mut [u8]): disknode.read_at.
10. write_at同上，但需要扩容
11. increa_size(new_size, disk_inode, fs): disk_inode.increase_size。



