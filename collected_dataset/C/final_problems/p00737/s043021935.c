//SPFA
#include <stdio.h>
#include <string.h>
#define INF (9*900)
#define rep(i,n) for(i=0;i<(n);i++)
int dx[]={1,0,-1,0};
int dy[]={0,1,0,-1};

struct Node{
	int y,x,dir;
};

void _a(struct Node *a){a->y+=dy[a->dir];a->x+=dx[a->dir];}
void _b(struct Node *a){a->dir=(a->dir+1)%4;_a(a);}
void _c(struct Node *a){a->dir=(a->dir+2)%4;_a(a);}
void _d(struct Node *a){a->dir=(a->dir-1+4)%4;_a(a);}
void _e(struct Node *a){}
void (*action[5])(struct Node *)={_a,_b,_c,_d,_e};

int in(int y,int x,int h,int w){
	return 0<=y&&y<h&&0<=x&&x<w;
}
int main(){
	struct Node a,b,que[900];
	int head,tail;
	int inQ[30][30][4],dst[30][30][4];
	
	int w,h,s[30][30],c[4];
	int i,j,k,ans;
	while(scanf("%d%d",&w,&h)==2&&(w||h)){
		rep(i,h)
			rep(j,w)scanf("%d",&s[i][j]);
		rep(i,4)scanf("%d",c+i);
		memset(dst,-1,sizeof(dst));
		memset(inQ,0,sizeof(inQ));
		rep(i,h)rep(j,w)rep(k,4)dst[i][j][k]=INF;
		a.y=a.x=a.dir=0;
		head=tail=0;
		dst[a.y][a.x][a.dir]=0;
		que[tail++]=a;
		while(head<tail){
			a=que[head++];
			inQ[a.y][a.x][a.dir]=0;
			b=a;
			action[s[b.y][b.x]](&b);
			if(in(b.y,b.x,h,w) && dst[a.y][a.x][a.dir] < dst[b.y][b.x][b.dir]){
				dst[b.y][b.x][b.dir] = dst[a.y][a.x][a.dir];
				if(inQ[b.y][b.x][b.dir]==0)que[tail++]=b;
			}
			rep(i,4){
				b=a;
				action[i](&b);
				if(in(b.y,b.x,h,w) && dst[a.y][a.x][a.dir]+c[i] < dst[b.y][b.x][b.dir]){
					dst[b.y][b.x][b.dir] = dst[a.y][a.x][a.dir]+c[i];
					if(inQ[b.y][b.x][b.dir]==0)que[tail++]=b;
				}
			}
		}
		ans=INF;
		rep(i,4){
			if(dst[h-1][w-1][i] < ans)ans = dst[h-1][w-1][i];
		}
		printf("%d\n",ans);
	}
}