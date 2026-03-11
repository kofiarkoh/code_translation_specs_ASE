#include <stdio.h>
#include <stdlib.h>
#define int long long

signed main(){
	int N, i, num[5] = {0, 0, 0, 0, 0};
	scanf("%lld", &N);
	int *a = (int *)malloc(sizeof(int) * N);
	for(i = 0; i < N; i++){
		scanf("%lld", &a[i]);
		num[a[i] + 2]++;
	}
	num[2] = 0;
	num[3] = 0;
	if(num[0] % 2 == 0){
		num[1] = 0;
	}
	else{
		if(num[1] > 0){
			num[1] = 1;
		}
		else{
			num[0] = 2 * (num[0] / 2);
		}
	}
	printf("%d\n", num[0] + num[1] + num[2] + num[3] + num[4]);
	for(i = 0; i < N; i++){
		if(num[a[i] + 2] > 0){
			printf("%d\n", i + 1);
			num[a[i] + 2]--;
		}
	}
	return 0;
}
