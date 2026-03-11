// MattG 1-21-2020 (2:47 AM)

#include <stdio.h>

#define MAX 200005

typedef unsigned long long ull;

ull max(ull a, ull b) {
    return a > b ? a : b;
}

ull min(ull a, ull b) {
    return a < b ? a : b;
}

int abs(int a) {
    return a >= 0 ? a : -a;
}

ull sqr(ull a) {
    return a * a;
}

typedef struct {
    int index;
    ull height;
} Entry;

ull get_overtake(int *h, ull *dp, Entry old, int x) {
    ull top = ( sqr(h[x]) - sqr(h[old.index]) + dp[x] - dp[old.index] );
    ull bottom = ( 2 * ( h[x] - h[old.index] ) );  
    return (top / bottom) + (top % bottom == 0 ? 0 : 1);
    // account for not divisible case
}

// return new min_end value
//    0 1 2 3 4 5 6 _   <- min_list
//      s         e     // check endpoint cases separately (or at least end)
// if we're on an index, we're considering whether to replace it or not

int binsert(int s, int e, int *h, ull *dp, Entry *min_list, int new_index) {

    // how many left? implement todo? maybe unnecessary
    
    int m = (s + e) / 2;
    ull ot = get_overtake(h, dp, min_list[m-1], new_index);
    if (ot > min_list[m].height) {   // go right
        return binsert(m+1, e, h, dp, min_list, new_index);
    }
    else if (ot < h[new_index]) {  // go left
        return binsert(s, m-1, h, dp, min_list, new_index);
    }
    else {
        min_list[m].index = new_index;
        min_list[m].height = ot;
        return m;
    }
}

int main() {
    int N;
    ull C;
    int h[MAX];
    ull dp[MAX];
    Entry min_list[MAX];
    int min_start = 0;
    int min_end = 0;
    int src_index, start_check, end_check;

    dp[0] = 0;
    min_list[0].index = 0;
    min_list[0].height = -1;

    scanf("%d %llu", &N, &C);
    scanf("%d", &h[0]);

    for (int i=1; i < N; i++) {
        scanf("%d", &h[i]);
        // update min_start index (here or end?) // might be wrong be careful
        while (min_start < min_end && h[i] >= min_list[min_start+1].height) {
            min_start++;
        }

        src_index = min_list[min_start].index;
        dp[i] = dp[src_index] + sqr(h[i] - h[src_index]) + C;

        start_check = get_overtake(h, dp, min_list[min_start], i);
        if (start_check < h[i]) {
            min_list[min_start].index = i;
            min_list[min_start].height = -1;
            min_end = min_start;
        }
        else {
            end_check = get_overtake(h, dp, min_list[min_end], i);
            if (end_check >= h[i]) {
                min_end++;
                min_list[min_end].index = i;
                min_list[min_end].height = end_check;
            }
            else {
                min_end = binsert(min_start+1, min_end, h, dp, min_list, i);
            }
        }
    }

    printf("%llu\n", dp[N-1]);
    return 0;
}
