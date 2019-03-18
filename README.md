# hello-rust

1. main() 在主线程监听用户输入的命令；
2. 扔给子线程做任务lib.rs 
  do_command();
3. 主线程和子线程通过channel来告知是否消息，通过静态data存放命令；
4. 输入“exit”，则两个线程退出；

