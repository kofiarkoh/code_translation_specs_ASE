#include <stdio.h>
#include<string.h> 

int main(void){
    int n,m,testpe[100000]={0},pe=0,cl=0,a[100000],c[100000]={0};
    char pf[100000][3];
    
    scanf("%d%d",&n,&m);
    
    for(int i=0;i<m;i++){
       scanf("%d%s",&a[i],pf[i]);
    }
    for(int i=0;i<m;i++){
        if(!c[a[i]]){
            if(strcmp("WA",pf[i])==0){testpe[a[i]]++;}
            else if(strcmp("AC",pf[i])==0) {c[a[i]]=1;cl++;}
        }
    }
    for(int i=0;i<n;i++){
        if(a[i]){pe+=testpe[a[i]];}
        
    }
    printf("%d %d",cl,pe);
    return 0;
}
