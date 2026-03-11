#include <stdio.h>
signed main(){
    int i,x,n,p[105];
    scanf("%d %d",&x,&n);
    for(i=0;i<n;i++){
        scanf("%d",&p[i]);
    }
    int flag[105]={};
    int ans=0;
    for(i=0;i<n;i++){
        flag[p[i]]=1;
    }
    for(i=1;i<=101;i++){
        if(flag[i]==0){
            int abs1,abs2;
            abs1=x-ans;
            abs2=x-i;
            if(abs1<0){
                abs1=abs1*-1;
            }
            if(abs2<0){
                abs2=abs2*-1;
            }
            if(abs1>abs2){
                ans=i;
            }
        }
    }
    printf("%d\n",ans);
    return(0);
}