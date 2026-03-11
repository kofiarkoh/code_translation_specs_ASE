#include <stdio.h>
#include<string.h>
int are(int a){return a>0?a:-a;}
int main(void){
    char po[114514];
    scanf("%s",po);
    int unti=strlen(po);
    int start=0;
    int end=unti-1;
    int ans=0;
    int maestart=0;
    int maeend=unti-1;
    while(start<=end){
        if(po[start]=='x')start++;
        else{
            if(po[end]=='x')end--;
            else{
                if(po[start]!=po[end]){printf("-1");return 0;}
                else {
                    ans+=are(start-maestart-(maeend-end));
                    maestart=start;
                    maeend=end;
                    start++;
                    end--;
                }
            }
        }
    }
    printf("%d",ans);
    return 0;
}
