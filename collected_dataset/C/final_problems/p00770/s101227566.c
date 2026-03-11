#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define repl(i,a,b) for(i=(int)(a);i<(int)(b);i++)
#define rep(i,n) repl(i,0,n)

struct P{
	int fi,se;
};

#define bool int
#define false 0
#define true 1
#define P struct P

#define INF INT_MAX/3

#define MAX_N 1000

int n,m;
int board[1111][1111];
bool is_prime[1111111];

int dx[]={1,0,-1,0};
int dy[]={0,-1,0,1};

P mem[1111][1111];
P none;

P max(P a,P b){
	if(a.fi==b.fi){
		if(a.se>b.se)return a;
		else return b;
	}else if(a.fi>b.fi)return a;
	else return b;
}

P dfs(int x,int y){
	if(mem[x][y].fi!=-1)return mem[x][y];
	P res;
	res.fi=0; res.se=0;
	int xx;
	repl(xx,x-1,x+2){
		if(xx>=0&&xx<1111&&y+1>=0&&y+1<1111&&board[xx][y+1]!=0&&board[xx][y+1]<=m)res=max(res,dfs(xx,y+1));
	}
	if(is_prime[board[x][y]]){
		res.fi++;
		if(res.se==0)res.se=board[x][y];
	}
	return mem[x][y]=res;
}

int main(){
	//cin.sync_with_stdio(false);
	memset(is_prime,1,sizeof(is_prime));
	is_prime[0]=is_prime[1]=false;
	int i,j;
	rep(i,1111111){
		if(is_prime[i]){
			for(int j=i*2;j<1111111;j+=i)is_prime[j]=false;
		}
	}
	int num=1;
	int x=1111/2,y=1111/2;
	int len=1,state=0;
	none.fi=-1; none.se=-1;
	while(1){
		rep(i,2){
			rep(j,len){
				board[x][y]=num++;
				x+=dx[state];y+=dy[state];
			}
			state=(state+1)%4;
		}
		if(num>1000000)break;
		len++;
	}
	while(1){
		scanf("%d %d",&m,&n);
		if(n==0)break;
		int sx,sy;
		rep(i,1111)rep(j,1111){
			if(board[i][j]==n){
				sx=i;sy=j;
			}
		}
		rep(i,1111)rep(j,1111)mem[i][j]=none;
		P res=dfs(sx,sy);
		printf("%d %d\n", res.fi,res.se);
	}
	return 0;
}