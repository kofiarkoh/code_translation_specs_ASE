#include <stdio.h>
#include <math.h>

int main(){
	int N, M, ans[1023][1023] = {0};
	scanf("%d%d",&N,&M);
	int n = (int)pow(2, N) - 1;
	int m = (int)pow(2, M) - 1;
	ans[0][m - 1] = 1;
	ans[n - 1][m - 1] = 1;
	ans[n - 1][0] = 1;
	ans[0][0] = 1;
	ans[0][m / 2] = 1;
	ans[n - 1][m / 2] = 1;
	ans[n / 2][m / 2] = 1;
	ans[n / 2][m - 1] = 1;
	ans[n/2][0] = 1;
	for (int i = 0; i < n;i++) {
    	if(i != 0){
        	printf("\n");
    	}
		for (int j = 0; j < m;j++) {
			printf("%d",ans[i][j]);
		}
	}
}