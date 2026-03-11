#include <stdio.h>

int main(void)
{
	int n,a,b,cost=0;
	scanf("%d%d%d",&n,&a,&b);
	int x[n],i;

	for(i=0;i<n;i++){
		scanf("%d",&x[i]);
		if(i>=1)
			if((x[i]-x[i-1])*a<b){
				cost+=a*(x[i]-x[i-1]);
			}else{
				cost+=b;
			}
	}

	printf("%d\n",cost);
	return 0;
}