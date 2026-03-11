#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h> // uint64_t

#define BUF_SIZE 30

// size: specify sizeof(str)
int get_str(char *str, int size) {
    if(!fgets(str, size, stdin)) return -1;
    return 0;
}

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

int get_int2(int *a1, int *a2) {
#ifdef BUF_SIZE
  char line[BUF_SIZE];
  if(!fgets(line, BUF_SIZE, stdin)) return -1;
  sscanf(line, "%d %d", a1, a2);
#else
#error
#endif
  return 0;
}

#define NUM_MAX 200000
#define DIVISOR 1000000007

struct pair {
    int x;
    int y;
};

static uint64_t factorial[NUM_MAX*2+1];
static uint64_t inv_factorial[NUM_MAX*2+1];

struct pair gcdext(int a, int b) {
    if(a == 0) { return (struct pair){0, 1}; }
    struct pair p = gcdext(b%a, a);
    return (struct pair){p.y - (b/a)*p.x, p.x};
}

void prep_factorial(int n) {
    factorial[0] = factorial[1] = 1;
    inv_factorial[0] = inv_factorial[1] = 1;
    int i;
    for(i = 2; i <= n; i++) {
        factorial[i] = (factorial[i-1] * i)%DIVISOR;
        struct pair p = gcdext(i, DIVISOR);
        int inv = (p.x + DIVISOR)%DIVISOR;
        inv_factorial[i] = (inv_factorial[i-1] * inv)%DIVISOR;
    }
    return;
}

uint64_t get_combi(int n, int k) {
    uint64_t res = factorial[n];
    res = (res * inv_factorial[k])%DIVISOR;
    res = (res * inv_factorial[n-k])%DIVISOR;
    return res;
}

uint64_t get_homog(int n, int k) {
    return get_combi(n+k-1, k);
}

int main(void) {
    int n, k;
    get_int2(&n, &k);
    if(k>=n-1) k = n-1;
    prep_factorial(n*2);
    int i;
    uint64_t ans = 0;
    for(i = 0; i <= k; i++) {
        int rem = n - i;
        // rem: splits
        // n-rem: balls(0-indexed)
        uint64_t res = get_homog(rem, n-rem);
        res = (res * get_combi(n, i))%DIVISOR;
#ifdef DEBUG
        printf("%d: %llu\n", i, res);
#endif
        ans = (ans + res)%DIVISOR;
    }
    printf("%d\n", (int)ans);
}