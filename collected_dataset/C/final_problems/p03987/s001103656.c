/* AtCoder My Practice
 * author: Leonardone @ NEETSDKASU
 */
#include <stdio.h>
#include <stdlib.h>

#define N_MAX (200001)

typedef struct _range {
    int upper, lower;
    struct _range *ubound, *lbound;
} Range;

Range *newRange(int lower, int upper) {
    Range *range = (Range*)malloc(sizeof(Range));
    range->lower = lower;
    range->upper = upper;
    return range;
}

int idx[N_MAX];
long long sum;
Range range;

void calc(int value, int index) {
    Range *tmp = &range;
    long long len;
    for (;;) {
        if (tmp->lower <= index && index <= tmp->upper) {
            len = (long long)(tmp->upper - tmp->lower + 1);
            sum += (len * (len + 1LL) / 2LL) * (long long)value;
            len = (long long)(tmp->upper - index);
            sum -= (len * (len + 1LL) / 2LL) * (long long)value;
            len = (long long)(index - tmp->lower);
            sum -= (len * (len + 1LL) / 2LL) * (long long)value;
            if (tmp->lower == index) {
                tmp->lower++;
            } else if (tmp->upper == index) {
                tmp->upper--;
            } else {
                tmp->lbound = newRange(tmp->lower, index - 1);
                tmp->ubound = newRange(index + 1, tmp->upper);
                tmp->lower = tmp->upper = index;
            }
            break;
        } else if (index < tmp->lower) {
            tmp = tmp->lbound;
        } else {
            tmp = tmp->ubound;
        }
    }
}


int main(void){
    int i, n;
    
    scanf("%d", &n);

    for (i = 0; i < n; i++) {
        int a;
        scanf("%d", &a);
        idx[a] = i;
    }
    
    range.upper = n - 1;
    
    for (i = 1; i <= n; i++) {
        calc(i, idx[i]);
    }
    
    printf("%lld\n", sum);

    return 0;
}
