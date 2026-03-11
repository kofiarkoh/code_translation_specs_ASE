//RUPC-C
#include<stdio.h>
char map[1005][1005];
int ff=0;
int dx[]={1,0,-1,0};
int dy[]={0,1,0,-1};
int dfs(int y,int x){
	int i,j,nx,ny,f=0;
	if(map[y][x]=='t'){
		ff=1;
		//printf("TTTT\n");
		return 1;
	}
	map[y][x]='#';
	for(i=0;i<4;i++){
		nx=x+dx[i];
		ny=y+dy[i];
		if(map[ny][nx]!='#' && map[ny][nx]!='1'){
			//if(f==0)f=dfs(ny,nx);
			//else dfs(ny,nx);
			if(dfs(ny,nx)==1){
				//break;
				f=1;
				map[y][x]='1';
				return 1;
			}
		}
	}
	map[y][x]='1';
	return f;
}
int main(){
	int i,j,w,h,n,ans=0,f=0;
	int x,y,sx,sy;
	scanf("%d%d",&w,&h);
	for(i=0;i<=w+1;i++)map[0][i]=map[h+1][i]='#';
	for(i=1;i<=h;i++){
		scanf("%s",map[i]+1);
		map[i][0]=map[i][w+1]='#';
	}
	//for(i=0;i<=h+1;i++)printf("%s\n",map[i]);
	//dfs(1,1);
	dfs(1,1);
	if(ff==1){
		printf("%d\n",ans);
		ff++;
	}
	scanf("%d",&n);
	for(i=0;i<n;i++){
		ans++;
		scanf("%d%d",&x,&y);
		x++;
		y++;
		sx=sy=1;
		map[y][x]='.';
		for(j=0;j<4;j++)
			if(map[y+dy[j]][x+dx[j]]=='1')break;
		if(j<4)sx=x,sy=y;
		//printf("\n");
		//for(j=0;j<=h+1;j++)printf("%s\n",map[j]);
		//if(dfs(sy,sx)==1)printf("%d\n",ans);
		//dfs(sy,sx);
		//printf("ff:%d\n",ff);
		if(ff==0)dfs(sy,sx);
		if(ff==1){
			printf("%d\n",ans);
			ff++;
		}
	}
	if(ff==0)printf("-1\n");
	return 0;
}