#include <stdio.h>

void dfs(char used[],char result[],int num,int n){
    int i;
    
    if(num==n){
        for(i=0;i<n-1;i++)printf("%d ",result[i]);
        printf("%d\n",result[i]);
        return;
    }
    
    for(i=1;i<=n;i++){
        if(used[i]==1)continue;
        
        result[num]=i;
        used[i]=1;
        
        dfs(used,result,num+1,n);
        
        used[i]=0;
    }
}
int main(){
    int i;
    
    int n;
    scanf("%d\n",&n);
    
    char used[n];
    for(i=0;i<n;i++)used[i]=0;
    char result[n];
    
    dfs(used,result,0,n);
    
    return 0;
}
