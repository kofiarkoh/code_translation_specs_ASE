#include <stdio.h>
int n,u,v,terminalnum;
int a[100001],b[100001],parent[100001],depth[100001],terminal[100001],vdepth[100001];

void check_root_terminal(int rootnode){
    int i, foundnum=0;
    for(i=0;i<n-1;i++){
        if(a[i]==rootnode || b[i]==rootnode)
            foundnum++;
    };
    if(foundnum==1){
        terminal[terminalnum]=rootnode;
        terminalnum++;
    };
    
};
    

void find_next_node(int currentnode, int currentdepth){
    int i,found=0;
    //printf("find_next_node(%d,%d)\n",currentnode,currentdepth);
    if(currentdepth>n){
        //printf("Error: depth=%d\n",currentdepth);
        return;
    }
    for(i=0;i<n-1;i++){
        if(a[i]==currentnode && b[i]!=parent[currentnode]){
            found=1;
            parent[b[i]]=currentnode;
            depth[b[i]]=currentdepth+1;
            find_next_node(b[i],currentdepth+1);
        };
        if(b[i]==currentnode && a[i]!=parent[currentnode]){
            found=1;
            parent[a[i]]=currentnode;
            depth[a[i]]=currentdepth+1;
            find_next_node(a[i],currentdepth+1);
        };
    };
    if(found==0){
        terminal[terminalnum]=currentnode;
        terminalnum++;
    };
};    

int main(void){
    int i, longestrun=0;
    scanf("%d %d %d",&n,&u,&v);
    u--;v--;
    //printf("%d %d %d \n",n,u,v);
    for(i=0;i<n-1;i++){
        scanf("%d %d",&a[i],&b[i]);
        a[i]--;b[i]--;
        //printf("vertex %d: (%d,%d)\n",i, a[i],b[i]);
    };
    depth[v]=0;
    parent[v]=-1;
    terminalnum=0;
    check_root_terminal(v);
    find_next_node(v,depth[v]);
    //printf("terminalnum=%d \n",terminalnum);
    vdepth[v]=0;
    for(i=0;i<terminalnum;i++){
        //printf("from v, terminal node %d: %d, depth=%d\n",i+1,terminal[i],depth[terminal[i]]);
        vdepth[terminal[i]]=depth[terminal[i]];
    }
    // 初期化は省略
    depth[u]=0;
    parent[u]=-1;
    terminalnum=0;
    check_root_terminal(u);
    find_next_node(u,depth[u]);
    for(i=0;i<terminalnum;i++){
        //printf("from u, terminal node %d: %d, depth=%d\n",i+1,terminal[i],depth[terminal[i]]);
        if(vdepth[terminal[i]]>depth[terminal[i]])
            if(longestrun<vdepth[terminal[i]]-1)
                longestrun=vdepth[terminal[i]]-1;
    } 
    printf("%d",longestrun);
    return 0;
};
