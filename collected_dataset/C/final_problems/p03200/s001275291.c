#include <stdio.h>
#include <stdlib.h>

int main(){
    char S[2000000];
    int n;
    long int i,ans=0;
    int w=0;
    scanf("%s",S);
    while(S[w]=='W') w++;
    for (i = 0;S[i] != '\0';i++);
    for(n=w;n<i;n++){
        if(S[n]=='W'){
            ans+=n-w;
            w++;
        }
    }


    printf("%ld\n",ans);

    return 0;
}
