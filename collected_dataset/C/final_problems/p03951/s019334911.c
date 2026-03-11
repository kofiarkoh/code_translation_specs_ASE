#include <stdio.h>

int main(){
    int n;
    scanf("%d",&n);
    char s[105];
    char t[105];
    scanf("%s %s",s,t);
    int i=0;
    int j=0;
    int a=0;
    for(i=0;i<=n-1;i++){
        for(j=0;j<=n-i-1;j++){
            if(t[j]!=s[i+j]){
                a=1;
            }
        }
        if(a==0){
            break;
        }else if(a==1){
            j=0;
            a=0;
        }
    }
    
    printf("%d",n+i);
    return 0;
}
