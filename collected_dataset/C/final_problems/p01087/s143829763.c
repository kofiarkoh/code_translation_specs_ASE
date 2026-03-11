#include<stdio.h>
#include<string.h>
int n;
char s[20][20];
int len[20];
int dfs(int d,int k,int e){
    int res=0;
    if(e)res=1;
    for(int i=d;i<n&&k<len[i];i++){
        if(s[i][k]=='.')continue;
        int a=s[i][k]-'0';
        if(s[i][k]=='+')a=dfs(i+1,k+1,0);
        if(s[i][k]=='*')a=dfs(i+1,k+1,1);
        if(e)res*=a;
        else res+=a;
    }
    return res;
}int main(void){
    while(1){
        scanf("%d",&n);
        if(n==0) break;
        for(int i=0;i<n;i++){
            scanf("%s",s[i]);
            len[i]=strlen(s[i]);
            //printf("%d\n",len[i]);
        }
        if(n==1) printf("%d\n",s[0][0]-'0');
        else{
            printf("%d\n",dfs(0,0,0));
        }
    }
}

