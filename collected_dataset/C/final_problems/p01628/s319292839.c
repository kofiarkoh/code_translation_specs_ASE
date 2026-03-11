#include <stdio.h>

int max(int arg1, int arg2) {
	if (arg1 < arg2) {
		return arg2;
	}
	return arg1;
}

int main(){
	int N, M;
	int i;
	int res = 0;
	int info[8] = {0};

	scanf("%d %d", &N, &M);
	for (i = 0; i < M; i++) {
		int tmp;
		int a;

		scanf("%d", &a);
		tmp = max(info[a], info[a-1])+1;
		info[a] = tmp;
		info[a-1] = tmp;
	}

	for (i = 0; i < N; i++) {
		if (res < info[i]) {
			res = info[i];
		}
	}

	printf("%d\n", res);

	return 0;
}