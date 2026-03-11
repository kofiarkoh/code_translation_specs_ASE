#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define BUF_SIZE 30

// size: specify sizeof(str)
int get_str(char *str, int size) {
    if(!fgets(str, size, stdin)) return -1;
    return 0;
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

struct prod {
    int value;
    int weight;
};

#define NUM_MAX 100
#define WEIGHT_MAX 10000
#define max(a,b) ((a) > (b) ? (a) : (b))

int main(void) {
    int num, w_sum;
    get_int2(&num, &w_sum);
    struct prod ps[NUM_MAX];
    int i, j;
    for(i = 0; i < num; i++) {
        get_int2(&ps[i].value, &ps[i].weight);
    }

    static int vals[WEIGHT_MAX+1];

    for(i = 1; i <= num; i++) {
        int v = ps[i-1].value;
        int w = ps[i-1].weight;
        for(j = w; j <= w_sum; j++) {
            vals[j] = max(vals[j-w]+v, vals[j]);
        }
    }
    printf("%d\n", vals[w_sum]);
    return 0;
}

