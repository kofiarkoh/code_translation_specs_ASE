#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h> // int64_t
#include <math.h>

#define max(a,b) ((a) > (b) ? (a) : (b))
#define min(a,b) ((a) > (b) ? (b) : (a))

int get_int(void) {
  int num;
  scanf("%d", &num);
  return num;
}

int get_int2(int *a1, int *a2) {
  scanf("%d %d", a1, a2);
  return 0;
}

void swap(int *a1, int *a2) {
    int tmp = *a1;
    *a1 = *a2;
    *a2 = tmp;
}

#define QUERY_MAX 100

int main(void) {
    int queries = get_int();

    int i;
    int64_t ans[QUERY_MAX];
    int aidx = 0;
    for(i = 0; i < queries; i++) {
        int a1, a2;
        int64_t res;
        get_int2(&a1, &a2);
        if(a1 > a2) swap(&a1, &a2);
        if(a1 == a2) {
            res = (a1-1)*2; // exclude a1-a2
        } else if (a1+1 == a2) {
            res = (a1-1)*2;
        } else {
            int64_t sum = (int64_t)a1*a2;
            int64_t power_2 = sqrt(-0.5 + sum);
            if(power_2*(power_2+1) >= sum) {
                res = (power_2-1)*2;
            } else {
                res = (power_2)*2-1;
            }
            if(sum <= 3) {
                res = 1;
            }
        }
        ans[aidx++] = res;
    }

    for(i = 0; i < aidx; i++) {
        printf("%lld\n", ans[i]);
    }
    return 0;
}