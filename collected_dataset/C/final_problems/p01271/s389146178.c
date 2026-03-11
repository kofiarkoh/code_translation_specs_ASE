#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#define N 100000000
#define rep(i,n) for(int i = 0 ; i < n ; ++i)

typedef struct{
    int rx,ry,lx,ly;
}Point;
Point queue[N];
int w,h;

int chk(int y,int x,char s[99][99]){
    return (y >= 0 && x >= 0 && y < h && x < w && s[y][x] != '#');
}

char rin[99][99] , len[99][99];

int used[52][52][52][52];

int rx,ry,lx,ly;

const int dx[] = {0,-1,0,1};
const int dy[] = {-1,0,1,0};

int head,num;
void enqueue(int ly,int lx,int ry,int rx){
    if(num<N){
        queue[(head+num)%N].ly=ly;
        queue[(head+num)%N].lx=lx;
        queue[(head+num)%N].ry=ry;
        queue[(head+num)%N].rx=rx;
        num++;
    }else{
        printf("エラー");
        exit(1);
    }
}
Point dequeue(){
    Point d={-1,-1,-1,-1};
    if(num>0){
        d=queue[head];
        head=(head+1)%N;
        num--;
    }
    return d;
}

int bfs(){
    
    memset(used,0,sizeof(used));
    enqueue(ly,lx,ry,rx);
    
    while(1){
        Point p=dequeue();
        if(p.ly==-1) break;
        
        if(used[p.ly][p.lx][p.ry][p.rx])continue;
        used[p.ly][p.lx][p.ry][p.rx] = 1;
        
        if(len[p.ly][p.lx] == '%' && rin[p.ry][p.rx] == '%')return 1;
        if(len[p.ly][p.lx] == '%' || rin[p.ry][p.rx] == '%')continue;
        
        rep(i,4){
            
            int nrx = p.rx,nry = p.ry,nlx = p.lx,nly = p.ly;
            
            if(chk(nly+dy[i],nlx+dx[i],len))nly += dy[i],nlx += dx[i];
            if(chk(nry+dy[i],nrx-dx[i],rin))nry += dy[i],nrx -= dx[i];
            
            
            if(!used[nly][nlx][nry][nrx])
                enqueue(nly,nlx,nry,nrx);
        }
    }
    
    return 0;
}

int main(){
    while(1){
        scanf("%d%d",&w,&h);
        if(w+h==0) break;
        getchar();
        rep(i,h) scanf("%s%s",len[i],rin[i]);
        rep(i,h){
            rep(j,w){
                if(len[i][j] == 'L')ly = i,lx = j;
                if(rin[i][j] == 'R')ry = i,rx = j;
            }
        }
        head=num=0;
        printf("%s\n",(bfs() ? "Yes": "No"));
        
    }
}

