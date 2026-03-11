#include <stdio.h>
#define Rep(i,a,n)for(int i=(int)(a);i<(int)(n);i++)
#define rep(i,n)Rep(i,0,n)

int main(){
	int d,c[27],s[366][27]={0},t[366],last[27]={0},sat=0;
	scanf("%d",&d);
	rep(i,26)scanf("%d",c+i+1);
	rep(i,d)rep(j,26)scanf("%d",&s[i+1][j+1]);
	rep(i,d)scanf("%d",t+i+1);
	rep(i,d){
		sat+=s[i+1][t[i+1]];
		last[t[i+1]]=i+1;
		rep(j,26)sat-=c[j+1]*(i+1-last[j+1]);
		printf("%d\n",sat);
	}
	return 0;
}