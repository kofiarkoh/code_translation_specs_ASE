#include <stdio.h>
#include <string.h>

int width,height;
int result;
char map[102][103];
int map2[102][102];

void tansaku(int x,int y,int now_cost,int is_hori) {
	int here_is_hori;
	if(x<0 || x>width+1 || y<0 || y>height+1)return;
	if(is_hori && map[y][x]!='#')now_cost++;
	if(map2[y][x]<=now_cost)return;
	map2[y][x]=now_cost;
	here_is_hori=(map[y][x]=='#');
	tansaku(x-1,y,now_cost,here_is_hori);
	tansaku(x,y-1,now_cost,here_is_hori);
	tansaku(x+1,y,now_cost,here_is_hori);
	tansaku(x,y+1,now_cost,here_is_hori);
}

int main(void) {
	int i,j;
	while(1) {
		scanf("%d%d",&width,&height);
		if(width==0 && height==0)break;
		memset(map,0,sizeof(map));
		for(i=1;i<=height;i++) {
			scanf("%s",&map[i][1]);
			map[i][0]='.';
			map[i][width+1]='.';
		}
		for(i=0;i<=width+1;i++) {
			map[0][i]='.';
			map[height+1][i]='.';
			for(j=0;j<=height+1;j++) {
				map2[j][i]=0x7fffffff;
			}
		}
		tansaku(0,0,0,0);
		for(i=1;i<=height;i++) {
			for(j=1;j<=width;j++) {
				if(map[i][j]=='&') {
					printf("%d\n",map2[i][j]);
					i=width+1;
					break;
				}
			}
		}
	}
	return 0;
}