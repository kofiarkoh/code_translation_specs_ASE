#include <stdio.h>
#define MEM 100005

int main(void) {
    int N, Q;
    long long arr[MEM];
    
    scanf("%d %d", &N, &Q);
    for (int i = 0; i < N; i++) scanf("%lld", arr+i);
    for (int query = 0; query < Q; query++) {
        long long
            q,
            res = 0,
            sum = 0;
        int right = 0;
        scanf("%lld", &q);

        for (int left = 0; left < N; left++) {
            while (right < N && sum + arr[right] <= q) {
                sum += arr[right];
                right++;
            }
            res += right - left;
            if (right == left) {
                right++;
            } else {
                sum -= arr[left];
            }
        }

        printf("%lld\n", res);
    }

    return 0;
}
