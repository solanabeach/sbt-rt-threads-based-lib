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


have to benchmark those two approaches. In general we can leverage concurrency by partioning the load(here, blocks) into further pieces and moving the work associated with each piece to a thread.
contentious data structures will get in our way when syncronizing between these pieces.
 
For each piece we can try to return a separate structure and have an integrating mechanism at the end. The separate structures accumulating local results introduce a memory footprint, which we might or might not want to have.


* Refer to possible synchronization strategies: https://miro.com/app/board/uXjVOnfu7j4=/ 


# possible synchronization scenarios:


Isolated tasks that can be embarassingly parallelized:

- _n_ threads receive a task and rseturn to the parent on completion: simple scenario explored in account statistics module.

Consumer pool:

- Thread pool of size _p_ where _p_ is the number of Kafka partitions(we're aiming for 64 atm) and each thread is a Kafka consumer assigned to a given partition. Each thread has a `sender` half of the `mpmc` channel of which the _channel_-consumers are chains of processing and analytics. It would be extremely nice to only pass down to the subsequent workerks a reference that is lifetimed on the completion of the last actor in the chain.


