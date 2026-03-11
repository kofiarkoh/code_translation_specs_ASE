#include <stdio.h>
#include <math.h>
#include <stdlib.h>

#define REP(i,n,m) for(i=n;i<m;i++)
#define rep(i,n) REP(i,0,n)

int an,bn,r;
char p[10000][10000];
double dist[42][42];

int main(void){
	char set;
	int i,j;
	int x,y,fx,fy,tx,ty,ans;

	rep(i,42) rep(j,42) dist[i][j] = sqrt(i*i+j*j);

	for(set=1;scanf("%d%d%d",&an,&bn,&r), an;set++){
		while(an--){
			scanf("%d%d",&x,&y);
			p[y][x] = set;
		}

		ans = 0;
		while(bn--){
			scanf("%d%d",&x,&y);
			fx = x-4*r<0 ? 0 : x-4*r;
			fy = y-4*r<0 ? 0 : y-4*r;
			tx = x+4*r+1>10000 ? 10000 : x+4*r+1;
			ty = y+4*r+1>10000 ? 10000 : y+4*r+1;
			REP(i,fy,ty) REP(j,fx,tx){
				if(p[i][j]==set && (i-y)*(i-y)+(j-x)*(j-x)<=16*r*r){
					ans++;
				}
			}
		}

		printf("%d\n",ans);
	}

	return 0;
}