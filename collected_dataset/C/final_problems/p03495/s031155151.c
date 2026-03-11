#include <stdio.h>
int main() {
    int n, k;
    scanf("%d %d", &n, &k);
    int cnt = 0;
    int a;
    for (int i = 0; i < n; i++) {
        scanf("%d", &a);
        if (a > k)
            cnt++;
    }
    printf("%d\n", cnt);
  	return 0;
}