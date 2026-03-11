#include <stdio.h>

int main( void )
{
    int i;
    char S[3];
    
    scanf("%s", &S);
    
    if(S[0]==S[1]&&S[0]==S[2])
        printf("No");
    else
        printf("Yes");
    
    return 0;
}
