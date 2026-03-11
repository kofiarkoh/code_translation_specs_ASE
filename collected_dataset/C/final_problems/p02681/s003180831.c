#include <stdio.h>
#include <string.h>

int main()
{
    char S[11], T[11];

    fgets(S, 11, stdin);
    fgets(T, 11, stdin);

    if ((strncmp(S, T, strlen(S) - 1) == 0) && (strlen(S) + 1 == strlen(T)))
    {
        puts("Yes");
    }
    else
    {
        puts("No");
    }
    return 0;
}