// Use `go run foo.go` to run your program

package main

import (
	. "fmt"
	"runtime"
)

func incrementing(c chan int, done chan bool) {
	//  increment i 1000000 times
	for j := 0; j < 999999; j++ {
		c <- 1
	}

	done <- true
}

func decrementing(c chan int, done chan bool) {
	//  decrement i 1000000 times
	for range 1000000 {
		c <- -1
	}

	done <- true
}

func server(c chan int, result chan int) {
	i := 0
	for x := range c {
		i += x
	}
	result <- i
}

func main() {
	// What does GOMAXPROCS do? What happens if you set it to 1?
	runtime.GOMAXPROCS(2)

	c := make(chan int)
	done := make(chan bool, 2)
	result := make(chan int)

	//  Spawn both functions as goroutines
	go server(c, result)

	go incrementing(c, done)
	go decrementing(c, done)

	<-done
	<-done

	close(c)

	i := <-result

	Println("The magic number is:", i)
}
