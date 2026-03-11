#include<stdio.h>
#include<stdlib.h>
#define rep(i,n) for(int i=0;(i)<(n);(i)++)
#define NMAX 50
#define MMAX 50
int N,M;
int dx[4]={-1,0,1,0};
int dy[4]={0,1,0,-1};
int direction;
int count4,t,y,x;
typedef struct{
    char c[NMAX+1][MMAX+1];
    int id[NMAX+1][MMAX+1];
}MZ;
MZ maze;
int check(int y, int x){
    if(0>y || y>=N || 0>x || x>=M){//枠外
        return 0;
    }
    else if(maze.c[y][x]=='#'){//壁
        maze.id[y][x]=0;
        return 0;
    }
    else if(maze.id[y][x]>0 && maze.id[y][x]<t){//すでに通った
        return 0;
    }
    return 1;
}
void change(int n){
    if(n==0){//右折
        int x=dx[3],y=dy[3];
        for(int i=2;i>=0;i--){
            dx[i+1]=dx[i]; dy[i+1]=dy[i];
        }
        dx[0]=x; dy[0]=y;
    }
    else{//左折
        int x=dx[0],y=dy[0];
        for(int i=0;i<3;i++){
            dx[i]=dx[i+1]; dy[i]=dy[i+1];
        }
        dx[3]=x; dy[3]=y;
    }
}
int main(void){
    int i,j;
    while(1){
        scanf("%d%d",&N,&M);
        if(N==0 && M==0) break;
        for(i=0;i<N;i++) scanf("%s",maze.c[i]);
        rep(i,NMAX) rep(j,MMAX) maze.id[i][j]=-1;
        y=x=0; t=1;
        dx[0]=-1; dx[1]=0; dx[2]=1; dx[3]=0;
        dy[0]=0;  dy[1]=1; dy[2]=0; dy[3]=-1;
        int starty=0,startx=0;
        while(1){
            //int tmpy=y+dy[right],tmpx=x+dx[right];
            //printf("%d %d\n",x,y);
            if(check(y+dy[0],x+dx[0])){//右折
                change(0); y+=dy[1]; x+=dx[1];
            }
            else if(check(y+dy[1],x+dx[1])){//直進
                y+=dy[1]; x+=dx[1];
            }
            else if(check(y+dy[2],x+dx[2])){//左折
                change(1); y+=dy[1]; x+=dx[1];
            }
            else if(check(y+dy[3],x+dx[3])){//引き返す
                change(0); change(0); y+=dy[1]; x+=dx[1];
            }
            else{printf("NO\n"); break;}//閉じ込め
            
            if(t==4&&x==0&&y==0){printf("YES\n"); break;}
            if(y==starty && x==startx){printf("NO\n"); break;}//スタート地点
            else if(t==1&&y==N-1&&x==0){//宝
                t++; maze.id[y][x]=t;
                starty=N-1; startx=0;
            }
            else if(t==2&&y==N-1&&x==M-1){
                t++; maze.id[y][x]=t;
                starty=N-1; startx=M-1;
            }
            else if(t==3&&y==0&&x==M-1){
                t++; maze.id[y][x]=t;
                starty=0; startx=M-1;
            }
            else{
                maze.id[y][x]=t;
            }
        }
        /*rep(i,N){
            rep(j,M){
                printf("%2d ",maze.id[i][j]);
            }
            puts("");
        }*/
    }
    return 0;
}

