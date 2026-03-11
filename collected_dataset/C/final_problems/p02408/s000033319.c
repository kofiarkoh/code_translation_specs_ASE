#include<stdio.h>
int main(void)
{
	int i,j,n,x,s[53],h[53],c[53],d[53];
	char cc;
	scanf("%d",&n);
	for(j=1;j<=13;j++) {
		s[j]=0;
		h[j]=0;
		c[j]=0;
		d[j]=0;
	}
	for(i=1;i<=2*n;i++) {
		scanf("%c %d",&cc,&x);
		if(cc=='S') {
			for(j=1;j<=13;j++) {
				if(j==x) {
					s[j]++;
				}
			}
		}
		if(cc=='H') {
			for(j=1;j<=13;j++) {
				if(j==x) {
					h[j]++;
				}
			}
		}
		if(cc=='C') {
			for(j=1;j<=13;j++) {
				if(j==x) {
					c[j]++;
				}
			}
		}
		if(cc=='D') {
			for(j=1;j<=13;j++) {
				if(j==x) {
					d[j]++;
				}
			}
		}
	}
	for(j=1;j<=13;j++) {
		if(s[j]==0) {
			printf("S %d\n",j);
		}
	}
	for(j=1;j<=13;j++) {
		if(h[j]==0) {
			printf("H %d\n",j);
		}
	}
	for(j=1;j<=13;j++) {
		if(c[j]==0) {
			printf("C %d\n",j);
		}
	}
	for(j=1;j<=13;j++) {
		if(d[j]==0) {
			printf("D %d\n",j);
		}
	}
	return 0;
}