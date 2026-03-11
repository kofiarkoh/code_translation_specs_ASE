#include<stdio.h>
#define mod 1000000007
#define are(h,i,j) (dp[(h)-1][i][j]+a[i][h-(i)]+a[j][h-(j)])
int max(int a,int b){
	if(a>b){ return a; } else{return b;}
}
int main(void){
	//定数倍ひどいことになってるなこれ
	int H,W,h,i,j,b;
	static int a[401][401]={0};
	static int dp[401][401][401]={0};
	scanf("%d %d",&H,&W);
	for(i=0;i<H;i++){
		for(j=0;j<W;j++){
			scanf("%d",&b);
			a[i][j]=b;
		}
	}
	dp[0][1][1]=a[0][0];
	for(h=1;h<=H+W;h++){
		for(i=1;i<=h;i++){
			for(j=i+1;j<=h+1;j++){//i<j
				dp[h][i][j]=max(max(dp[h-1][i][j],dp[h-1][i-1][j]),max(dp[h-1][i][j-1],dp[h-1][i-1][j-1]));
				dp[h][i][j]+=a[i-1][h-(i-1)]+a[j-1][h-(j-1)];
			}
		}
	}
	printf("%d\n",dp[H+W][H][H+1]);
	return 0;
}