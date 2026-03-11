#include<stdio.h>
#define rep(i,n) for(int i=0;(i)<(n);(i)++)
int map[20][20];
int starty,startx;
int dx[4]={ 0,1,0,-1};
int dy[4]={-1,0,1, 0};
int min;
int w,h;
void dfs(int i,int y,int x){
    if(i==11) return;
    /*printf("i:%d y:%d x:%d\n",i,y,x);
    printf("i:%d\n",i);
    rep(a,h) {
        rep(b,w){
            if(a==y&&b==x) printf("@ ");
            else printf("%d ",map[a][b]);
        }puts("");
    }puts("");*/
    for(int d=0;d<4;d++){
        int tmpy=y+dy[d],tmpx=x+dx[d];
        if(tmpy<0||tmpy>=h||tmpx<0||tmpx>=w) continue;
        else if(map[tmpy][tmpx]==1) continue;
        else{
            int ny=y,nx=x;
            while(1){
                ny+=dy[d]; nx+=dx[d];
                if(ny<0||ny>=h||nx<0||nx>=w) break;
                else if(map[ny][nx]==3){
                    if(min>i) min=i;
                    return;
                }
                else if(map[ny][nx]!=1) continue;
                else{
                    map[ny][nx]=0;
                    dfs(i+1,ny-dy[d],nx-dx[d]);
                    map[ny][nx]=1;
                    break;
                }
            }
        }
    }
}
int main(void){
    while(1){
        scanf("%d%d",&w,&h);
        if(w+h==0) break;
        rep(i,h) rep(j,w){
            scanf("%d",&map[i][j]);
            if(map[i][j]==2) starty=i,startx=j;
        }
        //printf("%d %d\n",starty,startx);
        min=10000;
        dfs(1,starty,startx);
        if(min==10000) printf("-1\n");
        else printf("%d\n",min);
    }
}

