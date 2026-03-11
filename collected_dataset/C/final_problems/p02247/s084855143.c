#include<stdio.h>
#include<string.h>
#define N 1000

int main() {
	int i, j, len, flag;
	char T[N], P[N];

	scanf("%s", T);
	scanf("%s", P);

	for (i = 0; i < strlen(T); i++) {
		flag = 0;
		if (T[i] == P[0]) {
			flag = 1;
			for (j = 0; j < strlen(P); j++) {
				if (T[i+j] != P[j]) {
					flag = 0;
					break;
				}
			}
		}
		if (flag == 1) printf("%d\n", i);
	}

	return 0;
}