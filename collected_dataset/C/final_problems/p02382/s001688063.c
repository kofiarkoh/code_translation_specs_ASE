#include <stdio.h>
#include <math.h>
#include <stdlib.h>
int main(){
	double a[1010];
	double b[1010];
	int n;
	double m=0;
	double ans=0;
	int i,j;
	scanf("%d",&n);
	for(i=0;i<n;i++){
		scanf("%lf",&a[i]);
	}
	for(i=0;i<n;i++){
		scanf("%lf",&b[i]);
	}
	for(i=1;i<4;i++){
		m=0;
		for(j=0;j<n;j++){
			m+=pow(abs(a[j]-b[j]),i);
		}
		printf("%lf\n",pow(m,1.0/(double)i));
	}
	m=0;
	for(i=0;i<n;i++){
		m=fmax(m,abs(a[i]-b[i]));
	}
	printf("%lf\n",m);
	return 0;
}
