#include<stdio.h>
#include <string.h>

int main(void){

    int ans=0,n,k,r,s,p;
    int i=0;
    int cr=0, cs=0, cp=0; 
    char t[10001], at[10001];

    scanf("%d %d", &n, &k);
    scanf("%d %d %d", &r, &s, &p);
    scanf("%s", &t);

    while(t[i] != '\0'){

        if(t[i]==at[i-k]){
            at[i]='f';
        }else if(t[i]=='r'){
            ans+=p;
            at[i]='r';
        }else if(t[i]=='s'){
            ans+=r;
            at[i]='s';
        }else if(t[i]=='p'){
            ans+=s;
            at[i]='p';
        }
        i++;
    }

    printf("%d",ans);
}