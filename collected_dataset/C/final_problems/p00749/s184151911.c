#include <stdio.h>
#include <string.h>

#define EPS 1e-7

int w,h;
char map[60][12];
char visited[60][12];
int okflag;

typedef struct {
	double x;
	int num;
} zyusin_t;

typedef struct {
	int min,max;
} range_t;

zyusin_t shindan_the_block(int x,int y);

void get_the_zyusin(range_t* range,zyusin_t* zyusin,int x,int y,char block) {
	if(x<0 || x>=w || y<0 || y>=h)return;
	if(visited[y][x] || map[y][x]!=block || !okflag)return;
	visited[y][x]=1;
	zyusin->x+=x+0.5;
	zyusin->num++;
	get_the_zyusin(range,zyusin,x-1,y,block);
	get_the_zyusin(range,zyusin,x+1,y,block);
	get_the_zyusin(range,zyusin,x,y-1,block);
	get_the_zyusin(range,zyusin,x,y+1,block);
	if(y>0 && map[y-1][x]!='.' && map[y-1][x]!=block && !visited[y-1][x]) {
		zyusin_t upzyusin;
		upzyusin=shindan_the_block(x,y-1);
		zyusin->x+=upzyusin.x*upzyusin.num;
		zyusin->num+=upzyusin.num;
	}
	if(y<h-1) {
		if(map[y+1][x]!='.' && map[y+1][x]!=map[y][x]) {
			if(x<range->min)range->min=x;
			if(range->max<x)range->max=x;
		}
	} else {
		if(x<range->min)range->min=x;
		if(range->max<x)range->max=x;
	}
}

zyusin_t shindan_the_block(int x,int y) {
	zyusin_t zyusin={0,1};
	range_t range;
	if(map[y][x]=='.')return zyusin;
	if(!okflag)return zyusin;
	zyusin.x=0;
	zyusin.num=0;
	range.min=range.max=x;
	get_the_zyusin(&range,&zyusin,x,y,map[y][x]);
	zyusin.x/=zyusin.num;
	if(zyusin.x<range.min+EPS || range.max+1<zyusin.x+EPS)okflag=0;
	return zyusin;
}

int main(void) {
	int i;
	while(scanf("%d%d",&w,&h)==2 && w!=0 && h!=0) {
		for(i=0;i<h;i++)scanf("%s",map[i]);
		memset(visited,0,sizeof(visited));
		okflag=1;
		for(i=0;i<w;i++) {
			if(map[h-1][i]!='.') {
				shindan_the_block(i,h-1);
				break;
			}
		}
		puts(okflag?"STABLE":"UNSTABLE");
	}
	return 0;
}