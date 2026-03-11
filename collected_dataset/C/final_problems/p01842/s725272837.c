#include <stdio.h>
#define max(p,q)((p)>(q)?(p):(q))
#define min(p,q)((p)<(q)?(p):(q))

int dp[60][60][110][2][2];
int check[60][60][110][2][2];
//dp[n][m][x][c][p];
//デッキのn,m枚目を次に使い、スタックにx枚積んであり、
//手番がcで、直前がパス(p?である:でない)
int a[60],b[60];

int calc(int n,int m,int x,int c){
	int ttt=0;
	int b0=0,b1=0;
	for(int i=0;i<x;i++){
		if(i%2==c){
			if(a[--n]==-1)b1=1;
			else if(!b0)ttt+=a[n];
		}else{
			if(b[--m]==-1)b0=1;
			else if(!b1)ttt-=b[m];
		}
	}
//	printf("%d %d %d %d %d\n",n,m,x,c,ttt);
	return ttt;
}


int N,M;
int f(int n,int m,int x,int c,int p){
	if(check[n][m][x][c][p]++)return dp[n][m][x][c][p];
	int temp;
	if(c==0){
		temp=-1e9;
		//テイク
		if(n<N)temp=max(temp,f(n+1,m,x+1,c^1,0));
		//パス
		//スタックの解消
		int ttt=calc(n,m,x,1);
		if(p)temp=max(temp,ttt);
		else temp=max(temp,ttt+f(n,m,0,c^1,!x));
	}else{
		temp=1e9;
		//テイク
		if(m<M)temp=min(temp,f(n,m+1,x+1,c^1,0));
		//パス
		//スタックの解消
		int ttt=calc(n,m,x,0);
		if(p)temp=min(temp,ttt);
		else temp=min(temp,ttt+f(n,m,0,c^1,!x));
	}
	return dp[n][m][x][c][p]=temp;
}

int main(){
	scanf("%d%d",&N,&M);
	for(int i=0;i<N;i++)scanf("%d",a+i);
	for(int i=0;i<M;i++)scanf("%d",b+i);	
	printf("%d\n",f(0,0,0,0,0));
}
