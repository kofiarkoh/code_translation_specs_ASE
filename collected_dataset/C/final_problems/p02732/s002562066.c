#include<stdio.h>
int main(){
	int N;
	long long An=0;
	int A[200000];
	long long B[200000]={0};
	scanf("%d",&N);
	for(int i=0;i<N;i++){
		scanf("%d",&A[i]);
		A[i]-=1;
		B[A[i]]++;
	}
	for(int i=0;i<N;i++){
		An+=B[i]*(B[i]-1)/2;
	}
	long long Bn;
	for(int i=0;i<N;i++){
		Bn=An;
		Bn-=B[A[i]]*(B[A[i]]-1)/2;
		Bn+=(B[A[i]]-1)*(B[A[i]]-2)/2;
		printf("%lld\n",Bn);
	}
}