#include <stdio.h>
#include <string.h>
#include <math.h>


struct data{int v,next;} a[400];
int vis[110],b[110][110],Q[110],H[110],cnt,N,M,x,y,d,ans,i,j;

int add(int u,int v){
    a[++cnt].v=v; a[cnt].next=H[u]; H[u]=cnt;
}

int calc(int c){
    int i,j,d,tt,s=0;
    for (i=1;i<=c;i++){
        d=100000000;
        for (j=1;j<=c;j++)
            if (i!=j && b[Q[i]][Q[j]]<d) d=b[Q[i]][Q[j]];
        s+=d;
    }
    if (s>ans) ans=s;
}

int dfs(int c,int u){
    int i,bo,tt,v;
    if (c>1) calc(c);
    for (tt=H[u];tt;tt=a[tt].next){
        v=a[tt].v;
        if (vis[v]) continue;
        bo=1;
        for (i=1;i<=c;i++)
            if (!b[v][Q[i]]){
                bo=0; break;
            }
        if (bo){
            vis[v]=1; Q[c+1]=v; dfs(c+1,v); vis[v]=0;
        }
    }
}

int main()
{
    while (~scanf("%d%d",&N,&M)){
        memset(b,0,sizeof(b));
        memset(H,0,sizeof(H));
        cnt=ans=0; 
        for (i=1;i<=M;i++){
            scanf("%d%d%d",&x,&y,&d);
            b[x][y]=b[y][x]=d; 
            add(x,y); add(y,x);
        }
        for (i=1;i<=N;i++){
            Q[1]=i;  vis[i]=1;
            dfs(1,i);  vis[i]=0;
        }
        printf("%d\n",ans);
    }
}