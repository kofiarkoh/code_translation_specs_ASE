#include <stdio.h>
#include <string.h>
int main()
{   int n;
    scanf("%d",&n);
    char s[11][n];
    for(int i=0;i<n;i++){
        scanf("%s",s[i]);
        }
        int flag=0;
    for(int i=0;i<n-1;i++){
        if(s[i][strlen(s[i])-1]!=s[i+1][0]){
            printf("No");
            return 0;
        }
        for(int j=i+1;j<n;j++){
            if(strcmp(s[i],s[j])==0){
                printf("No");
                return 0;
            }
        }
    }
    printf("Yes");
    return 0;
}