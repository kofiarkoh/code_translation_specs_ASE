#include <stdio.h>
#include <string.h>
#include <stdlib.h>

// 5*300
#define BUF_SIZE 1600

int get_int(void) {
  int num;
#ifdef BUF_SIZE
  static char line[BUF_SIZE];
  if(!fgets(line, BUF_SIZE, stdin)) return 0;
  sscanf(line, "%d", &num);
#else
#error
#endif
  return num;
}

// <arr[0]> <arr[1]> .. <arr[size-1]>
int fget_array(int *arr, int size) {
#ifdef BUF_SIZE
    static char line[BUF_SIZE];
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
#define NUM_MAX 300
#define WEIGHT_MAX 1000
#define SET_MAX 50


#define STACK_MAX 20000
#define max(a,b) ((a) > (b) ? (a) : (b))
#define DP_SIZE_MAX (NUM_MAX+1)*(NUM_MAX+1)

int drop_point(int v1, int v2) {
    return (abs(v1-v2)<=1)*2;
}

int main(void) {
    int ans[SET_MAX];
    int idx = 0;
    int i;
    // dp[size][start];
    while(1) {
        int dp[NUM_MAX+1][NUM_MAX+1] = {0};
        int num = get_int();
        if(!num) break;
        int ws[NUM_MAX+1];
        fget_array(ws, num);
        int size, start, end;
        // for size=2
        for(i = 0; i < num-1; i++) {
            dp[2][i] = drop_point(ws[i], ws[i+1]);
        }
        for(size = 3; size <= num; size++) {
            for(start = 0; start < num-size+1; start++) {
                end = start + size - 1;
                int res = dp[size-1][start]; // When topmost daruma left
                int pivot;
                // consider that the situation where pivot and end are dropped
                for(pivot = start; pivot < end; pivot++) {
                    int mid_size = end-pivot-1;
                    int mid = dp[mid_size][pivot+1];
                    res = max(
                        dp[pivot-start][start] + 
                          mid + 
                          (mid==mid_size)*drop_point(ws[pivot], ws[end]),
                        res
                    );
#ifdef DEBUG
                    printf("(%d, %d) -> %d\n",start, end, res);
#endif
                }
                dp[size][start] = res;
            }
        }
        ans[idx++] = dp[num][0];
    }
    for(i = 0; i < idx; i++) {
        printf("%d\n", ans[i]);
    }
    return 0;
}
