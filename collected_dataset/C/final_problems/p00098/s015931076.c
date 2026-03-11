#include <stdio.h>

int main(void) {
	int n;
	int a[101][101]={0};
	int i,j,k,l;
	int max,now;
	scanf("%d",&n);
	for(i=1;i<=n;i++) {
		for(j=1;j<=n;j++)scanf("%d",&a[i][j]);
	}
	for(i=1;i<=n;i++) {
		for(j=1;j<=n;j++)a[i][j]+=a[i][j-1];
	}
	for(j=1;j<=n;j++) {
		for(i=1;i<=n;i++)a[i][j]+=a[i-1][j];
	}

	max=-100000000;
	for(i=0;i<n;i++) {
		for(j=0;j<n;j++) {
			for(k=i+1;k<=n;k++) {
				for(l=j+1;l<=n;l++) {
					now=a[k][l]-a[k][j]-a[i][l]+a[i][j];
					if(now>max)max=now;
				}
			}
		}
	}
	printf("%d\n",max);
	return 0;
}