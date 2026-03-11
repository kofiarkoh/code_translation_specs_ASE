#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include<math.h>
#define swap(type,a,b) do{type t=a;a=b;b=t;}while(0);
#define MAX(a,b) (((a)>(b))?(a):(b))
#define MIN(a,b) (((a)<(b))?(a):(b))
#define ll long long
#define INF 100000000
#define FOR(i,a,n) for(i=a;i<n;i++)
int comp(const void* a,const void* b){
	return *(int*)a-*(int*)b;
}
void fill(int a[],int b,int c){
	int i;
	FOR(i,0,b) a[i]=c;
	return;
}
int dp[10001];
int main(void)
{
    int n,s,i,j;
    scanf("%d",&n);
    dp[0]=1;
    FOR(i,0,n){
        scanf("%d",&s);
        for(j=10000;j>=0;j--){
            if(dp[j]) dp[j+s]=1; 
        }
    }
    for(i=10000;i>=0;i--){
        if(i%10!=0&&dp[i]){
            printf("%d\n",i);
            return 0;
        }
    }
    puts("0");
	return 0;
}
