#include <stdio.h>

char isprime[100000];
int pr[100000];
int primenum;

void mpl(void) {
	int i,j;
	for(i=0;i<100000;i++)isprime[i]=1;
	isprime[0]=isprime[1]=0;
	primenum=0;
	for(i=2;i<100000;i++) {
		if(isprime[i]) {
			pr[primenum++]=i;
			for(j=i+i;j<100000;j+=i)isprime[j]=0;
		}
	}
}

int main(void) {
	int m,a,b;
	mpl();
	while(scanf("%d%d%d",&m,&a,&b)==3 && (m|a|b)!=0) {
		int i,j;
		int p=0,q=0;
		for(i=0;i<primenum;i++) {
			for(j=i;j<primenum;j++) {
				if(pr[i]*pr[j]>m || a*pr[j]>pr[i]*b)break;
				if(pr[i]*pr[j]>p*q) {
					p=pr[i];
					q=pr[j];
				}
			}
		}
		printf("%d %d\n",p,q);
	}
	return 0;
}