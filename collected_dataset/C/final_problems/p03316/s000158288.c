#include<stdio.h>

int main(void){
    char s[12];
    scanf("%s",s);
    int j,i=0,sum = 0;
    double N = 0,ten = 1.0/10.0;
    while(1){
        if(s[i] == '\0'){
            break;
        }
        sum += ((int)s[i])-48;
        i++;
    }
    for(j = i-1;j>=0;j--){
        N += ((int)s[j]-48.0)*ten*10.0;
        ten = ten*10.0;
    }
    if(((int)N)%((int)sum) == 0){
        printf("Yes\n");
    }
    else{
        printf("No\n");
    }
    return 0;
}