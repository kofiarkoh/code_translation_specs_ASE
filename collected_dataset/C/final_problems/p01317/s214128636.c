#include <stdio.h>
#define rep(i,n) for(i=0;i<n;i++)
#define INF 100000000
#define min(a,b) (a<b?a:b)
int main(){
	int i,j,k,n,m,bit,next,cur,r,newCost,ret;
	int land[200][200] , ship[200][200] , R[1000] ,dp[2][200];
	while(scanf("%d%d",&n,&m) && n){
		rep(i,n)rep(j,n)land[i][j] = ship[i][j] = INF;
		rep(i,n)land[i][i] = ship[i][i] = 0;
		
		rep(i,m){
			char t;
			int a,b,c;
			scanf("%d %d %d %c\n",&a,&b,&c,&t);
			a--,b--;
			if(t=='L')land[a][b] = land[b][a] = min(c,land[a][b]);
			else  ship[a][b] = ship[b][a] = min(c,ship[a][b]);
		}
		rep(k,n)rep(i,n)rep(j,n)land[i][j] = min(land[i][j],land[i][k]+land[k][j]);
		rep(k,n)rep(i,n)rep(j,n)ship[i][j] = min(ship[i][j],ship[i][k]+ship[k][j]);
		
		scanf("%d",&r);	
		rep(i,r)scanf("%d",&R[i]) , R[i]--;
		
		rep(j,2)rep(k,n)dp[j][k] = INF;
		dp[0][R[0]] = 0;
		
		rep(bit,r-1){
			rep(cur,n){
				rep(next,n){
					newCost = dp[bit&1][cur] + land[R[bit]][cur] + ship[cur][next] + land[next][R[bit+1]];
					dp[!(bit&1)][next] = min( dp[!(bit&1)][next],newCost);
				}
				dp[!(bit&1)	][cur] = min(dp[!(bit&1)][cur],dp[bit&1][cur]+land[R[bit]][R[bit+1]]);
			}
			rep(j,n)dp[bit&1][j] = INF;
		}
		ret = INF;
		rep(i,n)ret = min(dp[(r-1)&1][i],ret);
		printf("%d\n",ret);
	}
	return 0;
}