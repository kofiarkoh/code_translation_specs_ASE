#include <stdio.h>

int main()
{
	int i, k, flag = 0;
	int N, A[20000];
	scanf("%d",&N);
	for(i = 0; i < N; i++){
		scanf("%d", &A[i]);
	}
	for(i = 0;i < N; i++){
		k = i + 1;
		while(k < N){
			if(A[i] == A[k]){
				flag = 1;
			}
			k++;
			if(flag==1) break;
		}
		if(flag==1) break;
	}
	if(flag == 1) printf("NO");
	else printf("YES");
    return 0;
}