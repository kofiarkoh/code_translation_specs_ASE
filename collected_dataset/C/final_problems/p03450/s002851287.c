#include <stdio.h>
#include <stdlib.h>
int g[100001];
int di[100001];

int dfs(int i){
    if(g[i]!=i){
        int r = dfs(g[i]);
        di[i]+=di[g[i]];
        //printf("di[%d]=%d\n",i,di[i]);
        return r;
    }else{
        return i;
    }
}
int main(){
    int i,j;
    int N,M;
    scanf("%d %d",&N,&M);
    for(i=1;i<=N;i++)g[i]=i;
    for(i=0;i<M;i++){
        int l,r,d;
        scanf("%d %d %d",&l,&r,&d);
        int g1=dfs(l);
        int g2=dfs(r);
        if(g1!=g2){
            g[g2]=l;
            di[g2]=d-di[r];
        }else{
            //printf("di[%d](%d)-di[%d](%d)!=%d\n",r,di[r],l,di[l],d);
            if(di[r]-di[l]!=d){
                printf("No\n");
                return 0;
            }
        }
    }
    //printf("%d %d %d\n",di[1],di[2],di[3]);
    printf("Yes\n");
}
