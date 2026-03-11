#include <stdio.h>
#include <stdlib.h>
#define FOR(i,a,n) for(i=a;i<n;i++)
#define INF 1000000000

typedef struct{
	int x;
	int y;
}Pos;
typedef struct{
	int front;
	int rear;
	int size;
	Pos *data;
}Queue;
void init_queue(Queue *q,int s){
	q->data=calloc(s,sizeof(Pos));
	q->front=q->rear=0;
	q->size=s;
	return;
}
void end(Queue *q){
	free(q->data);
	return;
}
void push(Queue *q,int s,int t){
	q->data[q->rear].x=s,q->data[q->rear].y=t;
	q->rear=(q->rear+1)%q->size;
}
Pos pop(Queue *q){
	Pos res=q->data[q->front];
	q->front=(q->front+1)%q->size;
	return res;
}
int isempty(Queue *q){
	return q->front==q->rear;
}
char map[810][810];
int d[810][810];
int dx[4]={1,0,-1,0};
int dy[4]={0,1,0,-1};
int sx,sy;
void read(int w,int h,char c){
	int i,j;
	scanf("%*c");
	FOR(i,0,h+2) map[0][i]=c,map[w+1][i]=c;
	FOR(i,0,w+2) map[i][0]=c,map[i][h+1]=c;
	FOR(i,1,h+1){
		FOR(j,1,w+1){
	        scanf("%c",&map[j][i]);
	        if(map[j][i]=='S') sx=j,sy=i,map[j][i]='.';
	    }
        scanf("%*c");
    }
    return; 
}
Queue que;
int k;
void dfs(int x,int y,int n){
	int i,nx,ny;
	d[x][y]=0;
	push(&que,x,y);
	if(n==k) return;
	FOR(i,0,4){
		nx=x+dx[i],ny=y+dy[i];
		if(map[nx][ny]=='.'&&d[nx][ny]==INF){
			dfs(nx,ny,n+1);
		}
	}
	return;
}
int main(void){
	int h,w,nx,ny,i,j;
	Pos p;
	scanf("%d%d%d",&h,&w,&k);
	FOR(i,1,h+1){
		FOR(j,1,w+1){
			d[j][i]=INF;
		}
	}
	read(w,h,'@');
	init_queue(&que,h*w*2);
	dfs(sx,sy,0);
	while(1){
		p=pop(&que);
		if(p.x==1||p.y==1||p.x==w||p.y==h){
			printf("%d\n",(d[p.x][p.y]+k-1)/k+1);
			return 0;
		}
		FOR(j,0,4){
			nx=p.x+dx[j],ny=p.y+dy[j];
			if(d[nx][ny]==INF&&map[nx][ny]!='@'){
				d[nx][ny]=d[p.x][p.y]+1;
				push(&que,nx,ny);
			}
		}
	}
	return 0;
}


    