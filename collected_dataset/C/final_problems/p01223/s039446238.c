#include<stdio.h>
int main(void)
{
	int i,j,n,m,up,down,mostup,mostdown,x[100+1];
	scanf("%d",&n);
	for(i=1;i<=n;i++) {
	mostup=0;
	mostdown=0;
		scanf("%d",&m);
		for(j=1;j<=m;j++) {
			scanf("%d",&x[j]);
		}
		for(j=1;j<=m-1;j++) {
			if(x[j]<=x[j+1]) {
				up=x[j+1]-x[j];
				if(mostup<=up) {
					mostup=up;
				}
			}
			else {
			down=x[j]-x[j+1];
				if(mostdown<=down) {
					mostdown=down;
				}

			}
		}
		printf("%d %d\n",mostup,mostdown);
	}
	return 0;
}