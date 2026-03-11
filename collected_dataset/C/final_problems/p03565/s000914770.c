
#include <stdio.h>
#include <string.h>

int main(){
    char S[51], T[51];
    int i, j, Slen, Tlen, flg=0, cnt, n, k;
    
    scanf("%s", &S);
    scanf("%s", &T);

    Slen = strlen(S);
    Tlen = strlen(T);

    if(Slen < Tlen)
        flg = 1;

    for(i=0; i<Slen-Tlen+1; i++){
        cnt=0;

        for(j=0, k=i; j<Tlen; j++,k++){
            if(S[k] == T[j] || S[k] == '?'){
                cnt++;
            }
        }
        if(cnt == Tlen){
            flg = 1;
            break;
        }
    }

    if(flg == 0){
        printf("UNRESTORABLE\n");
        return 0;
    }

    for(i=k-Tlen, j=0; j<Tlen; i++,j++){
        S[i] = T[j];
    }

    for(i=0; i<Slen; i++){
        if(S[i] != '?')
            printf("%c", S[i]);
        else
            printf("a");
    }
    printf("\n");

    return 0;
}