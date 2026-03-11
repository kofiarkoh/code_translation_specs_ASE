#include <stdio.h>
#include <stdlib.h>
#define Rep(i,a,n)for(int i=(int)(a);i<(int)(n);i++)
#define rep(i,n)Rep(i,0,n)
int dn(const void*a,const void*b){return*(int*)b-*(int*)a;}
int main(){
	int n,k,a,b[200009]={0},c=0;
	scanf("%d%d",&n,&k);
	rep(i,n){scanf("%d",&a);b[a]++;if(b[a]==1)c++;}
	qsort(b,200008,4,dn);
	a=0;
	Rep(i,k,c+1)a+=b[i];
	printf("%d",a);
	return 0;
}