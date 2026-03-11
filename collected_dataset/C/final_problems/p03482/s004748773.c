#include <stdio.h>
#include <string.h>
#include <stdbool.h>
int min(int a,int b){return a<b?a:b;}
int max(int a,int b){return a>b?a:b;}
int main(void){
    int ans=2,count=1,prev=0;
    bool flag=false;
    char s[100020];
    scanf("%s",s);
    for (int i=1; i<strlen(s); i++){
        if (s[i]==s[i-1]){
            count++;
        } else {
            if (flag==false){
                ans=max(ans,count);
                prev=count;
                count=1;
                flag=true;
            } else {
                ans=max(ans,count+prev);
                prev=count;
                count=1;
            }
        }
    }
    printf("%d\n",min(strlen(s),max(ans,count)));
    return 0;
}