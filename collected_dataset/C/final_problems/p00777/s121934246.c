#include <stdio.h>
#include <string.h>
int map[805][805]={0};
int p[805];
int c, n, max;
int cnt[805][805] = {0};
int type[805] = {0};			// 0:leaf
int vis[805]={0};
int leaf;
int d[805] = {0};

void init(){
	c = 0;
	leaf = 0;
	max = 0;
	for(int i=2;i<=n;i++)	//1<=pi<i
	{
		scanf("%d", &p[i]);
		cnt[i][++cnt[i][0]] = p[i];			// 隣接の点のデータを 
		cnt[p[i]][++cnt[p[i]][0]] = i;		//記入
	} 
	for(int i=2;i<=n;i++)
	{
		scanf("%d", &d[i]);
		map[i][p[i]] = map[p[i]][i] = d[i];
		c += d[i];
	}
	for(int i=1;i<=n;i++)
	{
		if(cnt[i][0] > 1)
			type[i] = 1;
		else
			leaf += map[i][cnt[i][1]];	
	}
	c -= leaf;
}

void reset(){
	memset(cnt,0,sizeof(cnt));
	memset(map,0,sizeof(map));
	memset(type,0,sizeof(type));
}

void dfs(int now, int dis){
	vis[now] = 1;
	for(int i=1;i<=cnt[now][0];i++){
		if(vis[cnt[now][i]] != 1 && type[cnt[now][i]] != 0)
			dfs(cnt[now][i],dis+map[now][cnt[now][i]]);
	}
	vis[now] = 0;			
	if(dis > max) max = dis; 
}

int main(){ 
	while(scanf("%d",&n), n){
		init();
		for(int i=1;i<=n;i++){					//葉と隣接する 
			if(type[i] == 0){ 					//ところから最大直径 
				dfs(cnt[i][1], 0);				//を計算 
			}
		}
		printf("%d\n", leaf + 3*c - max);
		reset();
	}
} 
