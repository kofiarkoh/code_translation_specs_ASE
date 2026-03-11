/* ABC143-C
   Takatan */

#include<stdio.h>
int main(void){
    int number, answer, i;
    char slime[100001];
    answer = 1;

    scanf( "%d" , &number); /* 個数の入力 */
    scanf( "%s" , slime); /* 色の情報 */
    for(i=1; i < number; i++){
        if(slime[i]!=slime[i-1]){ /* 隣り合っている時 */
            answer++; /* カウント */
        }
    }
    printf( "%d\n" , answer);
    return 0;
}