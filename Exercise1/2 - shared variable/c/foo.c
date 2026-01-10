// Compile with `gcc foo.c -Wall -std=gnu99 -lpthread`, or use the makefile
// The executable will be named `foo` if you use the makefile, or `a.out` if you
// use gcc directly

#include <pthread.h>
#include <stdio.h>

int i = 0;

// Note the return type: void*
void *incrementingThreadFunction() {
  //  increment i 1_000_000 times
  for (int j = 0; j < 1000000; j++) {
    i++;
  }
  return NULL;
}

void *decrementingThreadFunction() {
  //  decrement i 1_000_000 times
  for (int j = 0; j < 1000000; j++) {
    i--;
  }
  return NULL;
}

int main() {
  // start the two functions as their own threads using `pthread_create`
  // Hint: search the web! Maybe try "pthread_create example"?
  pthread_t thread_1;
  pthread_create(&thread_1, NULL, incrementingThreadFunction, NULL);

  pthread_t thread_2;
  pthread_create(&thread_2, NULL, decrementingThreadFunction, NULL);

  // wait for the two threads to be done before printing the final result
  // Hint: Use `pthread_join`
  pthread_join(thread_1, NULL);
  pthread_join(thread_2, NULL);

  printf("The magic number is: %d\n", i);
  return 0;
}
