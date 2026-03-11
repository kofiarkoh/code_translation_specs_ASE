#include <stdio.h>
#include<string.h>

int main(void){
    // 自分の得意な言語で
    // Let's チャレンジ！！

    char str[1001];
    
    

    scanf("%s",str);
    int len=strlen(str);

    if(str[len-1]=='s'){
        str[len]='e';
        str[len+1]='s';
    }else{
        str[len]='s';
    }
    
      
    printf("%s\n",str);


   
    
    return 0;
}