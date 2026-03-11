#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
int min(int a,int b){return a<b?a:b;}
int main(void){
    int n,q,countc=0;
    scanf("%d",&n);
    char s[n+5];
    scanf("%s",s);
    scanf("%d",&q);
    int k[q],d[n],m[n],c[n];
    for (int i=0; i<q; i++){
        scanf("%d",&k[i]);
    }
    for (int i=0; i<n; i++){
        d[i]=0;
        m[i]=0;
        c[i]=0;
    }
    for (int i=strlen(s)-1; i>=0; i--){

        c[i]=countc;
        if(s[i]=='C'){
            countc++;
        }
    }
    for (int l=0; l<q; l++){
        long long ans=0;
        for (int i=0; i<=strlen(s); i++){
            if (s[i]=='D'){
                for (int j=1; j<k[l]; j++){
                    if (s[i+j]=='M'){
                        ans+=c[i+j]-c[min(i+k[l]-1,strlen(s)-1)];
                    }
                }
            }
        }
        printf("%lld\n",ans);
    }
    return 0;
}