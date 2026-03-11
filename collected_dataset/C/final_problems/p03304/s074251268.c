#include <stdio.h>

int main(){
	int n,m,d;
	scanf("%d%d%d",&n,&m,&d);
	printf("%.10f",1.0*(d==0?1:2)*(n-d)/n/n*(m-1));
}