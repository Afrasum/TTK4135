## Exercise 1 - Theory questions

### Concepts

What is the difference between _concurrency_ and _parallelism_?

> Parallelism: excectuing two task at the same time on multiple cpu cores
> Concurrency: multiple tasks are in progress at the same time, but may be done on only one cpu using f.eks context switching

What is the difference between a _race condition_ and a _data race_?

> Race condition: when a programs correctness depends on the timing or ordering of threads/events
> Data race: a subset of race conditions, when a race conditions occurs because of shared memory between threads without proper syncronization

_Very_ roughly - what does a _scheduler_ do, and how does it do it?

> Decides which task/process/thread to run next and for how long

### Engineering

Why would we use multiple threads? What kinds of problems do threads solve?

> Responsiveness, especially during slow work
> Parallelism: use multiple cores for more speed
> I/O overlap: do work while waiting

Some languages support "fibers" (sometimes called "green threads") or "coroutines"? What are they, and why would we rather use them over threads?

> Fibers/green threads/coroutines are lightweight execution units scheduled by runtime and not OS
> Cooperative or semi cooperative (they yield explicity, as they cant use hardware timers etc)
> 1 000 - 1 000 000 can exists
> Context switching happens in user space, cheap

Does creating concurrent programs make the programmer's life easier? Harder? Maybe both?

> Makes it harder during development but easier the moment you start testing and using lol

What do you think is best - _shared variables_ or _message passing_?

Depends on usage

> Best for low level, performance critcal and small well-scoped shared state code: shared variables - fast, natural, fine-grained control, but risk of race conditions, deadlocks etc
> Best for concurrent systems, servers, fault-tolerant systems and high level application logic: messaging - fewer races, easier reasoning, better modularity/scalability, work well with fibers/coroutines, but at the cost of overhead and latency
