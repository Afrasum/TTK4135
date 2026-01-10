# Sharing a variable - C and Go

## C

To explain what happens, consider the following output from running the same C program multiple times:

```bash
❯ ./foo
The magic number is: 120222
❯ ./foo
The magic number is: -326891
❯ ./foo
The magic number is: 133156
❯ ./foo
The magic number is: 195746
❯ ./foo
The magic number is: 343543
❯ ./foo
The magic number is: 396305
```

The result varies between runs and appears almost random, typically within the range −400 000 to 400 000. This occurs because both threads access and modify the same shared variable without synchronization, which leads to race conditions.

The operations i++ and i-- are not single instructions. Each consists of reading the current value of i, modifying it, and writing the result back to memory. If another thread modifies i between the read and write steps, that update can be overwritten. As a result, many increments and decrements are effectively lost.

Because this happens repeatedly during execution, the final value of i becomes non-deterministic and is rarely equal to 0.

## Go

### GOMAXPROCS(2)

When allowing Go to use two OS threads, we observe the following output:

```bash
❯ go run foo.go
The magic number is: -994940
❯ go run foo.go
The magic number is: -997452
❯ go run foo.go
The magic number is: 256928
❯ go run foo.go
The magic number is: 407584
❯ go run foo.go
The magic number is: 704124
❯ go run foo.go
The magic number is: -327449
❯ go run foo.go
The magic number is: -442276
❯ go run foo.go
The magic number is: 962174
❯ go run foo.go
The magic number is: 1000000
```

This behavior is essentially the same as in C. Two goroutines operate on the same shared variable i without synchronization. The operations i++ and i-- are not atomic and consist of reading the value, modifying it, and writing it back.

With GOMAXPROCS(2), Go can execute goroutines in parallel on two OS threads. This leads to race conditions where updates are lost, producing non-deterministic results similar to those seen in the C implementation. The large absolute values indicate that one goroutine often makes significant progress before the other gets much CPU time.

### GOMAXPROCS(1)

When setting GOMAXPROCS(1), the output becomes:

```bash
❯ go run foo.go
The magic number is: 0
❯ go run foo.go
The magic number is: 0
❯ go run foo.go
The magic number is: 0
❯ go run foo.go
The magic number is: 0
❯ go run foo.go
The magic number is: 0
❯ go run foo.go
The magic number is: 0
❯ go run foo.go
The magic number is: 0
```

With GOMAXPROCS(1), Go is restricted to a single OS thread. As a result, there is no true parallel execution, only cooperative scheduling of goroutines on one thread. In practice, the scheduler often allows one goroutine to run nearly to completion before switching to the other. This results in near-sequential execution, causing the increments and decrements to cancel out and producing a final value of 0.

The program still contains a data race, but the scheduling behavior in this configuration makes the result appear deterministic.
