#include <stdio.h>
#include <stdlib.h>

int main(){
	int N, i, j, ans;
	scanf("%d", &N);
	int *p = (int *)malloc(sizeof(int) * (N + 1));
	int *have_pair = (int *)malloc(sizeof(int) * (N + 1));
	p[0] = -1;
	for(i = 1; i <= N; i++){
		scanf("%d", &p[i]);
	}
	for(i = 1; i <= N; i++){
		for(j = 0; j <= N; j++){
			have_pair[j] = 0;
		}
		ans = 0;
		for(j = i; j > 0; j--){
			if(have_pair[j] == 0 && have_pair[p[j]] == 0){
				ans++;
				have_pair[p[j]] = 1;
			}
		}
/*		printf("i = %d\n", i);
		for(j = 0; j <= N; j++){
			printf("%d  ", have_pair[j]);
		}
		printf("\n");
*/		printf("%d\n", ans);
	}
	return 0;
}