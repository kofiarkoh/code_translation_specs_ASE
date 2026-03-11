#include <stdio.h>
#include <stdlib.h>

int main (void){
	int N,i;
	int X=0;
	int ans=0;
	scanf("%d",&N);
	int A[N];
	for(i=0;i<N;i++){
		scanf("%d",&A[i]);
		A[i] -= (i+1);
		//printf("%d\n",A[i]);
		X += A[i];
	}
	if( X/N >=0){
		for(i=0;i<N;i++){
			ans += abs(A[i] - X/N);
		}
	}else{
		int ans1=0;
		for(i=0;i<N;i++){
			ans1 += abs(A[i] - (X/N -1));
			ans  += abs(A[i] - (X/N -1));
		}
		if(ans1 > ans){
			printf("%d",ans);
			return 0;
		}else{
			printf("%d",ans1);
			return 0;
		}
	}
	printf("%d",ans);
	return 0;
}