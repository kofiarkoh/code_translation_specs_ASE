#include <stdio.h>
#include <stdlib.h>

#define INF 10100
#define Q 10001

int *p;
int *rank;
int *weight; // weight_x-weight_p[x]. difference weight.

void relate(int, int, int);
void makeSet(int);
void link(int, int,int);
int Same(int,int);
int findSet(int);
int diff(int, int);

int main(){
    int n,q;
    int com, x, y, z;
    int *ans;
    int i;

    scanf("%d %d", &n, &q);

    // create S
    p = (int *)malloc(n*sizeof(int));
    rank = (int *)malloc(n*sizeof(int));
    weight = (int *)malloc(n*sizeof(int));

    // create array answer
    ans = (int *)malloc(q*sizeof(int));
    for(i=0;i<q;i++){
        ans[i] = INF;
    }

    // make Set
    for(i=0;i<n;i++){
        makeSet(i);
    }

    for(i=0;i<q;i++){
        scanf("%d", &com);
        if(com == 0){
            scanf("%d %d %d", &x,&y,&z);
            relate(x, y, z);
        }
        else if(com == 1){
            scanf("%d %d", &x,&y);
            ans[i] = diff(x, y);
        }
    }

    // output
    for(i=0;i<q;i++){
        if(ans[i]!=INF){
            if(ans[i]==Q) printf("?\n");
            else printf("%d\n", ans[i]);
        }
    }

    return 0;
}

void makeSet(int x){
    p[x] = x;
    rank[x]=0;
    weight[x]=0; 
}

void relate(int x, int y, int z){
    int rx,ry;

    rx = findSet(x);
    ry = findSet(y);

    if(rx==ry) exit(0); // false
    
    if(rank[rx] > rank[ry]){
        p[ry] = rx;
        weight[ry]=z+weight[x]-weight[y];
    }
    else{
        p[rx]=ry;
        weight[rx]=-z+weight[y]-weight[x];
        if(rank[rx]==rank[ry]) rank[ry]=rank[ry]+1;
    }
}

int findSet(int x){ // return root.
    if(x!=p[x]){
        p[x]=findSet(p[x]);
        weight[x]+=weight[p[x]];
    }
    return p[x];
}

int Same(int x, int y){
    if(findSet(x)==findSet(y)) return 1;
    else return 0;
}

int diff(int x, int y){
    if(Same(x,y)==0) return Q;
    else return weight[y]-weight[x];
}
