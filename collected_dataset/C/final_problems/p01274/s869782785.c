#include<stdio.h>
long long MIN(long long a,long long b){return a<b?a:b;}
long long MAX(long long a,long long b){return a<b?b:a;}
long long n,m,i,j,k,a[100010],b[100010],c[100010],min,d[2][100010];
int main(){
	char s[100010][10];
	while(scanf("%d",&n),n){
		for(i=0;i<n;i++)scanf("%lld",&a[i]);
		scanf("%d",&m);
		for(i=0;i<m;i++)scanf("%*s %lld %s %lld",&b[i],&s[i],&c[i]);
		for(i=0;i<100010;i++)d[0][i]=d[1][i]=1e9+10;
		for(i=d[0][0]=d[1][0]=0;i<100000;i++){
			for(j=0;j<m;j++){
				k=(s[j][0]=='S'?0:1);
				d[k][MIN(100000,i+c[j])]=MIN(d[k][MIN(100000,i+c[j])],d[k][i]+b[j]);
			}
		}//return 0;
		for(i=0;i<2;i++){
			for(j=100000;j;j--)d[i][j-1]=MIN(d[i][j-1],d[i][j]);
		}
		min=1e9+10;
		for(i=0;i<100001;i++){
			k=d[1][i];
			for(j=0;j<n;j++)k+=d[0][MAX(0,a[j]-i)];
			min=MIN(min,k);
		}
		printf("%lld\n",min);
	}
	return 0;
}
