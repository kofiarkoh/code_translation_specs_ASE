#include <stdio.h>

int x,y,z;
int v[4],n,e[51]={0},a[51]={0};
double dp[51][5101];

int min(int in1,int in2) {
	return in1>in2?in2:in1;
}
int max(int in1,int in2) {
	return in1<in2?in2:in1;
}

double dfs(int current,int money){
	double sum=0.0;
	int i,j,h;
	while(current<y){
		if(e[current]==1) current+=a[current];
		else break;
	}
	if(current==0) for(i=0;i<51;i++) for(j=0;j<5101;j++) dp[i][j]=-1;
	if(current==y) return (double)money;
	if(e[current]==2) money+=a[current];
	if(e[current]==3) money-=a[current];
	if(money<0) money=0;
	if(dp[current][money]!=-1.0) return dp[current][money];
	for(i=0;i<x;i++){
		sum+=dfs(min(current+v[i],y),money);
	}
	return dp[current][money]=sum/(double)x;
}

int main(){
	int i;

	while(1){
		scanf("%d %d %d",&x,&y,&z);
		if(x==0 && y==0 && z==0) break;
		for(i=0;i<x;i++) scanf("%d",&v[i]);
		for(i=0;i<z;i++) {
			scanf("%d ",&n);
			scanf("%d %d",e + n,a + n);
		}
		printf("%d\n",(int)dfs(0,0));
	}
	return 0;
}