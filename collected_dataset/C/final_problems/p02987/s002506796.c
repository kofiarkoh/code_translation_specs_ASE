#include<stdio.h>
int main(){
    char s[5];
    int count1=0,count2=0;
    scanf("%s",s);
    if(s[0]==s[1]){
        if(s[2]==s[3]){
            puts("Yes");
            return 0;
        }
    }
    else{
        if(s[0]==s[2]){
            if(s[1]==s[3]){
                puts("Yes");
                return 0;
            }
        }
        else if(s[0]==s[3]){
            if(s[1]==s[2]){
                puts("Yes");
                return 0;
            }
        }
    }
    puts("No");
    return 0;
}