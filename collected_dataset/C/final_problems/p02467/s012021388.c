#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h> // uint64_t
#include <math.h>

#define BUF_SIZE 15

//sqrt(100000000)
#define NUM_MAX 31623

int get_int(void) {
  int num;
#ifdef BUF_SIZE
  char line[BUF_SIZE];
  if(!fgets(line, BUF_SIZE, stdin)) return 0;
  sscanf(line, "%d", &num);
#else
#error
#endif
  return num;
}

int main(void) {
    int n = get_int();
    int n1 = n;
    // less than NUM_MAX
    int max = ((int)sqrt(n)+1);
    char prime[NUM_MAX+1]; // flag
    memset(prime, 0x01, max);
    int ans[31]; // 2**31 = 2-billion > N
    int pidx = 0;
    int i, j;
    // get primes in advance @ Sieve of Eratosthenes
    for(i = 2; i <= max; i++) {
        if(!prime[i]) continue;
        // ensure that i is prime.
        int ulimit = max/i;
        // sieve
        for(j = 2; j <= ulimit; j++) {
            prime[i*j] = 0;
        }
    }
#ifdef DEBUG
    for(i = 2; i <= 10; i++) { printf("%d:%d ", i, prime[i]); }
    putchar('\n');
#endif
    int prev = 2;
    while(n>1) {
        int flag = 0;
        for(i = prev; i <= max; i++) {
            if(!prime[i]) continue;
            if(n%i) continue;
            // ensure that i is prime number
            ans[pidx++] = i;
            n = n / i;
            prev = i;
            flag = 1;
            break;
        }
        // num is unique prime number
        if(!flag) {
            ans[pidx++] = n; break;
        }
    }
    printf("%d: ", n1);
    for(i = 0; i < pidx; i++) {
        printf("%d%c", ans[i], (i==pidx-1) ? '\n' : ' ');
    }
    return 0;
}
