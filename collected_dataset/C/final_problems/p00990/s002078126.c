#include <stdio.h>

int n;
char id[100004];
int m;
char kouho[10];
int starNum;
int starPos[10];
char starMozi[10];

int getNonStarCheckSum(void) {
	int checkSum=0;
	int i;
	for(i=0;id[i];i++) {
		if(id[i]!='*') {
			int now=id[i]-'0';
			if((n-i)%2==0) {
				now*=2;
				if(now>=10)now=(now/10)+(now%10);
			}
			checkSum+=now;
		}
	}
	return checkSum;
}

int getStarCheckSum(void) {
	int checkSum=0;
	int i;
	for(i=0;i<starNum;i++) {
		int now=starMozi[i]-'0';
		if((n-starPos[i])%2==0) {
			now*=2;
			if(now>=10)now=(now/10)+(now%10);
		}
		checkSum+=now;
	}
	return checkSum;
}

int main(void) {
	int i;
	int nsc;
	int sc;
	int tansaku,tansakumax;
	int count;
	scanf("%d",&n);
	scanf("%s",id);
	scanf("%d",&m);
	for(i=0;i<m;i++) {
		char buf[4];
		scanf("%s",buf);
		kouho[i]=buf[0];
	}
	for(i=0;id[i];i++) {
		if(id[i]=='*')starPos[starNum++]=i;
	}
	nsc=getNonStarCheckSum();
	tansakumax=1;
	count=0;
	for(i=0;i<starNum;i++)tansakumax*=m;
	for(tansaku=0;tansaku<tansakumax;tansaku++) {
		int tansakubuf=tansaku;
		for(i=0;i<starNum;i++) {
			starMozi[i]=kouho[tansakubuf%m];
			tansakubuf/=m;
		}
		if((nsc+getStarCheckSum())%10==0)count++;
	}
	printf("%d\n",count);
	return 0;
}