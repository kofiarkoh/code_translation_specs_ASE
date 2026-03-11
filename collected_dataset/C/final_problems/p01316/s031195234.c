#include<stdio.h>

#define INF 1000000000

typedef struct{
	int val;
	int sum;
} data;

data dp[16][20000];

int main(void){
	int n,m,i,j,k,c[16],x[20000],min;
	data tmp;
	while(scanf("%d %d",&n,&m)!=0 && n!=0 && m!=0){
		for(i=0;i<m;i++){
			scanf("%d",&c[i]);
		}
		for(i=0;i<n;i++){
			scanf("%d",&x[i]);
		}
		for(i=0;i<n;i++){
			for(j=0;j<m;j++){
				dp[i][j].sum=INF;
			}
		}
		for(i=0;i<m;i++){
			dp[0][i].val=128+c[i];
			if(dp[0][i].val<0){
				dp[0][i].val=0;
			}
			if(dp[0][i].val>255){
				dp[0][i].val=255;
			}
			dp[0][i].sum=(dp[0][i].val-x[0])*(dp[0][i].val-x[0]);
		}
		for(i=1;i<n;i++){
			for(j=0;j<m;j++){
				for(k=0;k<m;k++){	
					tmp.val=dp[i-1][j].val+c[k];
					if(tmp.val<0){
						tmp.val=0;
					}
					if(tmp.val>255){
						tmp.val=255;
					}
					tmp.sum=dp[i-1][j].sum+((tmp.val-x[i])*(tmp.val-x[i]));
					if(tmp.sum<dp[i][j].sum){
						dp[i][j].val=tmp.val;
						dp[i][j].sum=tmp.sum;
					}
				}
			}
		}
		min=INF;
		if(n==0){
			min=0;
		}
		for(i=0;i<m && n!=0;i++){
			if(dp[n-1][i].sum<min){
				min=dp[n-1][i].sum;
			}
		}
		printf("%d\n",min);
	}
	return 0;
}