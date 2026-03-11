#include <stdio.h>
#include <string.h>
int main(void){
    char line[1025];
    char sign[4]=" ,.";
    char *ans;
    int i=0,len,sign1=0;
    fgets(line,1025,stdin);
    while(line[i++]);
    line[i-2]=0;
    ans=strtok(line,sign);
    while(ans){
        len=strlen(ans);
        if(len >= 3 && len <= 6){
            if(sign1)
                printf(" ");
            else
                sign1=1;
            printf("%s",ans);
        }
        ans=strtok(NULL,sign);
    }
    puts("");
    return 0;
}