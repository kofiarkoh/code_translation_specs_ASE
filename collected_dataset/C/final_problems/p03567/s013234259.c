
#include <stdio.h>
#include <string.h>
int main()
{   char s[6];
    scanf("%s",s);
    for(int i=0;i<6;i++)if(s[i]=='A'&&s[i+1]=='C'){
        printf("Yes");
        return 0;
    }
    printf("No");
    return 0;
}
