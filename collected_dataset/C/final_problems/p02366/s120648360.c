#include <stdio.h>
#include <string.h>
int n,m,head[100100],cnt=0,parent[100100],dfn[100100],low[100100],vis[100100],tag=0,book[100100],count=0;
struct node{
    int to,next;
}e[100000*2+100];//无向?
void addedge(int u,int v){
    cnt++,e[cnt].to=v,e[cnt].next=head[u],head[u]=cnt;
}
void dfs(int current,int previous){//?置 dfn[] low[] 
    int next,b;//next点 b? 
    vis[current]=1;
    //parent[current]=previous;//1 此句?除 
    tag++;
    dfn[current]=low[current]=tag;
    for(b=head[current];b;b=e[b].next){
        next=e[b].to;
        if(vis[next]==0){
            parent[next]=current;//1 添加此句 
            dfs(next,current);
            low[current]=low[current]>low[next]?low[next]:low[current];
        }else if(next!=previous){
            low[current]=low[current]>dfn[next]?dfn[next]:low[current];
        }
    }
} 
void art_point(){
    int i,p,p_num=0;
    for(i=0;i<n;i++){
        p=parent[i];
        if(p==0)p_num++;
        else if(p>0&&dfn[p]<=low[i])book[p]=1;
    }
    if(p_num>1)book[0]=1;
}
int main(){
    int i,u,v;
    memset(vis,0,sizeof(vis));
    memset(head,0,sizeof(head));
    scanf("%d%d",&n,&m);
    for(i=1;i<=m;i++){
        scanf("%d%d",&u,&v);
        addedge(u,v);
        addedge(v,u);
    }
    parent[0]=-1;
    dfs(0,-1);
    art_point();
    for(i=0;i<n;i++)
        if(book[i])
            printf("%d\n",i);
    return 0;
} 
