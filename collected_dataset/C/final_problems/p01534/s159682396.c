#include <stdio.h>

int n,m;
int map[50][3];

/* [song][new][lv1] */
int memo[51][51][51];
char visited[51][51][51];

int getscore(int song,int lv2,int lv1) {
	if(lv2==0 && lv1==0)return map[song][2];
	if(lv2+lv1>8)return -0x7fffffff;
	return map[song][0]*lv2+map[song][1]*lv1;
}

int tansaku(int song,int rest,int lv1) {
	int result=-0x7fffffff;
	int oru;
	int i,j;
	if(song>=n)return 0;
	if(visited[song][rest][lv1])return memo[song][rest][lv1];
	for(oru=0;oru<=8 && oru<=rest;oru++) {
		int nowresult=-0x7fffffff;
		for(i=0;i<=oru;i++) {
			for(j=0;j<=lv1;j++) {
				int tempresult;
				tempresult=getscore(song,i,j);
				if(tempresult>nowresult)nowresult=tempresult;
			}
		}
		nowresult+=tansaku(song+1,rest-oru,oru);
		if(nowresult>result)result=nowresult;
	}
	visited[song][rest][lv1]=1;
	memo[song][rest][lv1]=result;
	return result;
}

int main(void) {
	int i;
	scanf("%d%d",&n,&m);
	for(i=0;i<n;i++) {
		scanf("%d%d%d",&map[i][0],&map[i][1],&map[i][2]);
	}
	printf("%d\n",tansaku(0,m,0));
	return 0;
}