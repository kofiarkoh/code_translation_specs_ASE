#include <string.h>
#include <stdio.h>
int main()
{char s[12];
int i,cnt=0,len;
    scanf("%s",s);
    len=strlen(s);
    if(s[0]=='A'){
        for(i=2;i<= len - 2;i++){
            if(s[i]=='C'){
                cnt++;
            }
        }
        if(cnt==1){
            cnt=0;
            for(i=1;i<=len;i++){
            if((s[i]>='A')&&(s[i]<='Z')){
                cnt++;
            }    
            }
            if(cnt==1){
                printf("AC");
            }else{
                printf("WA");
            }
        }else{printf("WA");}
        }else {
        printf("WA");
    }

    return 0;
}
