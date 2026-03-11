#include <stdio.h>
#include <math.h>
#include <limits.h>
#include <stdlib.h>
long long N,W;
static long long  h[100001][3];
static long long  dp[101][100001];
long long Wg[1000],Val[1000];
long long  INF = INT_MAX;

int 
chmin(long long  *a, long long b) 
{
	 if (*a > b){
		 *a = b; //	printf("%ld\n",*a);
		 return 1; 
	 }
	 return 0;
}

int 
chmax(long long *a, long long b) 
{
	 if (*a < b){
		 *a = b; //	printf("%ld\n",*a);
		 return 1; 
	 }
	 return 0;
}

int 
main(){
	long long X,K;
	static char s[100010][20];
	static long long  key[100010];//[256];
	static int  check_same[100010];
	scanf("%lld",&N);
	long long i,j;
	for(i=0;i<N;i++){
		scanf("%s",s[i]);
		int j;
		for(j=0;j<10;j++){
			key[i]+=s[i][j]-'a';         //++;
			//printf("(%c,%d),",s[i][j],key[i][s[i][j]]);
		}
	}
	
	long long d;
	long long cnt=0;
	long long sum=0;
	for(i=0;i<N-1;i++){
		if(check_same[i]==0){
			for(j=i+1;j<N;j++){
				if(check_same[j]==0){
					//printf("(%d,%lld %lld)\n",i,key[i],key[j]);
					if(key[i]==key[j]){
						cnt++;
						check_same[j] =1;
					}
				}

			}
			if(cnt){
				//printf("cnt%lf,%lld\n",cnt,sum);
				sum += cnt*(cnt+1)/2;
			}
			cnt=0;
		}
	}
	
	printf("%lld\n",sum);
 return 0;
}