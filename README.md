# hello-rust

1. main() 监听用户输入的命令；
2. 扔给子线程做任务lib.rs 
  do_command();
3. 监听线程和子线程通过一个channel来发送消息；
4. 发送exit，则两个线程退出；

