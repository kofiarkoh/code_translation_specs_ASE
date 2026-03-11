#include <stdio.h>
#include <string.h>
int main(void){
    int n,i,tp=0,hp=0;
    char s[128],t[128];
    
    scanf("%d",&n);
    
    for(i=0;i<n;i++){
        scanf("%s %s",s,t);
        
        if(strcmp(s,t)>0){
            tp+=3;
        }else if(strcmp(s,t)<0){
            hp+=3;
        }else{
            tp++;
            hp++;
        }
        
    }
    
    printf("%d %d\n",tp,hp);
    
    return 0;
}

