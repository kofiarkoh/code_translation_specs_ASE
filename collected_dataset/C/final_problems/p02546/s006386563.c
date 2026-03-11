#include<stdio.h>
#include<string.h>

int main()
{
    char s[100];
    gets(s);

    if( s[strlen(s)-1]=='s')
        {strcat(s,"es");
        printf("%s",s);
        }
    else
    {
        strcat(s,"s");
        printf("%s",s);
    }



    return 0;
}
