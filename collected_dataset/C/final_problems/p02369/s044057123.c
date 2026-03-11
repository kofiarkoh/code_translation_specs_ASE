#include <stdio.h>
#include <stdlib.h>

#define WHITE 1
#define GREY 2
#define BLACK 3

int adj[100][1000]={{}, {}};
int color[100]={};
int parent[100]={}; 
int flag=0; // Have a closed circuit 1, otherwise 0;
int E,V;

void dfs_visit(int u);

int main(){
    int u, v;
    int i;
    
    //input
    scanf("%d %d", &V, &E);
    for(i=0;i<E;i++){
        scanf("%d %d", &u, &v);
        if(u != v){
            adj[u][v] = 1;
            color[u] = WHITE;
            color[v] = WHITE;
        }
    }

    for(u=0;u<V;u++){
        if(color[u] == WHITE) dfs_visit(u);
    }

    printf("%d\n", flag);

    return 0;
}

// DFS check
void dfs_visit(int u){
    int v;

    color[u]=GREY;

    for(v=0;v<E;v++){
        if(adj[u][v]==1){
            if(color[v]==WHITE){
                parent[v]=u;
                dfs_visit(v);
            }
            if(color[v]==GREY){
                flag=1;
                break;
            }
        }
    }

    color[u]=BLACK;
}
