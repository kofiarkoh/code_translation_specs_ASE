#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#define ll long long int
#define lim 100010
#define INF 1e9
#define mod 1000000007
#define MIN(x,y) ((x)<(y)?(x):(y))
#define MAX(x,y) ((x)<(y)?(y):(x))
#define ABS(x) ((x)>0?(x):-(x))

int main(void){
	int n;
	scanf("%d",&n);
	int w[101];
	int sum=0;
	for(int i=0;i<n;i++){
		scanf("%d",&w[i]);
		sum+=w[i];
	}
	int ans=sum;
	for(int i=0;i<n;i++){
		ans = MIN(ans,ABS(sum-2*w[i]));
		sum-=2*w[i];
	}
	printf("%d",ans);
	return 0;
}