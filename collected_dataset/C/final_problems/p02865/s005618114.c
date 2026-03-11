#include <stdio.h>
#define Fori(g) for(int i=0;i<g;i++)
#define	Forj(g) for(int j=0;j<g;j++)
#define min(aa,bb) return (aa<bb)?aa:bb
#define max(aa,bb) return (aa<bb)?bb:aa

int main(){
	int n;
	scanf("%d",&n);
	printf("%d\n",(n-1)/2);
	return 0;
}