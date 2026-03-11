/* ABC141-B
   Takatan */

#include<stdio.h>
int main(void){
    char dance[101]={};
    int flag, i;
    flag = 1;
    i = 0;

    scanf( "%s" , dance);

    while(flag==1&&dance[i]!='\0'){
        flag = 0; /* 初期化 */
        if(dance[i]=='U'||dance[i]=='D'){
            flag = 1; /* 踏みやすいことを示す */
        }else if(i%2==0){ /* 奇数文字目の時 */
            if(dance[i]=='R'){
                flag = 1; /* 踏みやすいことを示す */
            }
        }else{ /* 偶数文字目の時 */
            if(dance[i]=='L'){
                flag = 1; /* 踏みやすいことを示す */
            }
        }
        i++;
    }

    if(flag==0){  /* 踏みにくい時 */
        printf( "No\n" );
    }else{ /* 踏みやすい時 */
        printf( "Yes\n" );
    }
    return 0;
}