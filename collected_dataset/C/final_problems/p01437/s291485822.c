#include<stdio.h>
#include<string.h>
#define min(x,y) ((x<y)?x:y)
char map[110][110];
char map2[5][110][110];
typedef struct p{
	int x,y;
}P;
typedef struct situation{
	int x,y,d;
}situation;
situation c,s[1000000],n;
int dx[]={1,0,-1,0};
int dy[]={0,1,0,-1};
int rotate(char  n){
	if(n==0)return 1;
	if(n==1)return 2;
	if(n==2)return 3;
	if(n==3)return 0;
}
int main(){
	int i,j,k,h,w,f;
	long long l,z,sn;
	while(1){
		scanf("%d%d%lld",&h,&w,&l);
		if(h==0)break;
		for(i=1;i<=h;i++){
			scanf("%s",map[i]+1);
			for(j=1;j<=w;j++){
				if(map[i][j]=='E')c=(situation){j,i,0},map[i][j]='.';
				else if(map[i][j]=='N')c=(situation){j,i,3},map[i][j]='.';
				else if(map[i][j]=='W')c=(situation){j,i,2},map[i][j]='.';
				else if(map[i][j]=='S')c=(situation){j,i,1},map[i][j]='.';
			}
		}
		for(i=0;i<=w+1;i++)map[0][i]=map[h+1][i]='#';
		for(i=0;i<=h+1;i++)map[i][0]=map[i][w+1]='#';
		//for(i=0;i<=h+1;i++)printf("%s\n",map[i]);
		sn=0;f=0;
		s[0]=c;
		memset(map2,0,sizeof(map2));
		while(1){
			n.x=s[sn].x+dx[s[sn].d];
			n.y=s[sn].y+dy[s[sn].d];
			n.d=s[sn].d;
			if(map[n.y][n.x]=='.'){
				s[sn].d=rotate(rotate(rotate(s[sn].d)));
				s[++sn]=n;
				if(map2[n.d][n.y][n.x]==1){
					for(i=0;i<sn;i++){
						if(s[i].x==s[sn].x && s[i].y==s[sn].y && s[i].d==s[sn].d)goto nex;
					}
				}
				map2[n.d][n.y][n.x]=1;
				//printf("  %d %d %d\n",s[sn].y,s[sn].x,s[sn].d);
				//for(i=0;i<sn;i++){
				//	if(s[i].x==s[sn].x && s[i].y==s[sn].y && s[i].d==s[sn].d)goto nex;
				//}
				if(sn==l){
					f=1;
					break;
				}
			}else{
				s[sn].d=rotate(s[sn].d);
				//printf("   %d %d %d\n",s[sn].y,s[sn].x,s[sn].d);
			}
		}
		nex:
		if(f==0)z=(l-i)%(sn-i)+i;
		else z=sn;
		printf("%d %d ",s[z].y,s[z].x);
		if(s[z].d==0)printf("E\n");
		else if(s[z].d==1)printf("S\n");
		else if(s[z].d==2)printf("W\n");
		else if(s[z].d==3)printf("N\n");
	}
	return 0;
}