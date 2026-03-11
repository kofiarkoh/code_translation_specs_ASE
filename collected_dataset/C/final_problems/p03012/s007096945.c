#include<stdio.h>
#include<stdlib.h>
#define min(a,b) ((a)<(b)?(a):(b))

int main(){
	int n,i,j;
	int w[100];
	scanf("%d",&n);
	for(i=0;i<n;i++)scanf("%d",&w[i]);
	int sum=0,ans=100000000,sums=0;
	for(i=0;i<n;i++)sum+=w[i];
	for(i=0;i<n;i++){
		sums+=w[i];
		ans=min(ans,abs(sum-sums-sums));
	}
	printf("%d\n",ans);
	return 0;
}