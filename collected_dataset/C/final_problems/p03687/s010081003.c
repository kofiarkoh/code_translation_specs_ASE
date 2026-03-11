#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdio.h>

int main(int argc, const char * argv[]) {
    char s[101];
    scanf("%s",s);
    long l=strlen(s);
    int alp[26]={0},ans=200;
    for(int i=0;i<26;i++){
        int count=0;
        for(long j=l-1;j>=0;j--){
            if(s[j]=='a'+i){
                alp[i]=alp[i]<count?count:alp[i];
                count=0;
            }else{
                count++;
            }
            if(j==0){
                alp[i]=alp[i]<count?count:alp[i];
            }
        }
        ans=ans<alp[i]?ans:alp[i];
    }
    
    printf("%d",ans);
    return 0;
}
