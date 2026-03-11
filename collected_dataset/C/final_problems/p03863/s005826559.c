#include <stdio.h>
#include <string.h>
int main(void){
    char s[100000];
    int i,j,frg=0;
    
    scanf("%s",s);
    for(;;){
        for(i=1;i<strlen(s)-1;i++){
            if(s[i-1]!=s[i+1]){
                for(j=i;j<strlen(s)-1;j++){
                    s[j]=s[j+1];
                }
                s[strlen(s)-1]=0;
                frg=(frg+1)%2;
                break;
            }
        }
        if((strlen(s)==3&&s[0]==s[2])||strlen(s)==2||i==strlen(s)-2){
            break;
        }
    }
    if(frg==0){
        printf("Second\n");
    }else{
        printf("First\n");
    }
    return 0;
}
