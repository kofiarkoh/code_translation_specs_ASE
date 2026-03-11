#include<stdio.h>
#include<math.h>

int main(void){
	int i,N,t;
	int sum = 1;
	int s[100000];
	scanf("%d", &N);
	for (i = 0; i < N; i++) {
		s[i] = 0;
	}
	for (i = 1; i <= N; i++) {
		scanf("%d",&t);
		s[t]++;
	}
	for (i = 0; i < N-1; i++) {
		sum*=pow(s[i], s[i + 1]);
	}
	printf("%d\n", sum);

	return 0;
}