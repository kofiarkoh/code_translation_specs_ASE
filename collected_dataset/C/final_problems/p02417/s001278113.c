#include<stdio.h>
#include<ctype.h>
int main(){
    char moji;
    int num, i, count[26] = {};

    for(; scanf("%c", &moji) != EOF;){
    //    scanf("%c", &moji);
  //      if( moji == '\n'){// scanf("%c", &moji) == EOF ){
//            break;
//        }
        if( islower( moji ) ){
        num = moji - 'a';
        count[ num ] += 1;

//        printf("%d", num);
        }else if( isupper( moji ) ){
            num = moji - 'A';
            count[ num ] += 1;
        }

    }
    for( i = 0; i < 26; i++ ){
        printf("%c : %d\n", i + 'a', count[i]);
    }
    return 0;
}