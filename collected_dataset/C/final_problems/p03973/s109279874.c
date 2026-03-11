#include<stdio.h>
int main(){
	long ans = 0;
	int N;
	scanf("%d",&N);
	long A[N];
	int i = 0;
	for(i=0 ; i<N ; i++){
		scanf("%ld",&A[i]);
	}
	ans += A[0]-1 ;
	long r = 2;
	for(i = 1; i<N ; i++)
	{
		if(A[i] != r){
			ans += (A[i]-1)/r ;
		}
		else
			r++;
	}
	printf("%ld\n",ans);
	return 0;
}