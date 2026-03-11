#include <stdio.h>
#define WHITE 0
#define GRAY 1 
#define BLACK 2
typedef struct{
    int u;
    int k;
    int a[100];
    int color;
    int d;
    int f;
}rist;

void dfs(void);
void visit(int);
int n,time;
rist G[100];
int main(){
    int i,j;
    scanf("%d",&n);
    for(i=0;i<n;i++){
        scanf("%d%d",&G[i].u,&G[i].k);
        for(j=0;j<G[i].k;j++){
            scanf("%d",&G[i].a[j]);
        }
    }

    dfs();

    for(i=0;i<n;i++){
        printf("%d %d %d\n",G[i].u,G[i].d,G[i].f);
    }
    return 0;
}

void dfs(){
    int i;
    for(i=0;i<n;i++){
        G[i].color=WHITE;
    }
    time=0;
    for(i=0;i<n;i++){
        if(G[i].color==WHITE) visit(i);
    }
}

void visit(int u){
    int i,j;
    G[u].color=GRAY;
    G[u].d=++time;
    for(i=0;i<G[u].k;i++){
        for(j=0;j<n;j++){
            if(G[u].a[i]==G[j].u) if(G[j].color==WHITE) visit(j);
        }
    }
    G[u].color=BLACK;
    G[u].f=++time;
}
