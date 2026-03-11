#include <stdio.h>

int main(void) {
    int H, W;
    scanf("%d %d", &H, &W);
    char a[H][W];
    int list[26];
    for(int i=0; i<26; i++){
        list[i] = 0;
    }

    for(int i=0; i<H; i++){
        for(int j=0; j<W; j++){
            scanf("%c ", &a[i][j]);
            list[a[i][j]-'a']++;
        }
    }

    if(W==1 || H==1){
        int c=0;
        for(int i=0; i<26; i++){
            if(list[i]>=2){
                c+=(list[i]/2)*2;
                list[i]%=2;
            }    
        }
        c -= H+W-2;
        for(int i=0; i<26; i++){
            if(list[i]==1){
                c++;
            }
        }

        if(!(c-1)){
            printf("No\n");
            return 0;
        }
    } else if((H*W)%2){
        int c=0;
        for(int i=0; i<26; i++){
            if(list[i]>=4){
                c+=(list[i]/4)*4;
                list[i]%=4;
            }
        }
        c -= (H-1)*(W-1);
        for(int i=0; i<26; i++){
            if(list[i]>=2){
                c+=(list[i]/2)*2;
                list[i]%=2;
            }
        }
        c -= H+W-1;
        for(int i=0; i<26; i++){
            if(list[i]==1){
                c++;
            }
        }

        if(c-1){
            printf("No\n");
            return 0;
        }

    } else if(H%2){
        int c=0;
        for(int i=0; i<26; i++){
            if(list[i]>=4){
                c+=(list[i]/4)*4;
                list[i]%=4;
            }
        }
        c -= (H-1)*W;
        for(int i=0; i<26; i++){
            if(list[i]>=2){
                c+=(list[i]/2)*2;
                list[i]%=2;
            }
        }
        c -= H-1;
        for(int i=0; i<26; i++){
            if(list[i]==1){
                c++;
            }
        }

        if(c-1){
            printf("No\n");
            return 0;
        }
    } else if(W%2){
        int c=0;
        for(int i=0; i<26; i++){
            if(list[i]>=4){
                c+=(list[i]/4)*4;
                list[i]%=4;
            }
        }
        c -= (W-1)*H;
        for(int i=0; i<26; i++){
            if(list[i]>=2){
                c+=(list[i]/2)*2;
                list[i]%=2;
            }
        }
        c -= W-1;
        for(int i=0; i<26; i++){
            if(list[i]==1){
                c++;
            }
        }

        if(c-1){
            printf("No\n");
            return 0;
        }
    } else {
        int c=0;
        for(int i=0; i<26; i++){
            if(list[i]>=4){
                c+=(list[i]/4)*4;
                list[i]%=4;
            }
        }
        if(c-H*W){
            printf("No\n");
            return 0;
        }
    }
    printf("Yes\n");
    return 0;
}