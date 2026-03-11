#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(){
    int N, A, B, C, D;
    int flag = 1; // Yes
    int i;
    char c;

    scanf("%d %d %d %d %d", &N, &A, &B, &C, &D);
    getchar();

    A -= 1; B-= 1; C-=1; D -= 1; // 配列の要素番号とそろえる
    int masu[N];
    for(i=0; i<N; i++){
        scanf("%c", &c);
        if(c == '.'){
            masu[i] = 0;
        }else{
            masu[i] = 1;
        }
    }

    //C,Dの前に2つ並んだ石があったら不可能
    for(i=0; i<N-1; i++){
        if(masu[i] == 1){
            if(masu[i+1] == 1 && (i <= C && i <= D)){
                flag = 0;
                break;
            }
        }
    }

    // C > D にも拘わらず　... がないとAがBを抜けない
    if(C > D){
        //printf("point\n");
        flag = 0; 
        for(i=0; (i < D-1 && i < N-2); i++){
            if(masu[i] == 0){
                if(masu[i+1] == 0 && masu[i+2] == 0){
                    //printf("%d\n", i);
                    flag = 1;
                }
            }
        }
    }

    //for(i=0; i < N; i++)printf("%d", masu[i]);
    //printf("\n");

    //上記以外はいける　（とおもう）

    if(flag == 1){
        printf("Yes\n");
    }else{
        printf("No\n");
    }
    
    return 0;
}