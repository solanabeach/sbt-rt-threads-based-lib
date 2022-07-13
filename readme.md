threads, thread-safety blogs and resources:
- https://onesignal.com/blog/thread-safety-rust/
- https://users.rust-lang.org/t/how-to-share-a-hashmap-between-threads/51791/12
- https://github.com/xacrimon/dashmap


------



The given problem is to process the whole ledger as quickly as possible by ingesting blocks at maximum capacity (and possibly extracting some data from each)

Concurrency can be leveraged at multiple levels (we should benchmark all): 
- process _n_ blocks concurrently via n threads (thread)  *
- process a block's transactions concurrently (threads/block)


----

Particular problem at hand is processing blocks and accumulating statistics over accounts over a long interval of blocks. assuming block/thread.

- global hashmap with per-account data K=address V=accountprofile

It is not obvious to me whether to

- pass a top-level hashmap behind an arc'ed rwlock to each thread that would update it with the results of its block. for this case, the more threads we use the higher is the contention, so i guess no -- every ix will want to grab the lock.


- accumulate a local hashmap per block and return it from the thread, merge at the end 

- just return a vector of accountProfile structures from the thread and merge it into a global hashmap


have to benchmark those two.


* Refer to possible synchronization strategies: https://miro.com/app/board/uXjVOnfu7j4=/ 
