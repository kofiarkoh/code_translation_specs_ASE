#include <stdio.h>
#include <string.h>

int main(void){
    //文字列SとTに値を代入する
    char S[200001];
    char T[200001];
    memset(&S,'\0',sizeof(S));
    memset(&T,'\0',sizeof(T));
    scanf("%s",S);
    scanf("%s",T);
    
    //文字列Sの長さをsizeに代入する
    int size = strlen(S);

    //アルファベットの箱を用意し、初期化する
    char alpha[26];
    memset(&alpha,'\0',sizeof(alpha));

    //TとSの関係を確認する
    for(int i = 0;i<size;i++){
        //アルファベットT[i]の中身が\0の場合、S[i]を代入する
        if(alpha[T[i] - 'a'] == '\0'){
            alpha[T[i] - 'a'] = S[i];
        //アルファベットT[i]の中身がS[i]でない場合、Noを出力しプログラムを終了させる
        }else if(alpha[T[i] - 'a'] != S[i]){
            printf("No\n");
            return 0;
        }
    }
    
    //重複チェックのフラグを用意する
    int flag = 0;

    //アルファベットの箱の中身に、重複したアルファベットがないかチェックする
    for(int i = 0;i<26;i++){
        for(int j = 0;j<26;j++){
            //アルファベットをa-zまで順繰りに調べていき、
            //アルファベットの箱の中身に一致したものがある場合、flagを加算する
            if(alpha[j] == 'z' - i){
                flag++;
            }
        }
        //アルファベットに2つ以上重複したものがある場合、Noを出力しプログラムを終了させる
        if(flag > 1){
            printf("No\n");
            return 0;
        }
        //重複がなかった場合フラグをリセットする
        flag = 0;
    }
    
    //途中でNoが出なかった場合Yesを出力する
    printf("Yes\n");
    return 0;
}