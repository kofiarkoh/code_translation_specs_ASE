#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h>

#define BUF_SIZE 200

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

// <arr[0]> <arr[1]> .. <arr[size-1]>
int fget_array(int *arr, int size) {
#ifdef BUF_SIZE
    char line[BUF_SIZE];
    char *tmpbuf = line;
    int i;
    if(!fgets(line, BUF_SIZE, stdin)) return -1;
    for(i = 0; i < size; i++) {
        char *tmp = strtok(tmpbuf, " ");
        arr[i] = strtol(tmp, NULL, 10);
        tmpbuf = NULL;
    }
#else
#error
#endif
    return 0;
}

#define NUM_MAX 15
#define COLOR_MAX NUM_MAX
#include <limits.h>

#define max(a,b) ((a) > (b) ? (a) : (b))
#define min(a, b) ((a) > (b) ? (b) : (a))

struct pair {
    int height;
    uint64_t sum;
};

int main(void) {
    int num, colors;
    get_int2(&num, &colors);
    
    int buildings[NUM_MAX];
    fget_array(buildings, num);

    static struct pair dp[NUM_MAX+1][COLOR_MAX+1];
    struct pair ini = {0, (uint64_t)UINT64_MAX/10};
    int i, j, color;
    for(i = 0; i <= num; i++) {
        for(j = 0; j <= colors; j++) {
            dp[i][j] = ini;
        }
    }
    dp[0][0] = (struct pair){0, 0};
    dp[1][1] = (struct pair){buildings[0], 0};
    for(i = 2; i <= num; i++) {
        int b = buildings[i-1];
        int m = min(i, colors);
        for(color = 1; color <= m; color++) {
            dp[i][color] = dp[i-1][color];
            int prev = dp[i-1][color-1].height;
            if(!prev) continue;
            int nb = b > prev ? b : prev+1;
            struct pair np = {
                nb,
                dp[i-1][color-1].sum + (uint64_t)(nb-b)
            };
            if(dp[i][color].sum > np.sum) {
                dp[i][color] = np;
            }
        }
#ifdef DEBUG
        printf("i : %d\n", i);
        for(j = 1; j <= m; j++) {
            printf("#color%d=> %d:%llu ",j, dp[i][j].height, dp[i][j].sum);
        }
        putchar('\n');
#endif
    }
    printf("%llu\n", dp[num][colors].sum);

    return 0;
}