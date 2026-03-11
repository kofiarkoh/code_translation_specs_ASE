#include<stdio.h>
int main() {
	int H, N, x;
	scanf("%d%d", &H, &N);
	int A[N], i=0, sum=0, t=1;
	while(scanf("%d", &x) == t) {
		A[i++] = x;
		sum += x;
		if(i == N) break;
	}
	if(sum < H) printf("No\n");
	else printf("Yes\n");
	return 0;
}