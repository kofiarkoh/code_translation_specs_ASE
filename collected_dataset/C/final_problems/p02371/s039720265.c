#include <stdio.h>
#include <string.h>
#define INF 999999
struct node{
    int to,next,w;
}e[100100];
int cnt=0,head[100100],n,d[100100],q[100100];
void addedge(int u,int v,int w){//?接表 
    cnt++,e[cnt].to=v,e[cnt].next=head[u],e[cnt].w=w,head[u]=cnt;
}
void bfs(int x){
    int h,t,i,u,v,b;
    for(i=0;i<n;i++)d[i]=INF;
    h=t=1;
    q[t]=x;
    t++;
    d[x]=0;
    while(h<t){
        u=q[h];
        for(b=head[u];b;b=e[b].next){
            v=e[b].to;
            if(d[v]==INF){
                d[v]=d[u]+e[b].w;
                q[t]=v;
                t++;
            }
        }
        h++;
    }
}
int main(){
    int i,u,v,w,k,maxv;
    memset(head,0,sizeof(head));
    scanf("%d",&n);
    for(i=1;i<n;i++){
        scanf("%d%d%d",&u,&v,&w);
        addedge(u,v,w);//无向? 
        addedge(v,u,w);
    }
    bfs(0);
    maxv=-1;
    for(i=0;i<n;i++)
        if(d[i]!=INF&&maxv<d[i]){
            maxv=d[i];
            k=i;
        }
    bfs(k);
    maxv=-1;
    for(i=0;i<n;i++)
        if(d[i]!=INF&&maxv<d[i])
            maxv=d[i];
    printf("%d\n",maxv);//1 一个想不到的??，此?写成 printf("%d\n",d[i]); 竟然?了会 
    return 0;
}