#include <stdio.h>

int N, W, v[101], w[101], max;

void a(int i, int aw, int val) {
	if (i == N) {
		if (max < val) max = val;
	} else if (aw+w[i] > W) {
		a(i+1, aw, val);
	} else {
		a(i+1, aw+w[i], val+v[i]);
		a(i+1, aw, val);
	}
}

int main() {
	int i;
	max = 0;
	scanf("%d%d", &N, &W);
	for (i=0; i < N ;i++) scanf("%d%d",&v[i],&w[i]);
	a(0, 0, 0);
	printf("%d\n", max);
	return 0;
}