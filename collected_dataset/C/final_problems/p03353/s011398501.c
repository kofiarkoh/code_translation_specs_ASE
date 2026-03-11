#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(){
    char s[5000];
    int i,j,h,K;

    scanf("%s",s);
    scanf("%d",&K);

    char sub[K+1][strlen(s)+1],tmp[strlen(s)+1];
    int a=0,b,flg=1;

    for(i=0;i<K+1;i++){
        strcpy(sub[i],"~");
    }

    for(i=0;i<strlen(s);i++){
        for(j=i;j<strlen(s);j++){
            a=0;
            for(h=i;h<=j;h++){
                sub[K][a] = s[h];
                a++;
            }
            sub[K][a] = '\0';

            for(b=0;b<K;b++){
                if(strcmp(sub[b], sub[K])==0){
                    flg = 0;
                }
            }
            if(flg != 0){    
                for(a=1;a<K+1;a++){
                    for(b=1;b<K+1;b++){
                        if(strcmp(sub[b-1], sub[b])>0){
                            strcpy(tmp, sub[b-1]);
                            strcpy(sub[b-1], sub[b]);
                            strcpy(sub[b], tmp);
                        }
                    }
                }
            }
            flg=1;

        }
    }

    printf("%s\n",sub[K-1]);

    return 0;
}