#include <stdio.h>

int num;
int cost[100];
int value[100];

int memo[100][20];

int search(int pos,int restD) {
	int res=0,nowres;
	if(pos>=num)return 0;
	if(memo[pos][restD]>0)return memo[pos][restD]-1;
	res=search(pos+1,restD);
	if(restD>=cost[pos]) {
		nowres=value[pos]+search(pos+1,restD-cost[pos]);
		if(nowres>res)res=nowres;
	}
	memo[pos][restD]=res+1;
	return res;
}

int main(void) {
	int D;
	int x,y;
	char buf[4];
	int score;
	int i;
	num=0;
	scanf("%d",&D);
	scanf("%d",&x);
	for(i=0;i<x;i++) {
		scanf("%s%d",buf,&score);
		if(buf[0]=='D') {
			cost[num]=1;
			value[num]=score;
			num++;
		}
	}
	scanf("%d",&y);
	for(i=0;i<y;i++) {
		scanf("%s%d",buf,&score);
		if(buf[0]=='D' && buf[1]=='D') {
			cost[num]=2;
			value[num]=score;
			num++;
		}
	}
	printf("%d\n",search(0,D));
	return 0;
}